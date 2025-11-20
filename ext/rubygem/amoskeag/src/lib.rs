use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::collections::HashMap;
use std::panic;
use amoskeag::{compile, evaluate, CompiledProgram};
use amoskeag_stdlib_operators::Value;

// Maximum allowed sizes for defensive programming
const MAX_SOURCE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const MAX_JSON_SIZE: usize = 100 * 1024 * 1024; // 100MB
const MAX_SYMBOLS_COUNT: usize = 10_000;
const MAX_DICT_DEPTH: usize = 100;

// Opaque pointer types for Ruby FFI
pub struct AmoskeagProgram {
    program: CompiledProgram,
}

// Not currently used but may be useful for future error handling
#[allow(dead_code)]
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
// Not currently used but kept for potential future direct JSON parsing needs
#[allow(dead_code)]
fn json_to_value(json_str: &str) -> Result<Value, String> {
    let json_value: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    json_value_to_value(&json_value)
}

fn json_value_to_value(json: &serde_json::Value) -> Result<Value, String> {
    json_value_to_value_with_depth(json, 0)
}

fn json_value_to_value_with_depth(json: &serde_json::Value, depth: usize) -> Result<Value, String> {
    // Defensive: Prevent stack overflow from deeply nested structures
    if depth > MAX_DICT_DEPTH {
        return Err(format!("JSON structure too deeply nested (max depth: {})", MAX_DICT_DEPTH));
    }

    match json {
        serde_json::Value::Null => Ok(Value::Nil),
        serde_json::Value::Bool(b) => Ok(Value::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                // Defensive: Check for special float values
                if !f.is_finite() {
                    return Err(format!("Invalid number: {} (must be finite)", f));
                }
                Ok(Value::Number(f))
            } else {
                Err("Number conversion error".to_string())
            }
        }
        serde_json::Value::String(s) => {
            // Defensive: Check string length
            if s.len() > MAX_JSON_SIZE {
                return Err(format!("String too large: {} bytes (max: {})", s.len(), MAX_JSON_SIZE));
            }
            Ok(Value::String(s.clone()))
        }
        serde_json::Value::Array(arr) => {
            // Defensive: Check array size
            if arr.len() > 1_000_000 {
                return Err(format!("Array too large: {} elements (max: 1,000,000)", arr.len()));
            }

            let values: Result<Vec<Value>, String> = arr.iter()
                .map(|v| json_value_to_value_with_depth(v, depth + 1))
                .collect();
            Ok(Value::Array(values?))
        }
        serde_json::Value::Object(obj) => {
            // Check if this is a symbol
            if obj.len() == 1 && obj.contains_key("__symbol__") {
                if let Some(serde_json::Value::String(s)) = obj.get("__symbol__") {
                    // Defensive: Validate symbol name
                    if s.is_empty() {
                        return Err("Symbol name cannot be empty".to_string());
                    }
                    if s.len() > 1000 {
                        return Err(format!("Symbol name too long: {} bytes (max: 1000)", s.len()));
                    }
                    return Ok(Value::Symbol(s.clone()));
                }
            }

            // Defensive: Check object size
            if obj.len() > 100_000 {
                return Err(format!("Object too large: {} keys (max: 100,000)", obj.len()));
            }

            let mut dict = HashMap::new();
            for (k, v) in obj.iter() {
                // Defensive: Validate key length
                if k.len() > 1000 {
                    return Err(format!("Object key too long: {} bytes (max: 1000)", k.len()));
                }
                dict.insert(k.clone(), json_value_to_value_with_depth(v, depth + 1)?);
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
    // Defensive: Catch panics to prevent unwinding into C/Ruby
    let result = panic::catch_unwind(|| {
        // Defensive: Validate source pointer
        if source.is_null() {
            return Err("source pointer is null".to_string());
        }

        let source_str = unsafe {
            match CStr::from_ptr(source).to_str() {
                Ok(s) => s,
                Err(_) => {
                    return Err("Invalid UTF-8 in source".to_string());
                }
            }
        };

        // Defensive: Check source size
        if source_str.is_empty() {
            return Err("source is empty".to_string());
        }
        if source_str.len() > MAX_SOURCE_SIZE {
            return Err(format!(
                "source too large: {} bytes (max: {} bytes)",
                source_str.len(),
                MAX_SOURCE_SIZE
            ));
        }

        let symbols: Vec<String> = if symbols_json.is_null() {
            Vec::new()
        } else {
            let symbols_str = unsafe {
                match CStr::from_ptr(symbols_json).to_str() {
                    Ok(s) => s,
                    Err(_) => {
                        return Err("Invalid UTF-8 in symbols".to_string());
                    }
                }
            };

            // Defensive: Check symbols JSON size
            if symbols_str.len() > MAX_JSON_SIZE {
                return Err(format!(
                    "symbols JSON too large: {} bytes (max: {} bytes)",
                    symbols_str.len(),
                    MAX_JSON_SIZE
                ));
            }

            match serde_json::from_str::<Vec<String>>(symbols_str) {
                Ok(vec) => {
                    // Defensive: Check symbols count
                    if vec.len() > MAX_SYMBOLS_COUNT {
                        return Err(format!(
                            "Too many symbols: {} (max: {})",
                            vec.len(),
                            MAX_SYMBOLS_COUNT
                        ));
                    }

                    // Defensive: Validate each symbol
                    for sym in &vec {
                        if sym.is_empty() {
                            return Err("Symbol cannot be empty".to_string());
                        }
                        if sym.len() > 1000 {
                            return Err(format!(
                                "Symbol too long: {} bytes (max: 1000)",
                                sym.len()
                            ));
                        }
                    }

                    vec
                }
                Err(e) => {
                    return Err(format!("Invalid symbols JSON: {}", e));
                }
            }
        };

        let symbols_refs: Vec<&str> = symbols.iter().map(|s| s.as_str()).collect();

        match compile(source_str, &symbols_refs) {
            Ok(program) => {
                let boxed = Box::new(AmoskeagProgram { program });
                Ok(Box::into_raw(boxed))
            }
            Err(e) => {
                Err(format!("{:?}", e))
            }
        }
    });

    // Handle the result of panic::catch_unwind
    match result {
        Ok(Ok(program_ptr)) => program_ptr,
        Ok(Err(error_msg)) => {
            if !error_out.is_null() {
                if let Ok(err_cstring) = CString::new(error_msg) {
                    unsafe { *error_out = err_cstring.into_raw(); }
                }
            }
            ptr::null_mut()
        }
        Err(_panic) => {
            if !error_out.is_null() {
                if let Ok(err_cstring) = CString::new("Panic occurred during compilation") {
                    unsafe { *error_out = err_cstring.into_raw(); }
                }
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
    // Defensive: Catch panics to prevent unwinding into C/Ruby
    let result = panic::catch_unwind(|| {
        // Defensive: Validate pointers
        if program.is_null() {
            return Err("program pointer is null".to_string());
        }
        if data_json.is_null() {
            return Err("data_json pointer is null".to_string());
        }

        let program_ref = unsafe { &*program };

        let data_str = unsafe {
            match CStr::from_ptr(data_json).to_str() {
                Ok(s) => s,
                Err(_) => {
                    return Err("Invalid UTF-8 in data".to_string());
                }
            }
        };

        // Defensive: Check data JSON size
        if data_str.len() > MAX_JSON_SIZE {
            return Err(format!(
                "data JSON too large: {} bytes (max: {} bytes)",
                data_str.len(),
                MAX_JSON_SIZE
            ));
        }

        // Parse JSON data
        let json_data: serde_json::Value = match serde_json::from_str(data_str) {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("Invalid data JSON: {}", e));
            }
        };

        // Convert to HashMap<String, Value>
        let mut data = HashMap::new();
        if let serde_json::Value::Object(obj) = json_data {
            // Defensive: Check data object size
            if obj.len() > 100_000 {
                return Err(format!("Data object too large: {} keys (max: 100,000)", obj.len()));
            }

            for (k, v) in obj.iter() {
                match json_value_to_value(v) {
                    Ok(value) => { data.insert(k.clone(), value); }
                    Err(e) => {
                        return Err(format!("Data conversion error: {}", e));
                    }
                }
            }
        } else {
            return Err("Data must be a JSON object".to_string());
        }

        match evaluate(&program_ref.program, &data) {
            Ok(result) => {
                match value_to_json(&result) {
                    Ok(json) => {
                        // Defensive: Check result size
                        if json.len() > MAX_JSON_SIZE {
                            return Err(format!(
                                "Result JSON too large: {} bytes (max: {} bytes)",
                                json.len(),
                                MAX_JSON_SIZE
                            ));
                        }

                        match CString::new(json) {
                            Ok(c_str) => Ok(c_str.into_raw()),
                            Err(_) => Err("Result contains null bytes".to_string()),
                        }
                    }
                    Err(e) => {
                        Err(format!("Result conversion error: {}", e))
                    }
                }
            }
            Err(e) => {
                Err(format!("{:?}", e))
            }
        }
    });

    // Handle the result of panic::catch_unwind
    match result {
        Ok(Ok(result_ptr)) => result_ptr,
        Ok(Err(error_msg)) => {
            if !error_out.is_null() {
                if let Ok(err_cstring) = CString::new(error_msg) {
                    unsafe { *error_out = err_cstring.into_raw(); }
                }
            }
            ptr::null_mut()
        }
        Err(_panic) => {
            if !error_out.is_null() {
                if let Ok(err_cstring) = CString::new("Panic occurred during evaluation") {
                    unsafe { *error_out = err_cstring.into_raw(); }
                }
            }
            ptr::null_mut()
        }
    }
}

/// Free a compiled program
/// Defensive: Safe to call with null pointer, safe to call multiple times (though not recommended)
#[no_mangle]
pub extern "C" fn amoskeag_program_free(program: *mut AmoskeagProgram) {
    // Defensive: Catch panics during deallocation
    let _ = panic::catch_unwind(|| {
        if !program.is_null() {
            unsafe {
                // Convert back to Box and drop
                let _ = Box::from_raw(program);
            }
        }
    });
}

/// Free a string returned by the library
/// Defensive: Safe to call with null pointer, safe to call multiple times (though not recommended)
#[no_mangle]
pub extern "C" fn amoskeag_string_free(s: *mut c_char) {
    // Defensive: Catch panics during deallocation
    let _ = panic::catch_unwind(|| {
        if !s.is_null() {
            unsafe {
                // Convert back to CString and drop
                let _ = CString::from_raw(s);
            }
        }
    });
}
