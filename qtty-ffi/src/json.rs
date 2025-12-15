//! JSON serialization FFI for qtty-ffi.
//!
//! This module exposes `extern "C"` functions for serializing and deserializing
//! quantities to/from JSON. When the `serde` feature is enabled, the functions
//! operate using `serde_json`. When disabled, stub implementations preserve the
//! ABI and return error codes indicating the functionality is unavailable.

use crate::{QttyQuantity, UnitId, QTTY_ERR_INVALID_VALUE, QTTY_ERR_NULL_OUT, QTTY_ERR_UNKNOWN_UNIT, QTTY_OK};

/// Frees a C string allocated by qtty-ffi JSON functions.
///
/// # Safety
/// Pointer must have been allocated by qtty-ffi JSON APIs.
#[no_mangle]
#[cfg(feature = "serde")]
pub unsafe extern "C" fn qtty_string_free(ptr: *mut core::ffi::c_char) {
    if !ptr.is_null() {
        let _ = unsafe { std::ffi::CString::from_raw(ptr) };
    }
}

#[cfg(feature = "serde")]
fn make_cstring(s: String) -> *mut core::ffi::c_char {
    std::ffi::CString::new(s)
        .map(|cs| cs.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

/// Serializes a quantity's numeric value as a JSON number string.
#[no_mangle]
#[cfg(feature = "serde")]
pub unsafe extern "C" fn qtty_quantity_to_json_value(
    src: QttyQuantity,
    out_json: *mut *mut core::ffi::c_char,
) -> i32 {
    if out_json.is_null() {
        return QTTY_ERR_NULL_OUT;
    }
    let json = serde_json::to_string(&src.value).unwrap_or_else(|_| "null".to_string());
    unsafe { *out_json = make_cstring(json) };
    QTTY_OK
}

/// Deserializes a quantity's numeric value from a JSON number string and sets the unit.
#[no_mangle]
#[cfg(feature = "serde")]
pub unsafe extern "C" fn qtty_quantity_from_json_value(
    unit: UnitId,
    json: *const core::ffi::c_char,
    out: *mut QttyQuantity,
) -> i32 {
    if out.is_null() {
        return QTTY_ERR_NULL_OUT;
    }
    if registry::meta(unit).is_none() {
        return QTTY_ERR_UNKNOWN_UNIT;
    }
    if json.is_null() {
        return QTTY_ERR_INVALID_VALUE;
    }
    let cstr = unsafe { std::ffi::CStr::from_ptr(json) };
    let s = match cstr.to_str() {
        Ok(s) => s,
        Err(_) => return QTTY_ERR_INVALID_VALUE,
    };
    match serde_json::from_str::<f64>(s) {
        Ok(v) => {
            unsafe { *out = QttyQuantity::new(v, unit) };
            QTTY_OK
        }
        Err(_) => QTTY_ERR_INVALID_VALUE,
    }
}

/// Serializes a quantity as a JSON object: {"value": f64, "unit_id": u32}.
#[no_mangle]
#[cfg(feature = "serde")]
pub unsafe extern "C" fn qtty_quantity_to_json(
    src: QttyQuantity,
    out_json: *mut *mut core::ffi::c_char,
) -> i32 {
    if out_json.is_null() {
        return QTTY_ERR_NULL_OUT;
    }
    let obj = serde_json::json!({
        "value": src.value,
        "unit_id": src.unit as u32
    });
    let json = serde_json::to_string(&obj).unwrap_or_else(|_| "{}".to_string());
    unsafe { *out_json = make_cstring(json) };
    QTTY_OK
}

/// Deserializes a quantity from a JSON object: {"value": f64, "unit_id": u32}.
#[no_mangle]
#[cfg(feature = "serde")]
pub unsafe extern "C" fn qtty_quantity_from_json(
    json: *const core::ffi::c_char,
    out: *mut QttyQuantity,
) -> i32 {
    if out.is_null() {
        return QTTY_ERR_NULL_OUT;
    }
    if json.is_null() {
        return QTTY_ERR_INVALID_VALUE;
    }
    let cstr = unsafe { std::ffi::CStr::from_ptr(json) };
    let s = match cstr.to_str() {
        Ok(s) => s,
        Err(_) => return QTTY_ERR_INVALID_VALUE,
    };
    let v: serde_json::Value = match serde_json::from_str(s) {
        Ok(v) => v,
        Err(_) => return QTTY_ERR_INVALID_VALUE,
    };
    let value = v.get("value").and_then(|x| x.as_f64());
    let unit_id_u32 = v.get("unit_id").and_then(|x| x.as_u64());
    match (value, unit_id_u32.and_then(|u| u.try_into().ok()).and_then(UnitId::from_u32)) {
        (Some(val), Some(unit)) => {
            if registry::meta(unit).is_none() {
                return QTTY_ERR_UNKNOWN_UNIT;
            }
            unsafe { *out = QttyQuantity::new(val, unit) };
            QTTY_OK
        }
        (Some(_), None) => QTTY_ERR_UNKNOWN_UNIT,
        _ => QTTY_ERR_INVALID_VALUE,
    }
}

// ==============================
// No-serde stubs for ABI stability
// ==============================

/// No-op free when `serde` feature is disabled.
///
/// When built without the `serde` feature, JSON functions are unavailable and no
/// strings are allocated by the library. This function is provided for ABI
/// stability and performs no action.
#[no_mangle]
#[cfg(not(feature = "serde"))]
pub unsafe extern "C" fn qtty_string_free(_ptr: *mut core::ffi::c_char) {
    let _ = _ptr; // suppress unused warnings under strict lints
}

/// Stub: serializes a quantity value to JSON number string.
///
/// When `serde` is disabled, this function sets `*out_json` to null (if not
/// null) and returns `QTTY_ERR_INVALID_VALUE` to indicate the operation is
/// unsupported.
#[no_mangle]
#[cfg(not(feature = "serde"))]
pub unsafe extern "C" fn qtty_quantity_to_json_value(
    _src: QttyQuantity,
    out_json: *mut *mut core::ffi::c_char,
) -> i32 {
    if out_json.is_null() {
        return QTTY_ERR_NULL_OUT;
    }
    unsafe { *out_json = core::ptr::null_mut() };
    QTTY_ERR_INVALID_VALUE
}

/// Stub: deserializes a quantity value from JSON number string.
///
/// When `serde` is disabled, this function returns `QTTY_ERR_INVALID_VALUE` to
/// indicate the operation is unsupported.
#[no_mangle]
#[cfg(not(feature = "serde"))]
pub unsafe extern "C" fn qtty_quantity_from_json_value(
    _unit: UnitId,
    _json: *const core::ffi::c_char,
    _out: *mut QttyQuantity,
) -> i32 {
    if _out.is_null() {
        return QTTY_ERR_NULL_OUT;
    }
    QTTY_ERR_INVALID_VALUE
}

/// Stub: serializes a quantity to JSON object {"value": f64, "unit_id": u32}.
///
/// When `serde` is disabled, this function sets `*out_json` to null (if not
/// null) and returns `QTTY_ERR_INVALID_VALUE` to indicate the operation is
/// unsupported.
#[no_mangle]
#[cfg(not(feature = "serde"))]
pub unsafe extern "C" fn qtty_quantity_to_json(
    _src: QttyQuantity,
    out_json: *mut *mut core::ffi::c_char,
) -> i32 {
    if out_json.is_null() {
        return QTTY_ERR_NULL_OUT;
    }
    unsafe { *out_json = core::ptr::null_mut() };
    QTTY_ERR_INVALID_VALUE
}

/// Stub: deserializes a quantity from JSON object {"value": f64, "unit_id": u32}.
///
/// When `serde` is disabled, this function returns `QTTY_ERR_INVALID_VALUE` to
/// indicate the operation is unsupported.
#[no_mangle]
#[cfg(not(feature = "serde"))]
pub unsafe extern "C" fn qtty_quantity_from_json(
    _json: *const core::ffi::c_char,
    _out: *mut QttyQuantity,
) -> i32 {
    if _out.is_null() {
        return QTTY_ERR_NULL_OUT;
    }
    QTTY_ERR_INVALID_VALUE
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    #[test]
    fn value_only_round_trip() {
        let src = QttyQuantity::new(123.456, UnitId::Meter);
        let mut json_ptr: *mut core::ffi::c_char = core::ptr::null_mut();
        let ok = unsafe { qtty_quantity_to_json_value(src, &mut json_ptr) };
        assert_eq!(ok, QTTY_OK);
        assert!(!json_ptr.is_null());
        let s = unsafe { std::ffi::CStr::from_ptr(json_ptr) }.to_str().unwrap().to_string();
        unsafe { qtty_string_free(json_ptr) };

        let mut out = QttyQuantity::new(0.0, UnitId::Meter);
        let ok = unsafe { qtty_quantity_from_json_value(UnitId::Meter, std::ffi::CString::new(s).unwrap().as_ptr(), &mut out) };
        assert_eq!(ok, QTTY_OK);
        assert!((out.value - 123.456).abs() < 1e-12);
        assert_eq!(out.unit, UnitId::Meter);
    }

    #[test]
    fn object_round_trip_with_unit_id() {
        let src = QttyQuantity::new(2.0, UnitId::Kilometer);
        let mut json_ptr: *mut core::ffi::c_char = core::ptr::null_mut();
        let ok = unsafe { qtty_quantity_to_json(src, &mut json_ptr) };
        assert_eq!(ok, QTTY_OK);
        let s = unsafe { std::ffi::CStr::from_ptr(json_ptr) }.to_str().unwrap().to_string();
        unsafe { qtty_string_free(json_ptr) };

        let mut out = QttyQuantity::new(0.0, UnitId::Meter);
        let ok = unsafe { qtty_quantity_from_json(std::ffi::CString::new(s).unwrap().as_ptr(), &mut out) };
        assert_eq!(ok, QTTY_OK);
        assert_eq!(out.unit, UnitId::Kilometer);
        assert!((out.value - 2.0).abs() < 1e-12);
    }
}
