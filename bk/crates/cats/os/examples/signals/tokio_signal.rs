// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Shut an async service down on Ctrl-C or SIGTERM with Tokio.

use std::future::Future;
use std::time::Duration;

use tokio::signal;
use tokio::time;

/// Completes when the process receives Ctrl-C or, on Unix, SIGTERM.
async fn shutdown_signal() -> anyhow::Result<()> {
    let ctrl_c = signal::ctrl_c();

    #[cfg(unix)]
    {
        let mut sigterm =
            signal::unix::signal(signal::unix::SignalKind::terminate())?;
        tokio::select! {
            result = ctrl_c => result?,
            _ = sigterm.recv() => {}
        }
    }

    #[cfg(not(unix))]
    ctrl_c.await?;

    Ok(())
}

/// Ticks every few milliseconds until `shutdown` completes, then returns the
/// number of ticks. Taking the shutdown future as a parameter makes the
/// function testable without sending a real signal.
async fn serve_until(shutdown: impl Future<Output = ()>) -> u32 {
    let mut ticks = 0;
    let mut interval = time::interval(Duration::from_millis(5));
    tokio::pin!(shutdown);
    loop {
        tokio::select! {
            () = &mut shutdown => break,
            _ = interval.tick() => ticks += 1,
        }
    }
    // Finish in-flight requests, flush buffers, etc. here.
    ticks
}

/// Runs the service until a signal arrives; this example stops after a short
/// timeout so that it terminates without a keyboard.
pub fn run() -> anyhow::Result<()> {
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        // In a real program, pass `async { let _ = shutdown_signal().await; }`
        // instead of the timeout.
        let ticks = serve_until(time::sleep(Duration::from_millis(50))).await;
        println!("served {ticks} ticks before shutting down");
        Ok(())
    })
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn stops_immediately_when_shutdown_is_ready() {
        assert_eq!(serve_until(std::future::ready(())).await, 0);
    }

    #[tokio::test]
    async fn keeps_serving_until_shutdown() {
        let ticks = serve_until(time::sleep(Duration::from_millis(30))).await;
        assert!(ticks >= 1);
    }

    #[test]
    fn run_completes() -> anyhow::Result<()> {
        run()
    }
}
