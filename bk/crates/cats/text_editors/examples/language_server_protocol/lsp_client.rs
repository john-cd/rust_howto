// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Opens a document and asks for its symbols with `lsp-server` and
//! `lsp-types`, and shows the `Content-Length` framing that real servers use.

use std::io::BufReader;
use std::thread;

use lsp_server::Connection;
use lsp_server::Message;
use lsp_server::Notification;
use lsp_server::Request;
use lsp_server::RequestId;
use lsp_server::Response;
use lsp_types::DidOpenTextDocumentParams;
use lsp_types::DocumentSymbolParams;
use lsp_types::DocumentSymbolResponse;
use lsp_types::Location;
use lsp_types::OneOf;
use lsp_types::Position;
use lsp_types::Range;
use lsp_types::ServerCapabilities;
use lsp_types::SymbolInformation;
use lsp_types::SymbolKind;
use lsp_types::TextDocumentIdentifier;
use lsp_types::TextDocumentItem;
use lsp_types::Uri;
use lsp_types::notification::DidOpenTextDocument;
use lsp_types::notification::Notification as _;
use lsp_types::request::DocumentSymbolRequest;
use lsp_types::request::Request as _;

/// A server that remembers opened documents and lists their `fn` names.
fn run_server(connection: Connection) -> anyhow::Result<()> {
    let capabilities = ServerCapabilities {
        document_symbol_provider: Some(OneOf::Left(true)),
        ..Default::default()
    };
    connection.initialize(serde_json::to_value(capabilities)?)?;

    let mut documents: Vec<(Uri, String)> = Vec::new();
    for message in &connection.receiver {
        match message {
            Message::Notification(n)
                if n.method == DidOpenTextDocument::METHOD =>
            {
                let params: DidOpenTextDocumentParams =
                    serde_json::from_value(n.params)?;
                documents.push((
                    params.text_document.uri,
                    params.text_document.text,
                ));
            }
            Message::Request(request) => {
                if connection.handle_shutdown(&request)? {
                    break;
                }
                let params: DocumentSymbolParams =
                    serde_json::from_value(request.params)?;
                let uri = params.text_document.uri;
                let text = documents
                    .iter()
                    .find(|(u, _)| *u == uri)
                    .map(|(_, text)| text.as_str())
                    .unwrap_or_default();
                let symbols = function_symbols(&uri, text);
                let response = Response::new_ok(
                    request.id,
                    DocumentSymbolResponse::Flat(symbols),
                );
                connection.sender.send(Message::Response(response))?;
            }
            _ => {}
        }
    }
    Ok(())
}

/// Finds `fn name` at the start of a line (a stand-in for a real parser).
fn function_symbols(uri: &Uri, text: &str) -> Vec<SymbolInformation> {
    let mut symbols = Vec::new();
    for (line, content) in text.lines().enumerate() {
        if let Some(rest) = content.strip_prefix("fn ") {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            let line = u32::try_from(line).unwrap_or(u32::MAX);
            let range = Range::new(
                Position::new(line, 0),
                Position::new(line, u32::try_from(content.len()).unwrap_or(0)),
            );
            #[allow(deprecated)] // `deprecated` is a required field.
            symbols.push(SymbolInformation {
                name,
                kind: SymbolKind::FUNCTION,
                tags: None,
                deprecated: None,
                location: Location::new(uri.clone(), range),
                container_name: None,
            });
        }
    }
    symbols
}

fn main() -> anyhow::Result<()> {
    let (server_side, client) = Connection::memory();
    let server = thread::spawn(move || run_server(server_side));

    client.sender.send(
        Request::new(
            RequestId::from(1),
            "initialize".into(),
            serde_json::json!({ "processId": null, "rootUri": null, "capabilities": {} }),
        )
        .into(),
    )?;
    client.receiver.recv()?;
    client
        .sender
        .send(Notification::new("initialized".into(), ()).into())?;

    // `didOpen` sends the whole text; the server answers no response.
    let uri: Uri = "file:///project/main.rs".parse()?;
    let open = DidOpenTextDocumentParams {
        text_document: TextDocumentItem::new(
            uri.clone(),
            "rust".into(),
            1,
            "fn main() {}\n\nfn helper() {}\n".into(),
        ),
    };
    client.sender.send(
        Notification::new(DidOpenTextDocument::METHOD.into(), open).into(),
    )?;

    // `textDocument/documentSymbol`.
    let params = DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    };
    client.sender.send(
        Request::new(
            RequestId::from(2),
            DocumentSymbolRequest::METHOD.into(),
            params,
        )
        .into(),
    )?;
    let Message::Response(response) = client.receiver.recv()? else {
        anyhow::bail!("expected the documentSymbol response");
    };
    let result = response
        .response_result
        .map_err(|e| anyhow::anyhow!("server error: {}", e.message))?;
    let DocumentSymbolResponse::Flat(symbols) = serde_json::from_value(result)?
    else {
        anyhow::bail!("expected flat symbols");
    };
    let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
    println!("symbols: {names:?}");
    assert_eq!(names, ["main", "helper"]);

    client
        .sender
        .send(Request::new(RequestId::from(3), "shutdown".into(), ()).into())?;
    client.receiver.recv()?;
    client
        .sender
        .send(Notification::new("exit".into(), ()).into())?;
    server
        .join()
        .map_err(|_| anyhow::anyhow!("server thread panicked"))??;

    // Over stdio every message is framed as `Content-Length: N\r\n\r\n{json}`.
    let mut wire = Vec::new();
    Message::Notification(Notification::new("initialized".into(), ()))
        .write(&mut wire)?;
    println!("{}", String::from_utf8_lossy(&wire));
    assert!(wire.starts_with(b"Content-Length: "));
    let decoded = Message::read(&mut BufReader::new(wire.as_slice()))?;
    assert!(matches!(decoded, Some(Message::Notification(_))));
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    // Call through a binding: calling `main` directly trips `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the lsp client example through its entry point.
    #[test]
    fn test_lsp_client_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Drives a real `rust-analyzer` over stdio. Run it with
    /// `cargo test -- --ignored` when `rust-analyzer` is on the `PATH`.
    #[test]
    #[ignore = "needs rust-analyzer on the PATH"]
    fn test_real_rust_analyzer() -> anyhow::Result<()> {
        use std::io::BufWriter;
        use std::process::Command;
        use std::process::Stdio;

        let mut child = Command::new("rust-analyzer")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;
        let mut writer = BufWriter::new(
            child
                .stdin
                .take()
                .ok_or_else(|| anyhow::anyhow!("no stdin"))?,
        );
        let mut reader = BufReader::new(
            child
                .stdout
                .take()
                .ok_or_else(|| anyhow::anyhow!("no stdout"))?,
        );

        let initialize = Request::new(
            RequestId::from(1),
            "initialize".into(),
            serde_json::json!({ "processId": null, "rootUri": null, "capabilities": {} }),
        );
        Message::Request(initialize).write(&mut writer)?;
        // Skip server-initiated messages until the response arrives.
        let capabilities = loop {
            match Message::read(&mut reader)? {
                Some(Message::Response(r)) if r.id == RequestId::from(1) => {
                    break r.response_result.map_err(|e| {
                        anyhow::anyhow!("server error: {}", e.message)
                    })?;
                }
                Some(_) => {}
                None => anyhow::bail!("server closed the connection"),
            }
        };
        assert!(capabilities["capabilities"].is_object());

        Message::Notification(Notification::new("initialized".into(), ()))
            .write(&mut writer)?;
        let uri: Uri = "file:///tmp/rust_howto_lsp_client.rs".parse()?;
        Message::Notification(Notification::new(
            DidOpenTextDocument::METHOD.into(),
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem::new(
                    uri.clone(),
                    "rust".into(),
                    1,
                    "fn rust_analyzer_client_probe() {}\n".into(),
                ),
            },
        ))
        .write(&mut writer)?;
        Message::Request(Request::new(
            RequestId::from(2),
            DocumentSymbolRequest::METHOD.into(),
            serde_json::json!({
                "textDocument": { "uri": uri.to_string() }
            }),
        ))
        .write(&mut writer)?;
        let symbols = loop {
            match Message::read(&mut reader)? {
                Some(Message::Response(response))
                    if response.id == RequestId::from(2) =>
                {
                    break response.response_result.map_err(|error| {
                        anyhow::anyhow!("server error: {}", error.message)
                    })?;
                }
                Some(Message::Request(request)) => {
                    // rust-analyzer may request optional client settings while
                    // starting. This example has no settings, so answer with
                    // the protocol's empty/default values and keep reading.
                    let result = match request.method.as_str() {
                        "workspace/configuration" => serde_json::json!([]),
                        _ => serde_json::Value::Null,
                    };
                    Message::Response(Response::new_ok(request.id, result))
                        .write(&mut writer)?;
                }
                Some(_) => {}
                None => anyhow::bail!("server closed the connection"),
            }
        };
        assert!(symbols.is_array());
        assert!(symbols.as_array().is_some_and(|items| {
            items.iter().any(|item| {
                item.get("name").and_then(serde_json::Value::as_str)
                    == Some("rust_analyzer_client_probe")
            })
        }));

        Message::Request(Request::new(
            RequestId::from(3),
            "shutdown".into(),
            (),
        ))
        .write(&mut writer)?;
        loop {
            match Message::read(&mut reader)? {
                Some(Message::Response(r)) if r.id == RequestId::from(3) => {
                    break;
                }
                Some(_) => {}
                None => anyhow::bail!("server closed the connection"),
            }
        }
        Message::Notification(Notification::new("exit".into(), ()))
            .write(&mut writer)?;
        drop(writer);
        assert!(child.wait()?.success());
        Ok(())
    }
}
