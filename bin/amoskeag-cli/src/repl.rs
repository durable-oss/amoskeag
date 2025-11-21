//! Interactive REPL (Read-Eval-Print Loop)

use crate::backend::{evaluate_with_backend, BackendType};
use crate::format::format_value;
use amoskeag::{compile, AmoskeagValue as Value};
use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, Write};

/// Maximum input line length to prevent memory issues
const MAX_INPUT_LENGTH: usize = 10_000;

/// Run the interactive REPL
///
/// # Errors
/// Returns an error if I/O operations fail.
pub fn run_repl(backend_type: BackendType) -> Result<()> {
    println!(
        "Amoskeag REPL v{} (backend: {})",
        env!("CARGO_PKG_VERSION"),
        backend_type.name()
    );
    println!("Type 'exit' or 'quit' to exit, 'help' for help");
    println!();

    let mut data: HashMap<String, Value> = HashMap::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input)?;

        // EOF reached
        if bytes_read == 0 {
            println!();
            break;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // Check input length
        if input.len() > MAX_INPUT_LENGTH {
            eprintln!(
                "Error: Input too long (max {} characters)",
                MAX_INPUT_LENGTH
            );
            continue;
        }

        match input {
            "exit" | "quit" => break,
            "help" => {
                print_help();
                continue;
            }
            "clear" => {
                data.clear();
                println!("Data context cleared");
                continue;
            }
            "data" => {
                print_data(&data);
                continue;
            }
            _ => {}
        }

        // Handle set command
        if let Some(rest) = input.strip_prefix("set ") {
            handle_set_command(rest, &mut data);
            continue;
        }

        // Try to compile and evaluate
        eval_input(input, &data, &backend_type);
    }

    Ok(())
}

fn print_help() {
    println!("REPL Commands:");
    println!("  exit, quit     - Exit the REPL");
    println!("  help           - Show this help message");
    println!("  clear          - Clear the data context");
    println!("  data           - Show current data context");
    println!("  set KEY VALUE  - Set a data value (VALUE is JSON)");
    println!();
    println!("Otherwise, enter any Amoskeag expression to evaluate it.");
}

fn print_data(data: &HashMap<String, Value>) {
    if data.is_empty() {
        println!("(empty)");
    } else {
        for (key, value) in data {
            println!("  {}: {}", key, format_value(value));
        }
    }
}

fn handle_set_command(rest: &str, data: &mut HashMap<String, Value>) {
    let parts: Vec<&str> = rest.splitn(2, ' ').collect();
    if parts.len() != 2 {
        eprintln!("Usage: set KEY VALUE (VALUE should be valid JSON)");
        return;
    }

    let key = parts[0].trim();
    let value_str = parts[1].trim();

    if key.is_empty() {
        eprintln!("Error: Key cannot be empty");
        return;
    }

    match serde_json::from_str::<serde_json::Value>(value_str) {
        Ok(json) => match crate::json::json_to_value(&json) {
            Ok(value) => {
                data.insert(key.to_string(), value);
                println!("Set {} = {}", key, value_str);
            }
            Err(e) => eprintln!("Error converting value: {}", e),
        },
        Err(e) => eprintln!("Invalid JSON: {}", e),
    }
}

fn eval_input(input: &str, data: &HashMap<String, Value>, backend_type: &BackendType) {
    match compile(input, &[]) {
        Ok(program) => match evaluate_with_backend(&program, data, backend_type) {
            Ok(result) => {
                println!("=> {}", format_value(&result));
            }
            Err(e) => {
                eprintln!("Evaluation error: {}", e);
            }
        },
        Err(e) => {
            eprintln!("Compilation error: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_set_command_valid() {
        let mut data = HashMap::new();
        handle_set_command("x 42", &mut data);
        assert!(matches!(data.get("x"), Some(Value::Number(n)) if *n == 42.0));
    }

    #[test]
    fn test_handle_set_command_string() {
        let mut data = HashMap::new();
        handle_set_command("name \"alice\"", &mut data);
        assert!(matches!(data.get("name"), Some(Value::String(s)) if s == "alice"));
    }

    #[test]
    fn test_handle_set_command_object() {
        let mut data = HashMap::new();
        handle_set_command("user {\"name\": \"bob\"}", &mut data);
        assert!(matches!(data.get("user"), Some(Value::Dictionary(_))));
    }

    #[test]
    fn test_handle_set_command_no_value() {
        let mut data = HashMap::new();
        // This should print an error but not crash
        handle_set_command("x", &mut data);
        assert!(data.is_empty());
    }

    #[test]
    fn test_handle_set_command_empty_key() {
        let mut data = HashMap::new();
        handle_set_command(" 42", &mut data);
        // Empty key should be rejected
        assert!(data.is_empty());
    }

    #[test]
    fn test_print_data_empty() {
        let data = HashMap::new();
        // Just ensure it doesn't panic
        print_data(&data);
    }

    #[test]
    fn test_print_data_with_values() {
        let mut data = HashMap::new();
        data.insert("x".to_string(), Value::Number(42.0));
        // Just ensure it doesn't panic
        print_data(&data);
    }
}
