// ANCHOR: example
use std::io;

use anyhow::Result;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::text::Text;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;

fn main() {
    // Create a new DefaultTerminal and initialize it
    // with the following defaults:
    // - Backend: CrosstermBackend writing to Stdout
    // - Raw mode enabled
    // - Alternate screen buffer enabled
    // - Panic hook installed
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    // Restore the terminal to its original state
    ratatui::restore();
    if let Err(err) = app_result {
        println!("{:?}", err);
    }
}

#[derive(Debug, Default)]
struct App {
    mode: Mode,
}

#[derive(Debug, Default, PartialEq)]
enum Mode {
    #[default]
    Running,
    // Add more states / screens as needed...
    Exiting,
    Done,
}

impl App {
    // The `run` function contains the main loop of the application.
    // It repeatedly draws the UI and handles input events.
    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.mode != Mode::Done {
            // `draw` must render the entire UI.
            // You should only call it once for each pass
            // through your application’s main loop.
            terminal.draw(|frame| self.ui(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn ui(&self, frame: &mut Frame) {
        // The most important method on `Frame` is `render_widget()`,
        // which renders any type that implements the `Widget` trait:
        // `Paragraph`, `Line`... Here, we render the `App` itself,
        // which implements `Widget` below.
        frame.render_widget(self, frame.area());
    }

    /// Update the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        // Note that the `event::read` function blocks until there is an event.
        match event::read()? {
            // It's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.mode {
            Mode::Running => {
                if key_event.code == KeyCode::Char('q') {
                    self.mode = Mode::Exiting
                }
                // Call additional methods here to handle other
                // key events
            }
            Mode::Exiting => match key_event.code {
                KeyCode::Char('y') => self.mode = Mode::Done,
                KeyCode::Char('n') => self.mode = Mode::Running,
                _ => {}
            },
            _ => {}
        }
    }
}

// A common compositional pattern is to have a single
// root widget (the `App` struct itself).
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Create a layout with three vertical chunks using Layout.
        // Each chunk will hold a different widget.
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(area);

        // Within the root widget (and its children), we call the `render`
        // method, passing in the area which you want to render the
        // widgets to. Here, we draw a `Block` widget with a title.
        let title_block = Block::default().borders(Borders::ALL).title(
            Span::styled("Ratatui example", Style::default().fg(Color::Yellow)),
        );
        title_block.render(chunks[0], buf);

        // You can create a custom Widget struct to encapsulate the rendering
        // logic:
        MiddleWidget::new("This is an example of using ratatui to create terminal user interfaces")
            .render(chunks[1], buf);

        // Draw another Paragraph widget in the third chunk as a footer,
        // with instructions dependent on the current mode.
        let current_navigation_text = match self.mode {
            Mode::Running => Span::styled(
                "Press 'q' to quit",
                Style::default().fg(Color::Green),
            ),
            Mode::Exiting => Span::styled(
                "Press 'y' to confirm, 'n' to cancel",
                Style::default().fg(Color::LightRed),
            ),
            Mode::Done => {
                Span::styled("Goodbye!", Style::default().fg(Color::White))
            }
        };

        let footer = Paragraph::new(current_navigation_text)
            .block(Block::default().borders(Borders::ALL));
        footer.render(chunks[2], buf);
    }
}

struct MiddleWidget {
    text: String,
}

impl MiddleWidget {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Widget for MiddleWidget {
    // Draw a `Paragraph` widget with some example text.
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = Text::raw(self.text);
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Text"))
            .style(Style::default().fg(Color::White));
        paragraph.render(area, buf);
    }
}

#[test]
fn handle_key_event() -> io::Result<()> {
    let mut app = App::default();
    app.handle_key_event(KeyCode::Char('q').into());
    assert_eq!(app.mode, Mode::Exiting);
    // More tests here...
    Ok(())
}
// ANCHOR_END: example

#[test]
#[ignore = "requires user interaction"]
fn test() {
    main();
}
