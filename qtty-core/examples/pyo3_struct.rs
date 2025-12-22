//! Example: #[pyclass] struct with Quantity<U> and arithmetic operations
//!
//! Demonstrates keeping compile-time type safety in Rust while exposing
//! arithmetic operations in Python via runtime unit representation.
//!
//! Run with:
//! cargo run --example pyo3_struct --features pyo3

#[cfg(feature = "pyo3")]
fn main() {
    use pyo3::prelude::*;
    use qtty_core::length::Kilometer;
    use qtty_core::Quantity;
    use qtty_ffi::registry;

    /// A struct that uses typed Quantity<U> for Rust type safety,
    /// while supporting Python arithmetic via runtime conversions.
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

        /// Get distance value (in the struct's unit: Kilometer)
        #[getter]
        fn distance(&self) -> f64 {
            self.distance.value()
        }

        #[setter]
        fn set_distance(&mut self, value: f64) {
            self.distance = Quantity::new(value);
        }

        /// Convert to another unit at runtime.
        /// Usage: obj.in_meters() returns the distance in meters
        fn in_meters(&self) -> PyResult<f64> {
            use qtty_ffi::UnitId;
            registry::convert_value(self.distance.value(), UnitId::Kilometer, UnitId::Meter)
                .map_err(|_| pyo3::exceptions::PyTypeError::new_err(
                    "Cannot convert Kilometer to Meter"
                ))
        }

        /// Add two MyObject structs (same unit)
        fn __add__(&self, other: &MyObject) -> PyResult<MyObject> {
            Ok(MyObject {
                distance: self.distance + other.distance,
            })
        }

        /// Subtract two MyObject structs (same unit)
        fn __sub__(&self, other: &MyObject) -> PyResult<MyObject> {
            Ok(MyObject {
                distance: self.distance - other.distance,
            })
        }

        /// Multiply by scalar
        fn __mul__(&self, scalar: f64) -> PyResult<MyObject> {
            Ok(MyObject {
                distance: self.distance * scalar,
            })
        }

        /// Right multiply (scalar * obj)
        fn __rmul__(&self, scalar: f64) -> PyResult<MyObject> {
            self.__mul__(scalar)
        }

        /// Divide by scalar
        fn __truediv__(&self, scalar: f64) -> PyResult<MyObject> {
            if scalar == 0.0 {
                return Err(pyo3::exceptions::PyZeroDivisionError::new_err(
                    "Cannot divide by zero"
                ));
            }
            Ok(MyObject {
                distance: self.distance / scalar,
            })
        }

        fn __repr__(&self) -> String {
            format!("MyObject(distance={}km)", self.distance.value())
        }

        fn describe(&self) -> String {
            format!("Distance: {} km", self.distance.value())
        }
    }

    println!(
        "PyO3 example with typed Quantity<U> and arithmetic:\n\n\
         Usage from Python:\n\
         >>> from example import MyObject\n\
         >>> obj1 = MyObject(10.0)\n\
         >>> obj2 = MyObject(5.0)\n\
         >>> (obj1 + obj2).distance\n\
         15.0\n\
         >>> (obj1 - obj2).distance\n\
         5.0\n\
         >>> (obj1 * 2.0).distance\n\
         20.0\n\
         >>> obj1.in_meters()  # Runtime conversion\n\
         10000.0\n\
         >>> obj1.describe()\n\
         'Distance: 10.0 km'"
    );
}

#[cfg(not(feature = "pyo3"))]
fn main() {
    println!(
        "This example requires the 'pyo3' feature.\n\
         Run with: cargo run --example pyo3_struct --features pyo3"
    );
}
