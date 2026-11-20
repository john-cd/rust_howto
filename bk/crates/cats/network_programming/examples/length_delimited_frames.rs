//! Decodes length-prefixed messages across arbitrary stream read boundaries.

// ANCHOR: example
use futures_util::StreamExt;
use futures_util::poll;
use tokio::io::AsyncWriteExt;
use tokio::io::duplex;
use tokio_util::codec::FramedRead;
use tokio_util::codec::LengthDelimitedCodec;

/// Decodes a split message and two messages written together.
///
/// # Errors
///
/// Returns an error if writing, decoding, or reading a frame fails.
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let (mut writer, reader) = duplex(64);
    let codec = LengthDelimitedCodec::builder()
        .max_frame_length(16)
        .new_codec();
    let mut frames = FramedRead::new(reader, codec);

    // A big-endian u32 length precedes each payload. Start with only part of "cat".
    writer.write_all(b"\x00\x00\x00\x03c").await?;
    assert!(poll!(frames.next()).is_pending());

    writer.write_all(b"at").await?;
    let first = frames
        .next()
        .await
        .transpose()?
        .ok_or_else(|| anyhow::anyhow!("missing first frame"))?;
    assert_eq!(&*first, b"cat");

    writer
        .write_all(b"\x00\x00\x00\x03dog\x00\x00\x00\x03owl")
        .await?;
    writer.shutdown().await?;

    for expected in [b"dog", b"owl"] {
        let frame = frames
            .next()
            .await
            .transpose()?
            .ok_or_else(|| anyhow::anyhow!("missing frame"))?;
        assert_eq!(&*frame, expected);
    }
    assert!(frames.next().await.is_none());
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Confirms partial reads and coalesced messages yield the original payloads.
    #[test]
    fn reconstructs_message_boundaries() -> anyhow::Result<()> {
        main()
    }

    /// Rejects a length prefix above the configured maximum before reading a payload.
    #[tokio::test]
    async fn rejects_oversized_frame_header() -> anyhow::Result<()> {
        let (mut writer, reader) = duplex(64);
        let codec = LengthDelimitedCodec::builder()
            .max_frame_length(16)
            .new_codec();
        let mut frames = FramedRead::new(reader, codec);
        writer.write_all(&17_u32.to_be_bytes()).await?;
        assert!(matches!(
            poll!(frames.next()),
            std::task::Poll::Ready(Some(Err(_)))
        ));
        Ok(())
    }
}
