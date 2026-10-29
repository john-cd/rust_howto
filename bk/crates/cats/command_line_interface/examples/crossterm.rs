// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Terminal control with `crossterm`: raw mode, colors, cursor movement and key
//! events.

use std::io::Write as _;
use std::io::stdout;

use crossterm::ExecutableCommand as _;
use crossterm::cursor;
use crossterm::event;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::queue;
use crossterm::style;
use crossterm::style::Color;
use crossterm::style::Print;
use crossterm::style::SetBackgroundColor;
use crossterm::style::SetForegroundColor;
use crossterm::terminal;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;

/// Prints colored text and reacts to key presses until `q` is pressed.
fn main() -> anyhow::Result<()> {
    let mut stdout = stdout();

    // Enter raw mode and enable mouse capture:
    terminal::enable_raw_mode()?;
    // Execute the command immediately:
    execute!(stdout, EnableMouseCapture)?;

    // Clear the screen and move cursor to the top left.
    // We can queue commands instead of executing them directly.
    // When we call `Write::flush`, these commands will be executed.
    queue!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    // Many other commands...
    stdout.flush()?;

    // Display instructions:
    println!("Press 'q' to exit.");
    // Set the foreground color to blue and the background color to white:
    execute!(
        stdout,
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::White),
        Print("Hello, Crossterm!\n")
    )?;
    // We can use the `execute` function rather than the macro.
    // The `execute` function returns itself,
    // therefore we could queue another command.
    // Reset the color to the default:
    stdout.execute(style::ResetColor)?;

    loop {
        // Listen for key events:
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Up => {
                    println!("Up arrow key pressed.");
                }
                KeyCode::Down => {
                    println!("Down arrow key pressed.");
                }
                KeyCode::Left => {
                    println!("Left arrow key pressed.");
                }
                KeyCode::Right => {
                    println!("Right arrow key pressed.");
                }
                _ => {}
            }
        }
    }

    // Leave raw mode and disable mouse capture:
    terminal::disable_raw_mode()?;
    execute!(stdout, DisableMouseCapture)?;

    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the interactive example when a terminal and key input are
    /// available.
    #[test]
    #[ignore = "requires an interactive terminal and key input"]
    fn test_crossterm_handles_terminal_input() -> anyhow::Result<()> {
        main()
    }

    #[test]
    fn terminal_commands_produce_the_expected_escape_sequences()
    -> anyhow::Result<()> {
        let mut out: Vec<u8> = Vec::new();
        queue!(out, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        assert_eq!(String::from_utf8(out.clone())?, "\u{1b}[2J\u{1b}[1;1H");

        out.clear();
        execute!(
            out,
            SetForegroundColor(Color::Blue),
            Print("hi"),
            style::ResetColor
        )?;
        let text = String::from_utf8(out)?;
        assert!(text.contains("hi"));
        assert!(text.starts_with("\u{1b}[38;5;12m"), "{text:?}");
        assert!(text.ends_with("\u{1b}[0m"));
        Ok(())
    }
}
