# TODO Reviewed ??-??-2026.

"""Encode a generated clip and verify extracted audio and video."""

import argparse
import struct
import subprocess
import tempfile
import wave
from pathlib import Path

WIDTH, HEIGHT = 64, 48
SAMPLE_RATE = 8_000


def round_trip(ffmpeg: str, output: Path) -> None:
    """Encode a one-second test clip and check the decoded media."""
    clip = output / "fixture.mkv"
    thumbnail = output / "thumbnail.png"
    audio = output / "audio.wav"

    def run(*args: str | Path) -> bytes:
        return subprocess.run(
            [ffmpeg, "-nostdin", "-v", "error", *args],
            capture_output=True,
            check=True,
            timeout=30,
        ).stdout

    run(
        "-f",
        "lavfi",
        "-i",
        f"color=red:s={WIDTH}x{HEIGHT}:r=8:d=1",
        "-f",
        "lavfi",
        "-i",
        f"sine=frequency=440:sample_rate={SAMPLE_RATE}:duration=1",
        "-c:v",
        "ffv1",
        "-c:a",
        "pcm_s16le",
        clip,
    )
    run("-i", clip, "-map", "0:v:0", "-frames:v", "1", thumbnail)
    run("-i", clip, "-map", "0:a:0", "-c:a", "pcm_s16le", audio)

    with wave.open(str(audio), "rb") as wav:
        if (
            wav.getnchannels(),
            wav.getsampwidth(),
            wav.getframerate(),
            wav.getnframes(),
        ) != (1, 2, SAMPLE_RATE, SAMPLE_RATE):
            raise ValueError("expected one second of mono 16-bit PCM at 8 kHz")
        samples = struct.iter_unpack("<h", wav.readframes(SAMPLE_RATE))
        if max(abs(sample) for (sample,) in samples) not in range(3_000, 5_000):
            raise ValueError("decoded tone has an unexpected amplitude")

    pixels = run("-i", thumbnail, "-pix_fmt", "rgb24", "-f", "rawvideo", "-")
    png = thumbnail.read_bytes()
    if png[:8] != b"\x89PNG\r\n\x1a\n" or struct.unpack(">II", png[16:24]) != (
        WIDTH,
        HEIGHT,
    ):
        raise ValueError("thumbnail must be a 64 by 48 PNG")
    if len(pixels) != WIDTH * HEIGHT * 3:
        raise ValueError("unexpected RGB frame size")
    # YUV-to-RGB rounding may vary slightly with the FFmpeg build.
    red, green, blue = pixels[:3]
    if red < 245 or green > 5 or blue > 5:
        raise ValueError("expected a red test frame")

    print(f"Verified {WIDTH}x{HEIGHT} red PNG and {SAMPLE_RATE} mono PCM samples.")


def main() -> None:
    """Run the fixture in a temporary directory and report actionable failures."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ffmpeg", default="ffmpeg", help="FFmpeg executable path")
    args = parser.parse_args()
    try:
        with tempfile.TemporaryDirectory(prefix="rust-howto-media-") as directory:
            round_trip(args.ffmpeg, Path(directory))
    except FileNotFoundError:
        parser.exit(1, f"FFmpeg executable not found: {args.ffmpeg}\n")
    except subprocess.CalledProcessError as error:
        detail = error.stderr.decode(errors="replace").strip()
        parser.exit(1, f"FFmpeg exited with status {error.returncode}: {detail}\n")
    except (subprocess.TimeoutExpired, ValueError) as error:
        parser.exit(1, f"Media verification failed: {error}\n")


if __name__ == "__main__":
    main()
