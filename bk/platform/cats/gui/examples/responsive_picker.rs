// TODO Reviewed ??-??-2026.
//! Picks and reads a text file without waiting in the UI callback.

// ANCHOR: example
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use eframe::egui;

type LoadResult = std::io::Result<Option<String>>;

/// Reads selected text and preserves cancellation as a separate outcome.
fn load(path: Option<PathBuf>) -> LoadResult {
    path.map(fs::read_to_string).transpose()
}

/// Displays a worker's result when it becomes available.
#[derive(Default)]
struct App {
    pending: Option<mpsc::Receiver<LoadResult>>,
    text: String,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if ui
            .add_enabled(self.pending.is_none(), egui::Button::new("Open text file"))
            .clicked()
        {
            let (sender, receiver) = mpsc::channel();
            thread::spawn(move || {
                let result = load(
                    rfd::FileDialog::new()
                        .add_filter("Text", &["txt"])
                        .pick_file(),
                );
                let _ = sender.send(result);
            });
            self.pending = Some(receiver);
        }

        if let Some(receiver) = &self.pending {
            match receiver.try_recv() {
                Ok(result) => {
                    self.pending = None;
                    match result {
                        Ok(Some(text)) => self.text = text,
                        Ok(None) => {}
                        Err(error) => self.text = error.to_string(),
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {
                    ui.ctx().request_repaint_after(Duration::from_millis(32));
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    self.text = "File worker stopped".into();
                }
            }
        }
        egui::ScrollArea::vertical().show(ui, |ui| ui.label(&self.text));
    }
}

/// Opens the native text-file viewer.
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Text file viewer",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(App::default()))),
    )
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Verifies cancellation and reads a selected fixture without opening a dialog.
    #[test]
    fn loads_selection() -> std::io::Result<()> {
        let fixture = tempfile::NamedTempFile::new()?;
        std::fs::write(fixture.path(), "sample")?;
        assert_eq!(super::load(None)?, None);
        assert_eq!(
            super::load(Some(fixture.path().into()))?.as_deref(),
            Some("sample")
        );
        Ok(())
    }
}
