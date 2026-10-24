#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Receives from whichever Crossbeam channel is ready first.

use anyhow::Result;
use crossbeam_channel::bounded;
use crossbeam_channel::select;

/// Receives one message from each of two bounded channels.
fn selected_messages() -> Result<Vec<String>> {
    let (left_tx, left_rx) = bounded(1);
    let (right_tx, right_rx) = bounded(1);
    left_tx.send(String::from("left"))?;
    right_tx.send(String::from("right"))?;

    let mut messages = Vec::new();
    for _ in 0..2 {
        select! {
            recv(left_rx) -> message => messages.push(message?),
            recv(right_rx) -> message => messages.push(message?),
        }
    }

    messages.sort();
    Ok(messages)
}

/// Runs the channel selection example and checks both messages.
fn main() -> Result<()> {
    assert_eq!(selected_messages()?, vec!["left", "right"]);
    Ok(())
}

// ANCHOR_END: example

/// Runs the channel selection example.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `select!` receives all messages without relying on order.
    #[test]
    fn select_receives_from_several_channels() -> Result<()> {
        assert_eq!(selected_messages()?, vec!["left", "right"]);
        Ok(())
    }
}
