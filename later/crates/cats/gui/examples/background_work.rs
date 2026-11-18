// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Keeps bounded CPU work off the UI thread and rejects stale results.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::thread::JoinHandle;
use std::thread::{self};
use std::time::Duration;
use std::time::Instant;

/// One result tied to the exact input revision used by its worker.
#[derive(Debug)]
struct Reply {
    revision: u64,
    value: Result<u64, String>,
}

/// One owned worker, one reply slot, and a cooperative shutdown flag.
struct Worker {
    result: mpsc::Receiver<Reply>,
    handle: Option<JoinHandle<()>>,
    cancelled: Arc<AtomicBool>,
}

impl Worker {
    /// Starts a bounded computation from an immutable input snapshot.
    fn start(revision: u64, limit: u32, context: egui::Context) -> Self {
        let (sender, result) = mpsc::sync_channel(1);
        let cancelled = Arc::new(AtomicBool::new(false));
        let stop = Arc::clone(&cancelled);
        let handle = thread::spawn(move || {
            let value = if limit > 10_000 {
                Err("limit must be at most 10000".to_owned())
            } else {
                let mut total = 0_u64;
                for n in 0..limit {
                    if stop.load(Ordering::Relaxed) {
                        return;
                    }
                    total += u64::from(n).pow(2);
                }
                Ok(total)
            };
            // A closed receiver means the owner has gone away.
            if sender.send(Reply { revision, value }).is_ok() {
                context.request_repaint();
            }
        });
        Self {
            result,
            handle: Some(handle),
            cancelled,
        }
    }

    /// Takes a completed reply without joining an active thread on the UI thread.
    fn poll(&mut self) -> Option<Result<Reply, String>> {
        if !self.handle.as_ref()?.is_finished() {
            return None;
        }
        let handle = self.handle.take()?;
        if handle.join().is_err() {
            return Some(Err("worker panicked".to_owned()));
        }
        Some(self.result.try_recv().map_err(|error| error.to_string()))
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.cancelled.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            // This worker performs only a bounded loop with cancellation checks.
            let _outcome = handle.join();
        }
    }
}

/// UI-owned state; workers never mutate widgets or the current input revision.
#[derive(Default)]
struct App {
    revision: u64,
    limit: u32,
    worker: Option<Worker>,
    message: String,
    smoke_completion: Option<Arc<AtomicBool>>,
    started: Option<Instant>,
}

impl App {
    /// Installs a result only when it still describes the visible input.
    fn accept(&mut self, reply: Reply) {
        if reply.revision != self.revision {
            self.message = "Discarded an obsolete result; run again.".to_owned();
            return;
        }
        self.message = match reply.value {
            Ok(value) => format!("Sum of squares: {value}"),
            Err(error) => format!("Computation failed: {error}"),
        };
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Some(reply) = self.worker.as_mut().and_then(Worker::poll) {
            self.worker = None;
            match reply {
                Ok(reply) => self.accept(reply),
                Err(error) => self.message = error,
            }
        }
        ui.heading("Background calculation");
        if ui
            .add(egui::Slider::new(&mut self.limit, 0..=10_000).text("Samples"))
            .changed()
        {
            self.revision = self
                .revision
                .checked_add(1)
                .expect("revision cannot wrap in a UI session");
            self.message = "Input changed; calculate a fresh result.".to_owned();
        }
        if ui
            .add_enabled(self.worker.is_none(), egui::Button::new("Calculate"))
            .clicked()
        {
            self.worker = Some(Worker::start(self.revision, self.limit, ui.ctx().clone()));
        }
        if self.worker.is_some() {
            ui.spinner();
            // Polling also covers the interval between sending and thread completion.
            ui.ctx().request_repaint_after(Duration::from_millis(16));
        }
        ui.label(&self.message);
        if let Some(completion) = &self.smoke_completion {
            let finished = self.worker.is_none();
            let expired = self
                .started
                .is_some_and(|start| start.elapsed() > Duration::from_secs(5));
            if finished || expired {
                completion.store(
                    finished && self.message == "Sum of squares: 333283335000",
                    Ordering::Relaxed,
                );
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
    }
}

/// Runs the native GUI; tests exercise the state/worker contract without a display.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let smoke = std::env::args_os().skip(1).any(|arg| arg == "--smoke");
    let completion = Arc::new(AtomicBool::new(false));
    let signal = Arc::clone(&completion);
    eframe::run_native(
        "Background work",
        eframe::NativeOptions::default(),
        Box::new(move |creation| {
            let mut app = App::default();
            if smoke {
                app.worker = Some(Worker::start(0, 10_000, creation.egui_ctx.clone()));
                app.started = Some(Instant::now());
                app.smoke_completion = Some(signal);
            }
            Ok(Box::new(app))
        }),
    )?;
    if smoke && !completion.load(Ordering::Relaxed) {
        return Err("native smoke check did not complete its calculation".into());
    }
    if smoke {
        println!("Native worker result displayed and window closed.");
    }
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// An older revision cannot replace the current input's result.
    #[test]
    fn obsolete_reply_is_rejected() {
        let mut app = App {
            revision: 2,
            ..Default::default()
        };
        app.accept(Reply {
            revision: 1,
            value: Ok(42),
        });
        assert!(app.message.contains("obsolete"));
        app.accept(Reply {
            revision: 2,
            value: Ok(30),
        });
        assert_eq!(app.message, "Sum of squares: 30");
    }

    /// Worker failures remain visible rather than becoming fabricated values.
    #[test]
    fn computation_errors_are_displayed() {
        let mut app = App::default();
        app.accept(Reply {
            revision: 0,
            value: Err("invalid input".to_owned()),
        });
        assert!(app.message.contains("invalid input"));
    }

    /// A real worker sends the expected result and can be joined cleanly.
    #[test]
    fn worker_completes() {
        let mut worker = Worker::start(3, 4, egui::Context::default());
        worker
            .handle
            .take()
            .expect("worker handle")
            .join()
            .expect("worker completes");
        let reply = worker
            .result
            .recv_timeout(Duration::from_secs(2))
            .expect("bounded reply");
        assert_eq!(reply.revision, 3);
        assert_eq!(reply.value, Ok(14));
    }

    /// Dropping a worker requests cancellation and reclaims its thread.
    #[test]
    fn shutdown_reclaims_worker() {
        drop(Worker::start(0, 10_000, egui::Context::default()));
    }
}
