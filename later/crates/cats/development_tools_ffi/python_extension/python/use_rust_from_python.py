"""Import and call the Rust extension module built with Maturin."""

from rust_howto_python_module import add


result = add(10, 20)
if result != 30:
    raise RuntimeError(f"Expected 30, received {result}")

print(f"10 + 20 = {result}")
