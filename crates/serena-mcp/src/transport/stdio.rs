use crate::protocol::{McpRequest, McpResponse};
use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tracing::{debug, trace};

pub struct StdioTransport {
    stdin: tokio::sync::Mutex<BufReader<tokio::io::Stdin>>,
    stdout: tokio::sync::Mutex<tokio::io::Stdout>,
}

impl StdioTransport {
    pub fn new() -> Self {
        Self {
            stdin: tokio::sync::Mutex::new(BufReader::new(tokio::io::stdin())),
            stdout: tokio::sync::Mutex::new(tokio::io::stdout()),
        }
    }

    pub async fn receive(&self) -> Result<Option<McpRequest>> {
        let mut stdin = self.stdin.lock().await;

        // Read headers (LSP-style with Content-Length)
        let mut content_length: Option<usize> = None;
        let mut line = String::new();

        loop {
            line.clear();
            let bytes_read = stdin
                .read_line(&mut line)
                .await
                .context("Failed to read header line")?;

            if bytes_read == 0 {
                // EOF
                return Ok(None);
            }

            let trimmed = line.trim();

            if trimmed.is_empty() {
                // Empty line indicates end of headers
                break;
            }

            if let Some(length_str) = trimmed.strip_prefix("Content-Length:") {
                content_length = Some(
                    length_str
                        .trim()
                        .parse()
                        .context("Invalid Content-Length header")?,
                );
            }
        }

        let content_length = content_length.context("Missing Content-Length header")?;

        trace!("Reading message with Content-Length: {}", content_length);

        // Read the JSON content
        let mut buffer = vec![0u8; content_length];
        stdin
            .read_exact(&mut buffer)
            .await
            .context("Failed to read message body")?;

        let content = String::from_utf8(buffer).context("Message body is not valid UTF-8")?;

        trace!("Received message: {}", content);

        let request: McpRequest =
            serde_json::from_str(&content).context("Failed to parse JSON-RPC request")?;

        debug!("Parsed request: method={}", request.method);

        Ok(Some(request))
    }

    pub async fn send(&self, response: &McpResponse) -> Result<()> {
        let json = serde_json::to_string(response).context("Failed to serialize response")?;

        trace!("Sending response: {}", json);

        let content_length = json.len();
        let message = format!("Content-Length: {}\r\n\r\n{}", content_length, json);

        let mut stdout = self.stdout.lock().await;
        stdout
            .write_all(message.as_bytes())
            .await
            .context("Failed to write response")?;
        stdout.flush().await.context("Failed to flush stdout")?;

        debug!("Sent response: id={:?}", response.id);

        Ok(())
    }
}

impl Default for StdioTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::McpRequest;

    #[tokio::test]
    async fn test_parse_request() {
        let json = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let request: McpRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.id, Some(1));
        assert_eq!(request.method, "initialize");
    }

    #[tokio::test]
    async fn test_serialize_response() {
        let response = McpResponse::success(Some(1), serde_json::json!({"status": "ok"}));
        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"result\""));
    }
}
