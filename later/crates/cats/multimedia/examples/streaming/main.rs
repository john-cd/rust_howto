mod webrtc_audio_negotiation;

fn main() -> impl std::process::Termination {
    webrtc_audio_negotiation::run()
}
