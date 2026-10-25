// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Keeps business rules as data: a decision table in the JSON Decision Model
//! (JDM) format, evaluated with `zen-engine`.

use serde_json::json;
use zen_engine::DecisionEngine;
use zen_engine::model::DecisionContent;

/// Normally edited in a visual editor and loaded from a file or a database.
/// Rows are tried top to bottom; the `first` hit policy keeps the first match.
const RULES: &str = r#"{
  "nodes": [
    { "id": "in", "name": "request", "type": "inputNode" },
    {
      "id": "table", "name": "placement", "type": "decisionTableNode",
      "content": {
        "hitPolicy": "first",
        "inputs": [
          { "id": "i1", "name": "Account type", "field": "accountType" },
          { "id": "i2", "name": "Asset class", "field": "assetClass" }
        ],
        "outputs": [
          { "id": "o1", "name": "Preferred account", "field": "preferred" }
        ],
        "rules": [
          { "_id": "r1", "i1": "'tfsa'", "i2": "'bond'", "o1": "'rrsp'" },
          { "_id": "r2", "i1": "", "i2": "'bond'", "o1": "'taxable'" },
          { "_id": "r3", "i1": "'tfsa'", "i2": "", "o1": "'tfsa'" }
        ]
      }
    },
    { "id": "out", "name": "response", "type": "outputNode" }
  ],
  "edges": [
    { "id": "e1", "sourceId": "in", "targetId": "table" },
    { "id": "e2", "sourceId": "table", "targetId": "out" }
  ]
}"#;

fn main() -> anyhow::Result<()> {
    let content: DecisionContent = serde_json::from_str(RULES)?;
    let engine = DecisionEngine::default();
    let decision = engine.create_decision(content.into())?;

    // Evaluation is async.
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    // Convert the result back to plain JSON.
    let evaluate =
        |context: serde_json::Value| -> anyhow::Result<serde_json::Value> {
            let response = runtime
                .block_on(decision.evaluate(context.into()))
                .map_err(|e| anyhow::anyhow!("{e}"))?;
            Ok(response.result.to_value())
        };

    // Rule `r1` matches first.
    let result =
        evaluate(json!({ "accountType": "tfsa", "assetClass": "bond" }))?;
    println!("{result}");
    assert_eq!(result["preferred"], "rrsp");

    // An empty cell matches anything, so `r2` applies to every account type.
    let result =
        evaluate(json!({ "accountType": "margin", "assetClass": "bond" }))?;
    assert_eq!(result["preferred"], "taxable");

    // No rule matches: the table produces no output rather than an error.
    let result =
        evaluate(json!({ "accountType": "margin", "assetClass": "stock" }))?;
    println!("no match: {result}");
    assert!(result.get("preferred").is_none());
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
#[allow(clippy::main_recursion)]
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
}
