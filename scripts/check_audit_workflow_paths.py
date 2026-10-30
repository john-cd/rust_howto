#!/usr/bin/env python3
"""Check that the dependency-audit workflow triggers on dependency-only changes.

Every audited workspace's manifests, lockfile and `cargo deny` policy must be
covered by both `push.paths` and `pull_request.paths` in
`.github/workflows/audit_dependencies.yml`.

Run: python3 scripts/check_audit_workflow_paths.py
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
WORKFLOW = ROOT / ".github" / "workflows" / "audit_dependencies.yml"

# Audited workspace -> policy file.
WORKSPACES = {
    "bk/crates": "bk/crates/deny.toml",
    "later/crates": "later/crates/deny.toml",
    "tools": "bk/crates/deny.toml",
    "xmpl": "bk/crates/deny.toml",
}


def trigger_paths(text: str) -> dict[str, set[str]]:
    """Extract the `paths:` lists under `push:` and `pull_request:`."""
    result: dict[str, set[str]] = {}
    event = None
    in_paths = False
    for line in text.splitlines():
        if re.match(r"^\S", line):
            event = None
        m = re.match(r"^  (push|pull_request):\s*$", line)
        if m:
            event, in_paths = m.group(1), False
            result[event] = set()
            continue
        if event is None:
            continue
        if re.match(r"^    paths:\s*$", line):
            in_paths = True
        elif in_paths and (m := re.match(r'^      - "?([^"#]+?)"?\s*$', line)):
            result[event].add(m.group(1))
        elif re.match(r"^    \S", line):
            in_paths = False
    return result


def missing(paths: dict[str, set[str]]) -> list[str]:
    """Return one message per required path an event does not list."""
    problems = []
    for event in ("push", "pull_request"):
        listed = paths.get(event, set())
        required = {WORKFLOW.relative_to(ROOT).as_posix()}
        for ws, policy in WORKSPACES.items():
            required |= {
                f"{ws}/Cargo.lock",
                f"{ws}/Cargo.toml",
                f"{ws}/**/Cargo.toml",
                policy,
            }
        problems += [f"{event}: missing {p}" for p in sorted(required - listed)]
    return problems


def main() -> int:
    problems = missing(trigger_paths(WORKFLOW.read_text()))
    for p in problems:
        print(p, file=sys.stderr)
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
