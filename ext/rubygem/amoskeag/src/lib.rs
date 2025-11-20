use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::collections::HashMap;
use amoskeag::{compile, evaluate, CompiledProgram};
use amoskeag_stdlib_operators::Value;

// Opaque pointer types for Ruby FFI
pub struct AmoskeagProgram {
    program: CompiledProgram,
}

pub struct AmoskeagError {
    message: String,
}

// Convert Value to JSON string for simplified marshalling
fn value_to_json(value: &Value) -> Result<String, String> {
    match value {
        Value::Number(n) => Ok(serde_json::to_string(n).unwrap()),
        Value::String(s) => Ok(serde_json::to_string(s).unwrap()),
        Value::Boolean(b) => Ok(serde_json::to_string(b).unwrap()),
        Value::Nil => Ok("null".to_string()),
        Value::Array(arr) => {
            let json_arr: Result<Vec<String>, String> = arr.iter()
                .map(value_to_json)
                .collect();
            let json_arr = json_arr?;
            Ok(format!("[{}]", json_arr.join(",")))
        }
        Value::Dictionary(dict) => {
            let mut items = Vec::new();
            for (k, v) in dict.iter() {
                let v_json = value_to_json(v)?;
                items.push(format!("\"{}\":{}", k, v_json));
            }
            Ok(format!("{{{}}}", items.join(",")))
        }
        Value::Symbol(s) => Ok(format!("{{\"__symbol__\":\"{}\"}}", s)),
    }
}

// Convert JSON string to Value
fn json_to_value(json_str: &str) -> Result<Value, String> {
    let json_value: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    json_value_to_value(&json_value)
}

fn json_value_to_value(json: &serde_json::Value) -> Result<Value, String> {
    match json {
        serde_json::Value::Null => Ok(Value::Nil),
        serde_json::Value::Bool(b) => Ok(Value::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                Ok(Value::Number(f))
            } else {
                Err("Number conversion error".to_string())
            }
        }
        serde_json::Value::String(s) => Ok(Value::String(s.clone())),
        serde_json::Value::Array(arr) => {
            let values: Result<Vec<Value>, String> = arr.iter()
                .map(json_value_to_value)
                .collect();
            Ok(Value::Array(values?))
        }
        serde_json::Value::Object(obj) => {
            // Check if this is a symbol
            if obj.len() == 1 && obj.contains_key("__symbol__") {
                if let Some(serde_json::Value::String(s)) = obj.get("__symbol__") {
                    return Ok(Value::Symbol(s.clone()));
                }
            }

            let mut dict = HashMap::new();
            for (k, v) in obj.iter() {
                dict.insert(k.clone(), json_value_to_value(v)?);
            }
            Ok(Value::Dictionary(dict))
        }
    }
}

/// Compile an Amoskeag program with optional symbols
/// Returns a pointer to AmoskeagProgram on success, null on error
/// On error, error_out will be set to the error message
#[no_mangle]
pub extern "C" fn amoskeag_compile(
    source: *const c_char,
    symbols_json: *const c_char,
    error_out: *mut *mut c_char,
) -> *mut AmoskeagProgram {
    if source.is_null() {
        return ptr::null_mut();
    }

    let source_str = unsafe {
        match CStr::from_ptr(source).to_str() {
            Ok(s) => s,
            Err(_) => {
                if !error_out.is_null() {
                    let err_msg = CString::new("Invalid UTF-8 in source").unwrap();
                    unsafe { *error_out = err_msg.into_raw(); }
                }
                return ptr::null_mut();
            }
        }
    };

    let symbols: Vec<&str> = if symbols_json.is_null() {
        Vec::new()
    } else {
        let symbols_str = unsafe {
            match CStr::from_ptr(symbols_json).to_str() {
                Ok(s) => s,
                Err(_) => {
                    if !error_out.is_null() {
                        let err_msg = CString::new("Invalid UTF-8 in symbols").unwrap();
                        unsafe { *error_out = err_msg.into_raw(); }
                    }
                    return ptr::null_mut();
                }
            }
        };

        match serde_json::from_str::<Vec<String>>(symbols_str) {
            Ok(vec) => vec.iter().map(|s| s.as_str()).collect(),
            Err(e) => {
                if !error_out.is_null() {
                    let err_msg = CString::new(format!("Invalid symbols JSON: {}", e)).unwrap();
                    unsafe { *error_out = err_msg.into_raw(); }
                }
                return ptr::null_mut();
            }
        }
    };

    match compile(source_str, &symbols) {
        Ok(program) => {
            let boxed = Box::new(AmoskeagProgram { program });
            Box::into_raw(boxed)
        }
        Err(e) => {
            if !error_out.is_null() {
                let err_msg = CString::new(format!("{:?}", e)).unwrap();
                unsafe { *error_out = err_msg.into_raw(); }
            }
            ptr::null_mut()
        }
    }
}

/// Evaluate a compiled program with data
/// Returns JSON string representation of the result
/// On error, returns null and sets error_out
#[no_mangle]
pub extern "C" fn amoskeag_evaluate(
    program: *const AmoskeagProgram,
    data_json: *const c_char,
    error_out: *mut *mut c_char,
) -> *mut c_char {
    if program.is_null() || data_json.is_null() {
        return ptr::null_mut();
    }

    let program = unsafe { &*program };

    let data_str = unsafe {
        match CStr::from_ptr(data_json).to_str() {
            Ok(s) => s,
            Err(_) => {
                if !error_out.is_null() {
                    let err_msg = CString::new("Invalid UTF-8 in data").unwrap();
                    unsafe { *error_out = err_msg.into_raw(); }
                }
                return ptr::null_mut();
            }
        }
    };

    // Parse JSON data
    let json_data: serde_json::Value = match serde_json::from_str(data_str) {
        Ok(v) => v,
        Err(e) => {
            if !error_out.is_null() {
                let err_msg = CString::new(format!("Invalid data JSON: {}", e)).unwrap();
                unsafe { *error_out = err_msg.into_raw(); }
            }
            return ptr::null_mut();
        }
    };

    // Convert to HashMap<String, Value>
    let mut data = HashMap::new();
    if let serde_json::Value::Object(obj) = json_data {
        for (k, v) in obj.iter() {
            match json_value_to_value(v) {
                Ok(value) => { data.insert(k.clone(), value); }
                Err(e) => {
                    if !error_out.is_null() {
                        let err_msg = CString::new(format!("Data conversion error: {}", e)).unwrap();
                        unsafe { *error_out = err_msg.into_raw(); }
                    }
                    return ptr::null_mut();
                }
            }
        }
    }

    match evaluate(&program.program, &data) {
        Ok(result) => {
            match value_to_json(&result) {
                Ok(json) => {
                    match CString::new(json) {
                        Ok(c_str) => c_str.into_raw(),
                        Err(_) => ptr::null_mut(),
                    }
                }
                Err(e) => {
                    if !error_out.is_null() {
                        let err_msg = CString::new(format!("Result conversion error: {}", e)).unwrap();
                        unsafe { *error_out = err_msg.into_raw(); }
                    }
                    ptr::null_mut()
                }
            }
        }
        Err(e) => {
            if !error_out.is_null() {
                let err_msg = CString::new(format!("{:?}", e)).unwrap();
                unsafe { *error_out = err_msg.into_raw(); }
            }
            ptr::null_mut()
        }
    }
}

/// Free a compiled program
#[no_mangle]
pub extern "C" fn amoskeag_program_free(program: *mut AmoskeagProgram) {
    if !program.is_null() {
        unsafe {
            let _ = Box::from_raw(program);
        }
    }
}

/// Free a string returned by the library
#[no_mangle]
pub extern "C" fn amoskeag_string_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
