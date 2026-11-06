#!/usr/bin/env python3
# TODO Reviewed ??-??-2026

"""Keep the lint tables of every workspace identical to the shared profiles.

Each workspace manifest ends with `[workspace.lints.*]` tables (or `[lints.*]` for
the single-package `publish` crate). The single source of truth is
`scripts/lints/<profile>.toml`:

- `examples`: book and staging example workspaces (`bk/crates`, `later/crates`,
  `xmpl`, `playground`), which relax lints that examples intentionally violate.
- `strict`: maintained tooling (`tools`, `publish`), which denies unsafe code and
  missing documentation.

Run `python3 scripts/sync_lints.py` to rewrite the manifests, or add `--check` to
fail when a manifest has drifted (for CI and hooks).
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# Manifest path -> (profile, uses `[workspace.lints]` tables).
TARGETS = {
    "bk/crates/Cargo.toml": ("examples", True),
    "later/crates/Cargo.toml": ("examples", True),
    "xmpl/Cargo.toml": ("examples", True),
    "playground/Cargo.toml": ("examples", True),
    "tools/Cargo.toml": ("strict", True),
    "publish/Cargo.toml": ("strict", False),
}

START = re.compile(r"^\[(?:workspace\.)?lints\.rust\]", re.M)
END = re.compile(r"^\[(?!(?:workspace\.)?lints\.)", re.M)


def render(manifest: str, profile: str, workspace: bool) -> str:
    """Return `manifest` with its lint tables replaced by the profile's tables."""
    shared = (ROOT / "scripts" / "lints" / f"{profile}.toml").read_text(encoding="utf-8")
    # Drop the profile header comment; keep the tables and their comments.
    body = shared[shared.index("[workspace.lints.rust]"):].rstrip("\n") + "\n"
    if not workspace:
        body = body.replace("[workspace.lints.", "[lints.")
    start = START.search(manifest)
    if start is None:
        raise SystemExit("no lints table found")
    end = END.search(manifest, start.end())
    tail = ""
    if end:
        # Keep the comment block that introduces the next table with that table.
        block = re.search(r"(?:^#[^\n]*\n)*\Z", manifest[start.end() : end.start()], re.M)
        tail = "\n" + manifest[end.start() - len(block.group(0)) :]
    return manifest[: start.start()] + body + tail


def main() -> int:
    check = "--check" in sys.argv[1:]
    drifted = []
    for rel, (profile, workspace) in TARGETS.items():
        path = ROOT / rel
        current = path.read_bytes().decode("utf-8")
        eol = "\r\n" if "\r\n" in current else "\n"
        updated = render(current.replace("\r\n", "\n"), profile, workspace).replace("\n", eol)
        if updated != current:
            drifted.append(rel)
            if not check:
                path.write_bytes(updated.encode("utf-8"))
    if drifted:
        verb = "drifted from" if check else "updated to match"
        print(f"{verb} the shared lint profiles: {', '.join(drifted)}")
        return 1 if check else 0
    print("lint tables match the shared profiles")
    return 0


if __name__ == "__main__":
    sys.exit(main())
