# TODO Reviewed ??-??-2026.

# Purpose: Regenerates the tiny ONNX test fixtures.
# Usage: Run with python bk/crates/other/machine_learning/examples/machine_learning/models/make_models.py and provide any required arguments.
#
"""Regenerates the tiny ONNX test fixtures.

Requires numpy, scikit-learn, and skl2onnx in a version-pinned Python environment.
Run this script from any working directory; it writes beside the script.
The committed fixtures are sufficient for Rust tests; regeneration is optional.
"""
from pathlib import Path

import numpy as np
from skl2onnx import to_onnx
from sklearn.linear_model import LogisticRegression

MODELS_DIR = Path(__file__).resolve().parent

X = np.array([[-2, -2], [-1, -1], [-1, 0], [0, -1], [1, 1], [2, 2], [1, 0], [0, 1]], dtype=np.float32)
y = np.array([0, 0, 0, 0, 1, 1, 1, 1])

for name, labels in (("logreg_a.onnx", y), ("logreg_b.onnx", 1 - y)):
    model = LogisticRegression().fit(X, labels)
    onx = to_onnx(model, X[:1], options={id(model): {"zipmap": False}}, target_opset=13)
    with (MODELS_DIR / name).open("wb") as f:
        f.write(onx.SerializeToString())
    print(name, model.predict_proba(np.array([[1.5, 1.5], [-1.5, -1.5]], dtype=np.float32)))
