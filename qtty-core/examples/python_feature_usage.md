# Using the `python` feature (PyO3) with qtty

This short example shows how to build the Python bindings and run the existing Python quickstart demo.

Notes:
- The `qtty-core` crate has an optional `python` feature which adds `#[cfg_attr(feature = "python", pyo3::pyclass)]` to concrete unit marker types.
- The practical Python-facing entrypoint is the `qtty-py` crate (a small extension crate that re-exports the FFI-backed Python API). Use the instructions below to build and run the Python examples.

Quick steps (recommended):

1) Create and activate a Python virtual environment:

```bash
python -m venv .venv
source .venv/bin/activate
```

2) Install build tooling (`maturin`) and other dev tools:

```bash
pip install --upgrade pip
pip install maturin pytest
```

3) Build & install the Python extension in the venv (from the workspace root):

```bash
cd /path/to/workspace/siderust
# Build and install qtty-py into the venv (this will build the Rust crates and the Python extension)
# Note: qtty-py depends on qtty-ffi which provides the Python module. qtty-core's internal `python` feature is optional
# and generally not required to use the qtty-py API. If you explicitly need `qtty-core` built with `python` feature, enable
# it by adding features to the appropriate crate in your local cargo manifest.
maturin develop -i python
```

4) Run the included quickstart example (already present at `qtty-py/examples/quickstart.py`):

```bash
python qtty-py/examples/quickstart.py
```

What this shows

- How to import and use the Python API:
  - `from qtty import Quantity, DerivedQuantity, Unit`
  - Create quantities with `Quantity(value, Unit.Whatever)` and convert with `.to(...)`
  - Use derived quantities (division) and conversions like `.to(Unit.Meter, Unit.Second)`

Advanced notes for crate authors

- If you want Rust `#[pyclass]` on `qtty-core` unit marker types to be *active* during a build of your own Python extension, you must ensure `qtty-core` is compiled with the `python` feature. One way to do that is to set the dependency feature in your `Cargo.toml` for the crate that builds the final Python extension.

Example (in your crate `Cargo.toml`):

```toml
[dependencies]
qtty-core = { path = "../qtty-core", features = ["python"] }
```

- After enabling the feature, you can register the unit types in your `#[pymodule]` by calling `m.add_class::<YourUnit>()?;` for each type you want exported.

- For most Python users, `qtty-py` (the extension crate) already exposes a complete Python API without requiring you to enable `qtty-core`'s internal `python` feature directly.

If you'd like, I can:
- add a minimal `examples/python_register.rs` that shows a `#[pymodule]` registering a couple of concrete unit types and a `Quantity<Meter>` wrapper, or
- add a short README snippet inside `qtty-py` showing exact `maturin` commands for common OSes.
