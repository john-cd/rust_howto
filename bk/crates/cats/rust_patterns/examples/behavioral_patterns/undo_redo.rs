#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Implements an undo and redo history with reversible insert commands.

/// A reversible insertion at a character index.
#[derive(Clone, Copy, Debug)]
struct InsertChar {
    /// Character position at which to insert the value.
    index: usize,
    /// Character to insert.
    value: char,
}

/// Text plus command stacks that support undoing and redoing edits.
#[derive(Debug, Default)]
struct Editor {
    /// Current document contents.
    text: String,
    /// Applied commands that can be undone.
    undo_stack: Vec<InsertChar>,
    /// Undone commands that can be reapplied.
    redo_stack: Vec<InsertChar>,
}

impl Editor {
    /// Inserts a character and records the command for undo.
    fn insert(&mut self, index: usize, value: char) {
        let byte_index = self
            .text
            .char_indices()
            .nth(index)
            .map_or(self.text.len(), |(byte, _)| byte);
        self.text.insert(byte_index, value);
        self.undo_stack.push(InsertChar { index, value });
        self.redo_stack.clear();
    }

    /// Reverses the latest insertion when one is available.
    fn undo(&mut self) {
        if let Some(command) = self.undo_stack.pop() {
            let byte_index = self
                .text
                .char_indices()
                .nth(command.index)
                .map_or(self.text.len(), |(byte, _)| byte);
            let removed = self.text.remove(byte_index);
            debug_assert_eq!(removed, command.value);
            self.redo_stack.push(command);
        }
    }

    /// Reapplies the latest undone insertion when one is available.
    fn redo(&mut self) {
        if let Some(command) = self.redo_stack.pop() {
            let byte_index = self
                .text
                .char_indices()
                .nth(command.index)
                .map_or(self.text.len(), |(byte, _)| byte);
            self.text.insert(byte_index, command.value);
            self.undo_stack.push(command);
        }
    }
}

/// Demonstrates applying a command, undoing it, and redoing it.
fn main() {
    let mut editor = Editor::default();
    editor.insert(0, 'R');
    println!("edited: {}", editor.text);
    editor.undo();
    println!("undone: {}", editor.text);
    editor.redo();
    println!("redone: {}", editor.text);
}

// ANCHOR_END: example

/// Runs the example.
#[allow(clippy::main_recursion)]
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that undo removes an edit and redo restores the same text.
    #[test]
    fn undo_and_redo_restore_the_document() {
        let mut editor = Editor::default();
        editor.insert(0, 'R');
        assert_eq!(editor.text, "R");
        editor.undo();
        assert_eq!(editor.text, "");
        editor.redo();
        assert_eq!(editor.text, "R");
    }
}
