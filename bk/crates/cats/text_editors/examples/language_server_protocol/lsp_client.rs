// TODO Reviewed ??-??-2026

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

#[allow(clippy::main_recursion)]
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the lsp client example through its entry point.
    #[test]
    fn test_lsp_client_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    #[test]
    fn test_function_symbols() -> anyhow::Result<()> {
        let uri: Uri = "file:///test.rs".parse()?;
        let symbols = function_symbols(&uri, "fn foo()\nlet x = 1;\nfn bar_1()");
        let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["foo", "bar_1"]);

        let empty = function_symbols(&uri, "no functions here");
        assert!(empty.is_empty());
        Ok(())
    }
}
