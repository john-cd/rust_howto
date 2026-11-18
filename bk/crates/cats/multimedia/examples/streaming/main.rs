//! Runnable entry point for the streaming example.
/// Provides webrtc audio negotiation behavior for the streaming component.
mod webrtc_audio_negotiation;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    webrtc_audio_negotiation::run()
}
