# PyO3 Support for Quantity<U>

## Overview

`Quantity<U>` types support PyO3 integration through conversion traits and arithmetic operator support. You can use typed `Quantity<U>` fields directly in `#[pyclass]` structs while maintaining both Rust type safety and Python usability.

## Quick Start

### 1. Enable the PyO3 Feature

Add to your `Cargo.toml`:

```toml
[dependencies]
qtty-core = { version = "0.2", features = ["pyo3"] }
pyo3 = { version = "0.22", features = ["extension-module"] }
```

### 2. Define Your Struct with Quantity Fields

```rust
use pyo3::prelude::*;
use qtty_core::length::Kilometer;
use qtty_core::Quantity;

#[pyclass]
pub struct MyObject {
    distance: Quantity<Kilometer>,
}

#[pymethods]
impl MyObject {
    #[new]
    fn new(distance: f64) -> Self {
        Self {
            distance: Quantity::new(distance),
        }
    }

    #[getter]
    fn distance(&self) -> f64 {
        self.distance.value()
    }

    #[setter]
    fn set_distance(&mut self, value: f64) {
        self.distance = Quantity::new(value);
    }

    /// Add two objects (same unit)
    fn __add__(&self, other: &MyObject) -> MyObject {
        MyObject {
            distance: self.distance + other.distance,
        }
    }

    /// Multiply by scalar
    fn __mul__(&self, scalar: f64) -> MyObject {
        MyObject {
            distance: self.distance * scalar,
        }
    }

    fn describe(&self) -> String {
        format!("Distance: {} km", self.distance.value())
    }
}
```

### 3. Use from Python

```python
from mymodule import MyObject

obj1 = MyObject(10.0)
obj2 = MyObject(5.0)

# Arithmetic operations
result = obj1 + obj2       # Distance: 15.0 km
scaled = obj1 * 2.0        # Distance: 20.0 km
description = obj1.describe()
```

## How It Works

### Type Safety Bridge

Your struct uses **compile-time typed `Quantity<U>`** for Rust operations:
```rust
// In Rust: full type safety
let d: Quantity<Kilometer> = obj.distance;
// Compile error if you try to add Meters:
// let x = d + meter_quantity;  // ❌ Type error
```

At the Python boundary, conversions happen through our `IntoPy`/`FromPyObject` implementations:
- **Rust → Python**: `Quantity<U>` → `float` (via `.value()`)
- **Python → Rust**: `float` → `Quantity<U>`

### Arithmetic Operations

Implement PyO3 dunder methods to delegate to `Quantity<U>` operations:

```rust
fn __add__(&self, other: &MyObject) -> MyResult<MyObject> {
    Ok(MyObject {
        distance: self.distance + other.distance,  // Uses Quantity<U> arithmetic
    })
}
```

This keeps all the Rust type safety while enabling natural Python syntax.

### Runtime Unit Conversions

For converting between units at runtime, use qtty-ffi's `registry`:

```rust
use qtty_ffi::{registry, UnitId};

fn in_meters(&self) -> PyResult<f64> {
    registry::convert_value(
        self.distance.value(),
        UnitId::Kilometer,
        UnitId::Meter
    ).map_err(|_| PyTypeError::new_err("Incompatible units"))
}
```

Python usage:
```python
>>> obj = MyObject(1.0)  # 1 km
>>> obj.in_meters()
1000.0
```

## Key Advantages

✅ **Compile-time type safety in Rust** - catch unit errors at compile time
✅ **Natural Python syntax** - arithmetic with `+`, `*`, etc.
✅ **Minimal boilerplate** - inherit Rust's arithmetic operators
✅ **Runtime conversion** - support unit conversions via qtty-ffi registry
✅ **Clear abstraction** - Python sees floats, Rust sees typed Quantities

## Example: Complete Struct

```rust
#[pyclass]
pub struct Vector {
    x: Quantity<Meter>,
    y: Quantity<Meter>,
}

#[pymethods]
impl Vector {
    #[new]
    fn new(x: f64, y: f64) -> Self {
        Self {
            x: Quantity::new(x),
            y: Quantity::new(y),
        }
    }

    fn magnitude(&self) -> f64 {
        (self.x.value().powi(2) + self.y.value().powi(2)).sqrt()
    }

    fn __add__(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn scale(&self, factor: f64) -> Vector {
        Vector {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}
```

Python:
```python
v1 = Vector(3.0, 4.0)
v2 = Vector(1.0, 2.0)
v3 = v1 + v2
magnitude = v3.magnitude()  # ≈ 7.28
scaled = v1.scale(2.0)
```

## Limitations & Considerations

1. **Same-unit arithmetic only** - Operations between different unit types will require explicit conversion
2. **No Python-level unit type** - Python only sees floats; unit information is implicit in method names
3. **Conversion at boundaries** - All Python↔Rust conversions go through `f64`

## Testing

Run the example:
```bash
cargo run --example pyo3_struct --features pyo3
```
