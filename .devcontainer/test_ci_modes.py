"""Check CI entrypoint modes without compiling or installing anything."""

import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


class CiModes(unittest.TestCase):
    """Verify render-only mode preserves book checks and avoids Rust validation."""

    def run_mode(self, **overrides):
        """Run the real entrypoint against logging stand-ins in a disposable tree."""
        script = Path(__file__).with_name("ci.sh").resolve()
        bash = shutil.which("bash")
        if os.name == "nt":
            bash = "C:/Program Files/Git/bin/bash.exe"
        with tempfile.TemporaryDirectory(prefix="rust-howto-ci-") as directory:
            root = Path(directory)
            binary = root / "mock-bin"
            binary.mkdir()
            (root / "bk/crates").mkdir(parents=True)
            spelling = root / "bk/scripts/spelling"
            spelling.mkdir(parents=True)
            (root / "rustfmt-toolchain.toml").write_text('channel = "nightly-test"\n')
            mock = '#!/bin/bash\nprintf "%s %s\\n" "${0##*/}" "$*" >> "$CI_LOG"\n'
            for name in ("cargo", "just", "python3", "free", "df", "du"):
                command = binary / name
                command.write_text(mock, newline="\n")
                command.chmod(0o755)
            sleep = binary / "sleep"
            sleep.write_text("#!/bin/bash\nexec /usr/bin/sleep 0.01\n", newline="\n")
            sleep.chmod(0o755)
            spell = spelling / "spellcheck.sh"
            spell.write_text(mock, newline="\n")
            spell.chmod(0o755)
            log = root / "commands.log"
            env = dict(
                os.environ,
                CI_LOG=log.as_posix(),
                FIXTURE_BIN=binary.as_posix(),
                CI_SCRIPT=script.as_posix(),
            )
            for key in (
                "VALIDATE_CODE",
                "SKIP_TESTS",
                "SKIP_BOOK_BUILD",
                "BUILD_PDF",
                "NEXTEST_ARGS",
            ):
                env.pop(key, None)
            env["BUILD_PDF"] = "true"
            env.update(overrides)
            result = subprocess.run(
                [
                    bash,
                    "-c",
                    'export PATH="$(cygpath -u "$FIXTURE_BIN" 2>/dev/null || printf "%s" "$FIXTURE_BIN"):$PATH"; bash "$CI_SCRIPT"',
                ],
                cwd=root,
                env=env,
                capture_output=True,
                text=True,
                timeout=20,
                check=False,
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            return log.read_text()

    def test_render_only_keeps_spelling_and_book_checks(self):
        """Render-only mode performs book checks without auditing or compiling Rust."""
        log = self.run_mode(VALIDATE_CODE="false")
        self.assertIn("spellcheck.sh list", log)
        self.assertIn("python3 scripts/book/refdefs.py check --book-root .", log)
        self.assertIn("just bb", log)
        self.assertNotIn("cargo ", log)
        self.assertNotIn("just fmt-toml-check", log)

    def test_validation_runs_all_tests_once_without_rendering(self):
        """The validation stage retains build, Clippy, Nextest and doctests."""
        log = self.run_mode(SKIP_BOOK_BUILD="true")
        self.assertEqual(log.count("cargo build "), 1)
        self.assertEqual(log.count("cargo clippy "), 1)
        self.assertEqual(log.count("cargo nextest run "), 1)
        self.assertIn("cargo test --workspace --doc", log)
        self.assertNotIn("just bb", log)

    def test_skip_tests_does_not_skip_compilation(self):
        """The legacy skip-tests flag still compiles and lints before rendering."""
        log = self.run_mode(SKIP_TESTS="true")
        self.assertIn("cargo build ", log)
        self.assertIn("cargo clippy ", log)
        self.assertNotIn("cargo nextest", log)
        self.assertIn("just bb", log)


if __name__ == "__main__":
    unittest.main()
