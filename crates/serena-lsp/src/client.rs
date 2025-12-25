//! Generic LSP client implementation
//!
//! Provides a robust LSP client that handles JSON-RPC communication with language servers
//! over stdio. Based on the existing serena_core LSP client implementation.

use anyhow::{Context, Result};
use dashmap::DashMap;
use lsp_types::{
    notification::Notification, request::Request, ClientCapabilities, DocumentSymbolClientCapabilities,
    InitializeParams, InitializeResult, InitializedParams, TextDocumentClientCapabilities, TraceValue,
    Uri,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    process::Stdio,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{Child, Command},
    sync::{mpsc, oneshot},
    task::JoinHandle,
};
use tracing::{debug, error, warn};

/// JSON-RPC request message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Value,
    id: i64,
}

/// JSON-RPC notification message (no response expected)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRpcNotification {
    jsonrpc: String,
    method: String,
    params: Value,
}

/// JSON-RPC response message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<i64>,
    result: Option<Value>,
    error: Option<JsonRpcError>,
}

/// JSON-RPC error object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    pub data: Option<Value>,
}

/// LSP client that communicates with a language server via stdio
pub struct LspClient {
    child: Option<Child>,
    request_id: AtomicI64,
    sender: mpsc::Sender<String>,
    pending_requests: Arc<DashMap<i64, oneshot::Sender<Result<Value>>>>,
    _listener_task: JoinHandle<()>,
}

impl Drop for LspClient {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.start_kill();
        }
    }
}

impl LspClient {
    /// Create a new LSP client by spawning a language server process
    ///
    /// # Arguments
    /// * `command` - The command to execute (e.g., "rust-analyzer")
    /// * `args` - Command-line arguments for the language server
    ///
    /// # Returns
    /// A new LSP client instance that's ready to communicate
    pub async fn new(command: String, args: Vec<String>) -> Result<Self> {
        let mut child = Command::new(&command)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn language server")?;

        let stdin = child.stdin.take().context("Failed to open stdin")?;
        let stdout = child.stdout.take().context("Failed to open stdout")?;
        let stderr = child.stderr.take().context("Failed to open stderr")?;

        // Channel to send raw strings to stdin writer task
        let (tx, mut rx) = mpsc::channel::<String>(32);

        // Writer Task - sends messages to language server stdin
        let mut writer_stdin = stdin;
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                let content_length = msg.len();
                let full_msg = format!("Content-Length: {}\r\n\r\n{}", content_length, msg);
                if let Err(e) = writer_stdin.write_all(full_msg.as_bytes()).await {
                    error!("Failed to write to LSP stdin: {}", e);
                    break;
                }
                if let Err(e) = writer_stdin.flush().await {
                    error!("Failed to flush LSP stdin: {}", e);
                    break;
                }
            }
        });

        // Stderr Logger Task - logs language server stderr output
        let reader_stderr = BufReader::new(stderr);
        tokio::spawn(async move {
            let mut lines = reader_stderr.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                warn!("[LSP STDERR] {}", line);
            }
        });

        let pending_requests: Arc<DashMap<i64, oneshot::Sender<Result<Value>>>> = Arc::new(DashMap::new());
        let pending_requests_clone = pending_requests.clone();

        // Reader Task - receives messages from language server stdout
        let listener_task = tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            loop {
                // Read headers
                let mut content_length = 0;
                loop {
                    let mut line = String::new();
                    match reader.read_line(&mut line).await {
                        Ok(0) => return, // EOF
                        Ok(_) => {
                            if line.trim().is_empty() {
                                // End of headers
                                break;
                            }
                            if line.starts_with("Content-Length: ") {
                                if let Ok(len) = line.trim()["Content-Length: ".len()..].parse::<usize>() {
                                    content_length = len;
                                }
                            }
                        }
                        Err(e) => {
                            error!("Error reading LSP header: {}", e);
                            return;
                        }
                    }
                }

                if content_length == 0 {
                    continue;
                }

                // Read body
                let mut body_buf = vec![0u8; content_length];
                if let Err(e) = reader.read_exact(&mut body_buf).await {
                    error!("Error reading LSP body: {}", e);
                    return;
                }

                let body_str = String::from_utf8_lossy(&body_buf);
                debug!("Received LSP message: {}", body_str);

                // Parse JSON
                if let Ok(resp) = serde_json::from_str::<JsonRpcResponse>(&body_str) {
                    if let Some(id) = resp.id {
                        if let Some((_, tx)) = pending_requests_clone.remove(&id) {
                            if let Some(error) = resp.error {
                                let _ = tx.send(Err(anyhow::anyhow!("LSP Error {}: {}", error.code, error.message)));
                            } else {
                                let _ = tx.send(Ok(resp.result.unwrap_or(Value::Null)));
                            }
                        }
                    } else {
                        // Server-to-client notification or request
                        debug!("Received server notification/request");
                    }
                } else if let Ok(notif) = serde_json::from_str::<JsonRpcNotification>(&body_str) {
                    debug!("Received notification: {}", notif.method);
                    // TODO: Handle specific notifications (publishDiagnostics, etc.)
                }
            }
        });

        Ok(Self {
            child: Some(child),
            request_id: AtomicI64::new(1),
            sender: tx,
            pending_requests,
            _listener_task: listener_task,
        })
    }

    /// Send a typed LSP request and wait for the response
    ///
    /// # Type Parameters
    /// * `R` - The LSP request type (e.g., `GotoDefinition`, `DocumentSymbol`)
    ///
    /// # Arguments
    /// * `params` - Request parameters
    ///
    /// # Returns
    /// The typed response for the request
    pub async fn send_request<R>(&self, params: R::Params) -> Result<R::Result>
    where
        R: Request,
    {
        let id = self.request_id.fetch_add(1, Ordering::SeqCst);
        let params_json = serde_json::to_value(params)?;

        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: R::METHOD.to_string(),
            params: params_json,
            id,
        };

        let req_str = serde_json::to_string(&req)?;

        let (tx, rx) = oneshot::channel();
        self.pending_requests.insert(id, tx);

        self.sender
            .send(req_str)
            .await
            .context("Failed to send request")?;

        let response_value = rx.await.context("Sender dropped before response")??;

        let result = serde_json::from_value(response_value)?;
        Ok(result)
    }

    /// Send a typed LSP notification (no response expected)
    ///
    /// # Type Parameters
    /// * `N` - The LSP notification type (e.g., `DidOpenTextDocument`)
    ///
    /// # Arguments
    /// * `params` - Notification parameters
    pub async fn send_notification<N>(&self, params: N::Params) -> Result<()>
    where
        N: Notification,
    {
        let params_json = serde_json::to_value(params)?;
        let notif = JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: N::METHOD.to_string(),
            params: params_json,
        };
        let notif_str = serde_json::to_string(&notif)?;
        self.sender
            .send(notif_str)
            .await
            .context("Failed to send notification")?;
        Ok(())
    }

    /// Initialize the language server
    ///
    /// This must be called before sending any other requests. It sends the
    /// `initialize` request followed by the `initialized` notification.
    ///
    /// # Arguments
    /// * `root_uri` - The root URI of the workspace
    ///
    /// # Returns
    /// The server's initialization result containing capabilities
    pub async fn initialize(&self, root_uri: Uri) -> Result<InitializeResult> {
        let capabilities = ClientCapabilities {
            text_document: Some(TextDocumentClientCapabilities {
                document_symbol: Some(DocumentSymbolClientCapabilities {
                    hierarchical_document_symbol_support: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        let params = InitializeParams {
            process_id: Some(std::process::id()),
            root_uri: Some(root_uri.clone()),
            capabilities,
            trace: Some(TraceValue::Off),
            ..Default::default()
        };

        let result = self.send_request::<lsp_types::request::Initialize>(params).await?;

        self.send_notification::<lsp_types::notification::Initialized>(InitializedParams {})
            .await?;

        Ok(result)
    }

    /// Shutdown the language server gracefully
    ///
    /// Sends the `shutdown` request followed by the `exit` notification
    pub async fn shutdown(&mut self) -> Result<()> {
        // Send shutdown request
        let _: () = self.send_request::<lsp_types::request::Shutdown>(()).await?;

        // Send exit notification
        self.send_notification::<lsp_types::notification::Exit>(()).await?;

        // Wait a bit for graceful shutdown
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Kill the process if still running
        if let Some(mut child) = self.child.take() {
            let _ = child.kill().await;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsonrpc_request_serialization() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "initialize".to_string(),
            params: serde_json::json!({}),
            id: 1,
        };

        let serialized = serde_json::to_string(&req).unwrap();
        assert!(serialized.contains("\"method\":\"initialize\""));
        assert!(serialized.contains("\"id\":1"));
    }

    #[test]
    fn test_jsonrpc_response_deserialization() {
        // Test with non-null result
        let json = r#"{"jsonrpc":"2.0","id":1,"result":{"success":true}}"#;
        let resp: JsonRpcResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.id, Some(1));
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());

        // Test with null result
        let json_null = r#"{"jsonrpc":"2.0","id":2,"result":null}"#;
        let resp_null: JsonRpcResponse = serde_json::from_str(json_null).unwrap();
        assert_eq!(resp_null.id, Some(2));
        // Note: serde deserializes null as None for Option<Value>
        assert!(resp_null.result.is_none() || resp_null.result == Some(Value::Null));
        assert!(resp_null.error.is_none());
    }
}
