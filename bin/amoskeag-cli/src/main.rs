//! Amoskeag CLI
//!
//! Command-line interface for running Amoskeag programs

mod backend;
mod commands;
mod format;
mod json;
mod repl;

use backend::BackendType;
use commands::{eval_string, print_usage, run_file};
use repl::run_repl;

use anyhow::{bail, Result};
use std::env;

/// Maximum number of command line arguments to prevent abuse
const MAX_ARGS: usize = 1000;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // Defensive check on argument count
    if args.len() > MAX_ARGS {
        bail!("Too many arguments (max {})", MAX_ARGS);
    }

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "run" => handle_run_command(&args)?,
        "eval" => handle_eval_command(&args)?,
        "repl" => handle_repl_command(&args)?,
        "--help" | "-h" | "help" => print_usage(),
        "--version" | "-v" | "version" => {
            println!("amoskeag {}", env!("CARGO_PKG_VERSION"));
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            print_usage();
            std::process::exit(1);
        }
    }

    Ok(())
}

fn handle_run_command(args: &[String]) -> Result<()> {
    if args.len() < 3 {
        eprintln!("Error: 'run' command requires a source file");
        print_usage();
        std::process::exit(1);
    }

    let (source_file, data_file, symbols, backend) = parse_run_eval_args(args)?;

    let source_file = source_file.ok_or_else(|| anyhow::anyhow!("Missing source file"))?;

    run_file(source_file, data_file, &symbols, backend)
}

fn handle_eval_command(args: &[String]) -> Result<()> {
    if args.len() < 3 {
        eprintln!("Error: 'eval' command requires a source string");
        print_usage();
        std::process::exit(1);
    }

    let (source, data_file, symbols, backend) = parse_run_eval_args(args)?;

    let source = source.ok_or_else(|| anyhow::anyhow!("Missing source expression"))?;

    eval_string(source, data_file, &symbols, backend)
}

fn handle_repl_command(args: &[String]) -> Result<()> {
    let mut backend = BackendType::default();

    if args.len() > 2 && (args[2] == "--backend" || args[2] == "-b") {
        if args.len() > 3 {
            backend = BackendType::from_str(&args[3])?;
        } else {
            bail!("--backend requires a value");
        }
    }

    run_repl(backend)
}

/// Parse arguments for run and eval commands
/// Returns (source, data_file, symbols, backend)
fn parse_run_eval_args<'a>(
    args: &'a [String],
) -> Result<(
    Option<&'a str>,
    Option<&'a String>,
    Vec<&'a str>,
    BackendType,
)> {
    let mut source = None;
    let mut data_file = None;
    let mut backend = BackendType::default();
    let mut symbols = Vec::new();
    let mut i = 2;

    while i < args.len() {
        let arg = &args[i];

        if arg == "--backend" || arg == "-b" {
            if i + 1 >= args.len() {
                bail!("--backend requires a value");
            }
            backend = BackendType::from_str(&args[i + 1])?;
            i += 2;
        } else if arg.starts_with("--") || arg.starts_with('-') {
            bail!("Unknown option: {}", arg);
        } else if source.is_none() {
            source = Some(args[i].as_str());
            i += 1;
        } else if data_file.is_none() {
            data_file = Some(&args[i]);
            i += 1;
        } else {
            symbols.push(args[i].as_str());
            i += 1;
        }
    }

    Ok((source, data_file, symbols, backend))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_args(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_parse_run_eval_args_basic() {
        let args = make_args(&["amoskeag", "run", "file.amos"]);
        let (source, data, symbols, backend) = parse_run_eval_args(&args).unwrap();
        assert_eq!(source, Some("file.amos"));
        assert!(data.is_none());
        assert!(symbols.is_empty());
        assert_eq!(backend, BackendType::Interpreter);
    }

    #[test]
    fn test_parse_run_eval_args_with_data() {
        let args = make_args(&["amoskeag", "run", "file.amos", "data.json"]);
        let (source, data, symbols, _) = parse_run_eval_args(&args).unwrap();
        assert_eq!(source, Some("file.amos"));
        assert_eq!(data.map(|s| s.as_str()), Some("data.json"));
        assert!(symbols.is_empty());
    }

    #[test]
    fn test_parse_run_eval_args_with_symbols() {
        let args = make_args(&["amoskeag", "run", "file.amos", "data.json", "approve", "deny"]);
        let (_, _, symbols, _) = parse_run_eval_args(&args).unwrap();
        assert_eq!(symbols, vec!["approve", "deny"]);
    }

    #[test]
    fn test_parse_run_eval_args_with_backend() {
        let args = make_args(&["amoskeag", "run", "--backend", "interpreter", "file.amos"]);
        let (source, _, _, backend) = parse_run_eval_args(&args).unwrap();
        assert_eq!(source, Some("file.amos"));
        assert_eq!(backend, BackendType::Interpreter);
    }

    #[test]
    fn test_parse_run_eval_args_backend_short() {
        let args = make_args(&["amoskeag", "run", "-b", "interpreter", "file.amos"]);
        let (source, _, _, backend) = parse_run_eval_args(&args).unwrap();
        assert_eq!(source, Some("file.amos"));
        assert_eq!(backend, BackendType::Interpreter);
    }

    #[test]
    fn test_parse_run_eval_args_backend_no_value() {
        let args = make_args(&["amoskeag", "run", "--backend"]);
        assert!(parse_run_eval_args(&args).is_err());
    }

    #[test]
    fn test_parse_run_eval_args_unknown_option() {
        let args = make_args(&["amoskeag", "run", "--unknown", "file.amos"]);
        assert!(parse_run_eval_args(&args).is_err());
    }

    #[test]
    fn test_parse_run_eval_args_empty() {
        let args = make_args(&["amoskeag", "run"]);
        let (source, data, symbols, _) = parse_run_eval_args(&args).unwrap();
        assert!(source.is_none());
        assert!(data.is_none());
        assert!(symbols.is_empty());
    }
}
