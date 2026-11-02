"""Regression tests for the dependency-audit workflow trigger check."""

import unittest

from scripts.check_audit_workflow_paths import WORKFLOW, missing, trigger_paths


class AuditWorkflowPathTests(unittest.TestCase):
    """Checks the committed workflow and the checker itself."""

    def test_committed_workflow_covers_every_workspace(self) -> None:
        self.assertEqual(missing(trigger_paths(WORKFLOW.read_text())), [])

    def test_dropping_a_lockfile_is_reported(self) -> None:
        text = WORKFLOW.read_text().replace('      - "xmpl/Cargo.lock"\n', "")
        problems = missing(trigger_paths(text))
        self.assertIn("push: missing xmpl/Cargo.lock", problems)
        self.assertIn("pull_request: missing xmpl/Cargo.lock", problems)

    def test_dropping_a_nested_workspace_lockfile_is_reported(self) -> None:
        lockfile = (
            "later/crates/cats/wasm/examples/wasm_standalone_runtimes/components/guest/Cargo.lock"
        )
        text = WORKFLOW.read_text().replace(f'      - "{lockfile}"\n', "")
        problems = missing(trigger_paths(text))
        self.assertIn(f"push: missing {lockfile}", problems)
        self.assertIn(f"pull_request: missing {lockfile}", problems)


if __name__ == "__main__":
    unittest.main()
