# TODO Reviewed ??-??-2026.
"""Compare two command binaries using interleaved, repeated wall-clock runs."""

import argparse
import json
import statistics
import subprocess
import sys
import time
from pathlib import Path


def run(binary: Path) -> tuple[float, bytes]:
    start = time.perf_counter()
    result = subprocess.run([str(binary.resolve())], check=True, capture_output=True)
    return time.perf_counter() - start, result.stdout


def spread(samples: list[float]) -> float:
    ordered = sorted(samples)
    return (ordered[-2] - ordered[1]) / statistics.median(ordered)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("baseline", type=Path)
    parser.add_argument("candidate", type=Path)
    parser.add_argument("--threshold", type=float, default=0.10)
    parser.add_argument("--output", type=Path, default=Path("benchmark-results.json"))
    args = parser.parse_args()
    if not 0 < args.threshold < 1:
        parser.error("--threshold must be between zero and one")

    expected = run(args.baseline)[1]
    if run(args.candidate)[1] != expected:
        print("outputs differ; benchmark comparison is invalid", file=sys.stderr)
        return 2

    samples: dict[str, list[float]] = {"baseline": [], "candidate": []}
    for _ in range(15):
        for name, binary in (("baseline", args.baseline), ("candidate", args.candidate)):
            duration, output = run(binary)
            if output != expected:
                print(f"{name} output changed during measurement", file=sys.stderr)
                return 2
            samples[name].append(duration)

    base = statistics.median(samples["baseline"])
    candidate = statistics.median(samples["candidate"])
    ratio = candidate / base
    args.output.write_text(json.dumps({"seconds": samples, "threshold": args.threshold}, indent=2) + "\n", encoding="utf-8")
    print(f"baseline median: {base:.6f}s; candidate median: {candidate:.6f}s; ratio: {ratio:.3f}")
    print(f"baseline spread: {spread(samples['baseline']):.1%}; candidate spread: {spread(samples['candidate']):.1%}")
    if max(spread(values) for values in samples.values()) > 0.05:
        print("measurement is noisy; rerun on a stable runner before gating")
        return 0
    if ratio > 1 + args.threshold:
        print(f"regression exceeds {args.threshold:.0%} threshold", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
