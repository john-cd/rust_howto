"""Regression tests for the example-test survey heuristics."""

from pathlib import Path
from tempfile import TemporaryDirectory
import unittest

from scripts.check_example_tests import classify


class ExampleTestSurveyTests(unittest.TestCase):
    """Checks behavior used to classify example tests."""

    def test_ensure_in_main_counts_as_a_behavior_check(self) -> None:
        """Counts a main-level ensure when a test invokes main."""
        source = """
fn main() -> anyhow::Result<()> {
    anyhow::ensure!(true, "condition failed");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        let _ = super::main();
    }
}
"""
        with TemporaryDirectory() as temp_dir:
            example = Path(temp_dir) / "example.rs"
            example.write_text(source, encoding="utf-8")

            self.assertTrue(classify(example)[-1])

    def test_ignored_external_helper_test_is_accepted(self) -> None:
        """Accepts an ignored service test that exercises a helper."""
        source = """
fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "needs a reachable service"]
    fn service_test() {
        helper();
    }
}
"""
        with TemporaryDirectory() as temp_dir:
            example = Path(temp_dir) / "example.rs"
            example.write_text(source, encoding="utf-8")

            classified = classify(example)
            self.assertTrue(classified[3])
            self.assertTrue(classified[4])


if __name__ == "__main__":
    unittest.main()
