//! HTTP JSON-RPC transport for MCP
//!
//! Provides bidirectional request-response communication using HTTP POST requests
//! with JSON-RPC payloads.

use crate::{McpRequest, McpResponse};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use tracing::{debug, error, trace};

/// HTTP transport for MCP using JSON-RPC over HTTP POST
#[derive(Clone)]
pub struct HttpTransport {
    /// Callback for handling incoming requests
    handler: Arc<dyn Fn(McpRequest) -> McpResponse + Send + Sync>,
}

impl HttpTransport {
    /// Create a new HTTP transport with a request handler
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(McpRequest) -> McpResponse + Send + Sync + 'static,
    {
        Self {
            handler: Arc::new(handler),
        }
    }

    /// Handle an incoming MCP request
    pub fn handle_request(&self, request: McpRequest) -> McpResponse {
        trace!("Handling HTTP request: method={}", request.method);
        (self.handler)(request)
    }
}

/// Axum handler for HTTP JSON-RPC requests
pub async fn http_handler(
    State(transport): State<Arc<HttpTransport>>,
    Json(request): Json<McpRequest>,
) -> Response {
    debug!(
        "Received HTTP JSON-RPC request: method={}, id={:?}",
        request.method, request.id
    );

    let response = transport.handle_request(request);

    debug!(
        "Sending HTTP JSON-RPC response: id={:?}, error={:?}",
        response.id,
        response.error.as_ref().map(|e| &e.message)
    );

    Json(response).into_response()
}

/// Batch request handler for multiple JSON-RPC requests
pub async fn http_batch_handler(
    State(transport): State<Arc<HttpTransport>>,
    Json(requests): Json<Vec<McpRequest>>,
) -> Response {
    debug!("Received HTTP JSON-RPC batch request: {} items", requests.len());

    if requests.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(McpResponse::error(
                None,
                -32600,
                "Invalid Request: empty batch",
            )),
        )
            .into_response();
    }

    let responses: Vec<McpResponse> = requests
        .into_iter()
        .map(|req| transport.handle_request(req))
        .collect();

    debug!("Sending HTTP JSON-RPC batch response: {} items", responses.len());

    Json(responses).into_response()
}

/// Error response for invalid JSON
pub fn invalid_json_error() -> Response {
    error!("Invalid JSON in request");
    (
        StatusCode::BAD_REQUEST,
        Json(McpResponse::error(None, -32700, "Parse error")),
    )
        .into_response()
}

/// Error response for internal errors
pub fn internal_error(message: impl Into<String>) -> Response {
    let msg = message.into();
    error!("Internal error: {}", msg);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(McpResponse::error(None, -32603, msg)),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::McpRequest;

    #[test]
    fn test_http_transport_handler() {
        let transport = HttpTransport::new(|req| {
            McpResponse::success(req.id, serde_json::json!({"method": req.method}))
        });

        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(1),
            method: "test_method".to_string(),
            params: None,
        };

        let response = transport.handle_request(request);

        assert_eq!(response.id, Some(1));
        assert!(response.error.is_none());
        assert!(response.result.is_some());

        let result = response.result.unwrap();
        assert_eq!(result["method"], "test_method");
    }

    #[test]
    fn test_http_transport_error_response() {
        let transport = HttpTransport::new(|req| {
            McpResponse::error(req.id, -32601, "Method not found")
        });

        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(1),
            method: "unknown_method".to_string(),
            params: None,
        };

        let response = transport.handle_request(request);

        assert_eq!(response.id, Some(1));
        assert!(response.result.is_none());
        assert!(response.error.is_some());

        let error = response.error.unwrap();
        assert_eq!(error.code, -32601);
        assert_eq!(error.message, "Method not found");
    }
}
