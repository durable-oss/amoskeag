//! CLI command implementations

use crate::backend::{evaluate_with_backend, BackendType};
use crate::format::format_value;
use crate::json::parse_json_data;
use amoskeag::compile;
use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Maximum source file size in bytes (10 MB)
const MAX_SOURCE_SIZE: u64 = 10 * 1024 * 1024;

/// Maximum data file size in bytes (100 MB)
const MAX_DATA_SIZE: u64 = 100 * 1024 * 1024;

/// Run a program from a source file
///
/// # Errors
/// Returns an error if the file cannot be read, parsed, or evaluated.
pub fn run_file(
    source_file: &str,
    data_file: Option<&String>,
    symbols: &[&str],
    backend_type: BackendType,
) -> Result<()> {
    // Validate source file
    validate_file_path(source_file)?;
    validate_file_size(source_file, MAX_SOURCE_SIZE, "Source")?;

    // Read the source file
    let source = fs::read_to_string(source_file)
        .with_context(|| format!("Failed to read source file: {}", source_file))?;

    if source.trim().is_empty() {
        bail!("Source file is empty: {}", source_file);
    }

    // Read the data file (if provided)
    let data = load_data_file(data_file)?;

    // Compile the program
    let program = compile(&source, symbols).with_context(|| "Failed to compile program")?;

    // Evaluate using the selected backend
    let result = evaluate_with_backend(&program, &data, &backend_type)?;

    // Print the result
    println!("{}", format_value(&result));

    Ok(())
}

/// Evaluate an expression from a string
///
/// # Errors
/// Returns an error if the expression cannot be parsed or evaluated.
pub fn eval_string(
    source: &str,
    data_file: Option<&String>,
    symbols: &[&str],
    backend_type: BackendType,
) -> Result<()> {
    if source.trim().is_empty() {
        bail!("Source expression is empty");
    }

    // Read the data file (if provided)
    let data = load_data_file(data_file)?;

    // Compile the program
    let program = compile(source, symbols).with_context(|| "Failed to compile program")?;

    // Evaluate using the selected backend
    let result = evaluate_with_backend(&program, &data, &backend_type)?;

    // Print the result
    println!("{}", format_value(&result));

    Ok(())
}

fn validate_file_path(path: &str) -> Result<()> {
    if path.is_empty() {
        bail!("File path cannot be empty");
    }

    let p = Path::new(path);

    if !p.exists() {
        bail!("File does not exist: {}", path);
    }

    if !p.is_file() {
        bail!("Path is not a file: {}", path);
    }

    Ok(())
}

fn validate_file_size(path: &str, max_size: u64, file_type: &str) -> Result<()> {
    let metadata = fs::metadata(path).with_context(|| format!("Failed to read file metadata: {}", path))?;

    if metadata.len() > max_size {
        bail!(
            "{} file too large: {} bytes (max {} bytes)",
            file_type,
            metadata.len(),
            max_size
        );
    }

    Ok(())
}

fn load_data_file(data_file: Option<&String>) -> Result<HashMap<String, amoskeag::AmoskeagValue>> {
    if let Some(data_path) = data_file {
        validate_file_path(data_path)?;
        validate_file_size(data_path, MAX_DATA_SIZE, "Data")?;

        let data_content = fs::read_to_string(data_path)
            .with_context(|| format!("Failed to read data file: {}", data_path))?;

        parse_json_data(&data_content)
    } else {
        Ok(HashMap::new())
    }
}

/// Print usage information
pub fn print_usage() {
    println!("Amoskeag CLI v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("USAGE:");
    println!("  amoskeag run <source-file> [options] [data-file] [symbols...]");
    println!("  amoskeag eval <source-string> [options] [data-file] [symbols...]");
    println!("  amoskeag repl [options]");
    println!("  amoskeag --help");
    println!("  amoskeag --version");
    println!();
    println!("COMMANDS:");
    println!("  run    Run an Amoskeag program from a file");
    println!("  eval   Evaluate an Amoskeag expression from a string");
    println!("  repl   Start an interactive REPL");
    println!();
    println!("OPTIONS:");
    println!(
        "  -b, --backend <name>   Select execution backend (available: {})",
        BackendType::available_backends()
    );
    println!("  -h, --help             Print help information");
    println!("  -v, --version          Print version information");
    println!();
    println!("BACKENDS:");
    println!("  interpreter  Tree-walking interpreter (default, full language support)");
    #[cfg(feature = "jit")]
    println!("  jit          LLVM JIT compiler (enterprise feature, fast numeric expressions)");
    #[cfg(not(feature = "jit"))]
    println!("  jit          Enterprise feature (contact support for access)");
    println!();
    println!("ARGUMENTS:");
    println!("  <source-file>    Path to the Amoskeag source file (.amos)");
    println!("  <source-string>  Amoskeag expression to evaluate");
    println!("  [data-file]      Optional path to JSON data file");
    println!("  [symbols...]     Optional list of valid symbol names (without colons)");
    println!();
    println!("EXAMPLES:");
    println!("  amoskeag run example.amos");
    println!("  amoskeag run example.amos data.json approve deny");
    println!("  amoskeag eval \"2 + 3\"");
    println!("  amoskeag eval \"2 + 3 * 4\" --backend jit");
    println!("  amoskeag eval \"if user.age > 18 :adult else :minor end\" user.json adult minor");
    println!("  amoskeag repl");
    println!("  amoskeag repl --backend jit");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_validate_file_path_empty() {
        assert!(validate_file_path("").is_err());
    }

    #[test]
    fn test_validate_file_path_nonexistent() {
        assert!(validate_file_path("/nonexistent/path/file.txt").is_err());
    }

    #[test]
    fn test_validate_file_path_directory() {
        assert!(validate_file_path("/tmp").is_err());
    }

    #[test]
    fn test_validate_file_path_valid() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "test").unwrap();
        assert!(validate_file_path(temp.path().to_str().unwrap()).is_ok());
    }

    #[test]
    fn test_validate_file_size() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "test").unwrap();
        let path = temp.path().to_str().unwrap();

        // File should be under 1MB
        assert!(validate_file_size(path, 1024 * 1024, "Test").is_ok());

        // File should be over 1 byte (it has "test\n")
        assert!(validate_file_size(path, 1, "Test").is_err());
    }

    #[test]
    fn test_load_data_file_none() {
        let result = load_data_file(None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_load_data_file_valid() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "{{\"x\": 42}}").unwrap();
        let path = temp.path().to_str().unwrap().to_string();

        let result = load_data_file(Some(&path)).unwrap();
        assert!(result.contains_key("x"));
    }

    #[test]
    fn test_load_data_file_invalid_json() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "not json").unwrap();
        let path = temp.path().to_str().unwrap().to_string();

        assert!(load_data_file(Some(&path)).is_err());
    }

    #[test]
    fn test_eval_string_empty() {
        let result = eval_string("", None, &[], BackendType::Interpreter);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_string_whitespace() {
        let result = eval_string("   ", None, &[], BackendType::Interpreter);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_file_empty_path() {
        let result = run_file("", None, &[], BackendType::Interpreter);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_file_nonexistent() {
        let result = run_file("/nonexistent/file.amos", None, &[], BackendType::Interpreter);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_file_empty_content() {
        let temp = NamedTempFile::new().unwrap();
        let path = temp.path().to_str().unwrap();
        let result = run_file(path, None, &[], BackendType::Interpreter);
        assert!(result.is_err());
    }
}
