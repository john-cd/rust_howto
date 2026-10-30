// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Negotiate a WebRTC audio stream between two peers with `str0m`.
//!
//! `str0m` is Sans-I/O: it never opens sockets or spawns threads. The
//! application supplies the signaling channel, here two function calls in the
//! same process, and later moves UDP packets between the peers. The offerer
//! creates a session description (SDP) with one audio "m-line"; the answerer
//! accepts it and returns an answer; the offerer applies that answer.

use std::net::SocketAddr;
use std::time::Instant;

use anyhow::Result;
use str0m::Candidate;
use str0m::Rtc;
use str0m::media::Direction;
use str0m::media::MediaKind;

/// Creates a peer that advertises a local UDP address as an ICE candidate.
fn new_peer(address: &str) -> Result<Rtc> {
    let mut rtc = Rtc::new(Instant::now());
    let address: SocketAddr = address.parse()?;
    rtc.add_local_candidate(Candidate::host(address, "udp")?);
    Ok(rtc)
}

/// Runs the offer/answer exchange and returns the offer's SDP text.
fn negotiate() -> Result<String> {
    let mut offerer = new_peer("192.0.2.1:5000")?;
    let mut answerer = new_peer("192.0.2.2:5000")?;

    // The offerer sends audio; a valid offer needs at least one media line.
    let mut changes = offerer.sdp_api();
    changes.add_media(MediaKind::Audio, Direction::SendOnly, None, None, None);
    let (offer, pending) = changes.apply().ok_or_else(|| {
        anyhow::anyhow!("adding media should produce an offer")
    })?;
    let offer_sdp = offer.to_sdp_string();

    // Signaling is up to the application: send the offer to the other peer.
    let answer = answerer.sdp_api().accept_offer(offer)?;
    let answer_sdp = answer.to_sdp_string();
    offerer.sdp_api().accept_answer(pending, answer)?;

    // The answerer receives audio, so it answers `a=recvonly`.
    assert!(answer_sdp.contains("m=audio"));
    assert!(answer_sdp.contains("a=recvonly"));
    Ok(offer_sdp)
}

/// Prints the offer that a signaling server would forward.
fn main() -> Result<()> {
    let offer_sdp = negotiate()?;
    println!("{offer_sdp}");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }

    /// The offer advertises one send-only audio line and the local candidate.
    #[test]
    fn offer_describes_audio_and_candidate() -> Result<()> {
        let sdp = negotiate()?;
        assert_eq!(sdp.matches("m=audio").count(), 1);
        assert!(sdp.contains("a=sendonly"));
        assert!(sdp.contains("192.0.2.1"));
        Ok(())
    }
}
