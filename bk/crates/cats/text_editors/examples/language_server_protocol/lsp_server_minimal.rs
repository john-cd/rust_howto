// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A tiny Language Server Protocol server and client talking over an
//! in-memory channel, using `lsp-server` and `lsp-types`.
//!
//! `Connection::stdio()` would speak the same JSON-RPC messages over a
//! process's stdin/stdout with `Content-Length` framing.

use std::thread;

use lsp_server::Connection;
use lsp_server::Message;
use lsp_server::Request;
use lsp_server::RequestId;
use lsp_server::Response;
use lsp_types::Hover;
use lsp_types::HoverContents;
use lsp_types::HoverParams;
use lsp_types::HoverProviderCapability;
use lsp_types::MarkedString;
use lsp_types::Position;
use lsp_types::ServerCapabilities;
use lsp_types::TextDocumentIdentifier;
use lsp_types::TextDocumentPositionParams;
use lsp_types::Uri;
use lsp_types::request::HoverRequest;
use lsp_types::request::Request as _;

/// The server: answers `initialize` and `textDocument/hover`.
fn run_server(connection: Connection) -> anyhow::Result<()> {
    // Handshake: advertise what the server supports.
    let capabilities = ServerCapabilities {
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    };
    connection.initialize(serde_json::to_value(capabilities)?)?;

    for message in &connection.receiver {
        match message {
            Message::Request(request) => {
                // Replies to `shutdown`; returns true once `exit` follows.
                if connection.handle_shutdown(&request)? {
                    break;
                }
                let response = match request.method.as_str() {
                    HoverRequest::METHOD => {
                        let hover = Hover {
                            contents: HoverContents::Scalar(
                                MarkedString::String(
                                    "Hello from the server".into(),
                                ),
                            ),
                            range: None,
                        };
                        Response::new_ok(request.id, hover)
                    }
                    _ => Response::new_err(
                        request.id,
                        lsp_server::ErrorCode::MethodNotFound as i32,
                        format!("unsupported method {}", request.method),
                    ),
                };
                connection.sender.send(Message::Response(response))?;
            }
            Message::Notification(_) | Message::Response(_) => {}
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    // Two connected ends: the first for the server, the second for us.
    let (server_side, client) = Connection::memory();
    let server = thread::spawn(move || run_server(server_side));

    // 1. `initialize`, then the `initialized` notification.
    let init = Request::new(
        RequestId::from(1),
        "initialize".into(),
        serde_json::json!({ "processId": null, "rootUri": null, "capabilities": {} }),
    );
    client.sender.send(init.into())?;
    let Message::Response(init_response) = client.receiver.recv()? else {
        anyhow::bail!("expected the initialize response");
    };
    assert_eq!(init_response.id, RequestId::from(1));
    client.sender.send(
        lsp_server::Notification::new(
            "initialized".into(),
            serde_json::json!({}),
        )
        .into(),
    )?;

    // 2. A hover request. Responses are matched to requests by id.
    let uri: Uri = "file:///project/main.rs".parse()?;
    let params = HoverParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position: Position::new(0, 3),
        },
        work_done_progress_params: Default::default(),
    };
    client.sender.send(
        Request::new(RequestId::from(2), HoverRequest::METHOD.into(), params)
            .into(),
    )?;
    let Message::Response(hover_response) = client.receiver.recv()? else {
        anyhow::bail!("expected the hover response");
    };
    assert_eq!(hover_response.id, RequestId::from(2));
    let result = hover_response
        .response_result
        .map_err(|e| anyhow::anyhow!("server error: {}", e.message))?;
    let hover: Hover = serde_json::from_value(result)?;
    println!("hover: {hover:?}");
    assert!(matches!(hover.contents, HoverContents::Scalar(_)));

    // 3. Orderly shutdown: `shutdown` request, then `exit` notification.
    client
        .sender
        .send(Request::new(RequestId::from(3), "shutdown".into(), ()).into())?;
    let Message::Response(_) = client.receiver.recv()? else {
        anyhow::bail!("expected the shutdown response");
    };
    client
        .sender
        .send(lsp_server::Notification::new("exit".into(), ()).into())?;

    server
        .join()
        .map_err(|_| anyhow::anyhow!("server thread panicked"))??;
    Ok(())
}
// ANCHOR_END: example

#[allow(clippy::main_recursion)]
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the lsp server minimal example through its entry point.
    #[test]
    fn test_lsp_server_minimal_example_entry_point() -> anyhow::Result<()> {
        main()
    }
}
