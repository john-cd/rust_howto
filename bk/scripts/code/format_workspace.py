#!/usr/bin/env python3
"""Run rustfmt over Cargo targets in bounded command-line chunks."""

from __future__ import annotations

import json
import os
import subprocess
import sys


def main() -> int:
    if len(sys.argv) != 2 or sys.argv[1] not in {"check", "format"}:
        print("usage: format_workspace.py [check|format]", file=sys.stderr)
        return 2

    metadata = subprocess.run(
        ["cargo", "metadata", "--locked", "--format-version", "1"],
        check=True,
        capture_output=True,
        text=True,
    )
    cargo_metadata = json.loads(metadata.stdout)
    workspace_root = cargo_metadata["workspace_root"]
    toolchain = os.environ.get("RUSTFMT_TOOLCHAIN")
    check = sys.argv[1] == "check"
    rustfmt = ["rustup", "which", "rustfmt"]
    if toolchain:
        rustfmt.extend(["--toolchain", toolchain])
    rustfmt_path = subprocess.run(
        rustfmt,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()

    targets: dict[str, set[str]] = {}
    for package in cargo_metadata["packages"]:
        if package["source"] is not None:
            continue
        edition = package["edition"]
        sources = targets.setdefault(edition, set())
        sources.update(
            target["src_path"]
            for target in package["targets"]
            if target["src_path"].endswith(".rs")
        )

    config_path = os.path.join(workspace_root, "rustfmt.toml")
    chunk_size = 16  # Keep each Windows process command line well below 32,767 chars.
    for edition, source_set in sorted(targets.items()):
        sources = sorted(source_set)
        print(
            f"{'Checking' if check else 'Formatting'} {len(sources)} Rust targets "
            f"(edition {edition}) in {workspace_root}",
            flush=True,
        )
        for start in range(0, len(sources), chunk_size):
            command = [rustfmt_path, "--edition", edition]
            if os.path.isfile(config_path):
                command.extend(["--config-path", config_path])
            if check:
                command.append("--check")
            command.extend(sources[start : start + chunk_size])
            result = subprocess.run(command, check=False)
            if result.returncode:
                print(
                    f"rustfmt exited with {result.returncode} for target "
                    f"{sources[start]}",
                    file=sys.stderr,
                )
                return result.returncode

    if check:
        print("Rust formatting checks passed.", flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
