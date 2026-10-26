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

/// Evaluates one context and rejects missing inputs or an unmatched rule.
///
/// # Errors
///
/// Returns an error when a required input is absent, engine evaluation fails,
/// or the decision table produces no preferred account.
fn evaluate(context: serde_json::Value) -> anyhow::Result<serde_json::Value> {
    for field in ["accountType", "assetClass"] {
        if context.get(field).is_none() {
            anyhow::bail!("missing required input field `{field}`");
        }
    }

    let content: DecisionContent = serde_json::from_str(RULES)?;
    let decision = DecisionEngine::default().create_decision(content.into())?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let response = runtime
        .block_on(decision.evaluate(context.into()))
        .map_err(|error| {
            anyhow::anyhow!("decision evaluation failed: {error}")
        })?;
    let result = response.result.to_value();
    if result.get("preferred").is_none() {
        anyhow::bail!("no decision-table rule matched the supplied inputs");
    }

    Ok(result)
}

/// Demonstrates successful matches and clear errors for invalid inputs.
fn main() -> anyhow::Result<()> {
    // Rule `r1` matches first.
    let result =
        evaluate(json!({ "accountType": "tfsa", "assetClass": "bond" }))?;
    println!("{result}");
    assert_eq!(result["preferred"], "rrsp");

    // An empty cell matches anything, so `r2` applies to every account type.
    let result =
        evaluate(json!({ "accountType": "margin", "assetClass": "bond" }))?;
    assert_eq!(result["preferred"], "taxable");

    // Missing required inputs are rejected before evaluation.
    let missing_input_error = evaluate(json!({ "assetClass": "bond" }))
        .expect_err("accountType is required");
    assert!(
        missing_input_error
            .to_string()
            .contains("missing required input field `accountType`")
    );
    println!("missing field: {missing_input_error}");

    // No matching row becomes an explicit domain error.
    let no_match_error = evaluate(json!({
        "accountType": "margin",
        "assetClass": "stock",
    }))
    .expect_err("a missing preferred account should be rejected");
    assert_eq!(
        no_match_error.to_string(),
        "no decision-table rule matched the supplied inputs"
    );
    println!("no match: {no_match_error}");
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

    /// Verifies that an absent account type produces an actionable error.
    #[test]
    fn missing_account_type_is_rejected() {
        let error = evaluate(json!({ "assetClass": "bond" }))
            .expect_err("accountType is required");

        assert!(
            error
                .to_string()
                .contains("missing required input field `accountType`")
        );
    }

    /// Verifies that no matching row produces a decision-table error.
    #[test]
    fn no_matching_rule_is_reported() {
        let error = evaluate(json!({
            "accountType": "margin",
            "assetClass": "stock",
        }))
        .expect_err("the decision table has no matching row");

        assert_eq!(
            error.to_string(),
            "no decision-table rule matched the supplied inputs"
        );
    }

    /// Verifies the example's successful decision-table scenarios.
    #[test]
    fn main_succeeds_with_valid_inputs() -> anyhow::Result<()> {
        main()
    }
}
