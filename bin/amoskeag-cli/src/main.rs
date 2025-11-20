//! Amoskeag CLI
//!
//! Command-line interface for running Amoskeag programs

use amoskeag::{compile, evaluate, AmoskeagValue as Value};
#[cfg(feature = "jit")]
use amoskeag_parser::Expr;
#[cfg(feature = "jit")]
use amoskeag::Backend;
use anyhow::{Context, Result, bail};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

/// Supported backend types
enum BackendType {
    Interpreter,
    #[cfg(feature = "jit")]
    Jit,
}

impl BackendType {
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "interpreter" | "interp" => Ok(BackendType::Interpreter),
            #[cfg(feature = "jit")]
            "jit" => Ok(BackendType::Jit),
            #[cfg(not(feature = "jit"))]
            "jit" => bail!("JIT backend not available. This is an enterprise feature. Contact support for access."),
            _ => bail!("Unknown backend: {}. Available backends: interpreter{}", s,
                      if cfg!(feature = "jit") { ", jit" } else { "" }),
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "run" => {
            if args.len() < 3 {
                eprintln!("Error: 'run' command requires a source file");
                print_usage();
                std::process::exit(1);
            }

            let mut source_file = None;
            let mut data_file = None;
            let mut backend = BackendType::Interpreter;
            let mut symbols = Vec::new();
            let mut i = 2;

            while i < args.len() {
                if args[i] == "--backend" || args[i] == "-b" {
                    if i + 1 >= args.len() {
                        bail!("--backend requires a value");
                    }
                    backend = BackendType::from_str(&args[i + 1])?;
                    i += 2;
                } else if source_file.is_none() {
                    source_file = Some(&args[i]);
                    i += 1;
                } else if data_file.is_none() {
                    data_file = Some(&args[i]);
                    i += 1;
                } else {
                    symbols.push(args[i].as_str());
                    i += 1;
                }
            }

            let source_file = source_file.ok_or_else(|| anyhow::anyhow!("Missing source file"))?;

            run_file(source_file, data_file, &symbols, backend)?;
        }
        "eval" => {
            if args.len() < 3 {
                eprintln!("Error: 'eval' command requires a source string");
                print_usage();
                std::process::exit(1);
            }

            let mut source = None;
            let mut data_file = None;
            let mut backend = BackendType::Interpreter;
            let mut symbols = Vec::new();
            let mut i = 2;

            while i < args.len() {
                if args[i] == "--backend" || args[i] == "-b" {
                    if i + 1 >= args.len() {
                        bail!("--backend requires a value");
                    }
                    backend = BackendType::from_str(&args[i + 1])?;
                    i += 2;
                } else if source.is_none() {
                    source = Some(&args[i]);
                    i += 1;
                } else if data_file.is_none() {
                    data_file = Some(&args[i]);
                    i += 1;
                } else {
                    symbols.push(args[i].as_str());
                    i += 1;
                }
            }

            let source = source.ok_or_else(|| anyhow::anyhow!("Missing source expression"))?;

            eval_string(source, data_file, &symbols, backend)?;
        }
        "repl" => {
            let mut backend = BackendType::Interpreter;
            if args.len() > 2 && (args[2] == "--backend" || args[2] == "-b") {
                if args.len() > 3 {
                    backend = BackendType::from_str(&args[3])?;
                }
            }
            run_repl(backend)?;
        }
        "--help" | "-h" | "help" => {
            print_usage();
        }
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

fn run_file(source_file: &str, data_file: Option<&String>, symbols: &[&str], backend_type: BackendType) -> Result<()> {
    // Read the source file
    let source = fs::read_to_string(source_file)
        .with_context(|| format!("Failed to read source file: {}", source_file))?;

    // Read the data file (if provided)
    let data = if let Some(data_path) = data_file {
        let data_content = fs::read_to_string(data_path)
            .with_context(|| format!("Failed to read data file: {}", data_path))?;
        parse_json_data(&data_content)?
    } else {
        HashMap::new()
    };

    // Compile the program
    let program = compile(&source, symbols)
        .with_context(|| "Failed to compile program")?;

    // Evaluate using the selected backend
    let result = evaluate_with_backend(&program, &data, &backend_type)?;

    // Print the result
    println!("{}", format_value(&result));

    Ok(())
}

fn eval_string(source: &str, data_file: Option<&String>, symbols: &[&str], backend_type: BackendType) -> Result<()> {
    // Read the data file (if provided)
    let data = if let Some(data_path) = data_file {
        let data_content = fs::read_to_string(data_path)
            .with_context(|| format!("Failed to read data file: {}", data_path))?;
        parse_json_data(&data_content)?
    } else {
        HashMap::new()
    };

    // Compile the program
    let program = compile(source, symbols)
        .with_context(|| "Failed to compile program")?;

    // Evaluate using the selected backend
    let result = evaluate_with_backend(&program, &data, &backend_type)?;

    // Print the result
    println!("{}", format_value(&result));

    Ok(())
}

fn run_repl(backend_type: BackendType) -> Result<()> {
    let backend_name = match backend_type {
        BackendType::Interpreter => "interpreter",
        #[cfg(feature = "jit")]
        BackendType::Jit => "jit",
    };

    println!("Amoskeag REPL v{} (backend: {})", env!("CARGO_PKG_VERSION"), backend_name);
    println!("Type 'exit' or 'quit' to exit, 'help' for help");
    println!();

    let mut data = HashMap::new();

    loop {
        print!("> ");
        use std::io::Write;
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match input {
            "exit" | "quit" => break,
            "help" => {
                println!("REPL Commands:");
                println!("  exit, quit - Exit the REPL");
                println!("  help       - Show this help message");
                println!("  clear      - Clear the data context");
                println!();
                println!("Otherwise, enter any Amoskeag expression to evaluate it.");
                continue;
            }
            "clear" => {
                data.clear();
                println!("Data context cleared");
                continue;
            }
            _ => {}
        }

        // Try to compile and evaluate
        match compile(input, &[]) {
            Ok(program) => {
                match evaluate_with_backend(&program, &data, &backend_type) {
                    Ok(result) => {
                        println!("=> {}", format_value(&result));
                    }
                    Err(e) => {
                        eprintln!("Evaluation error: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Compilation error: {}", e);
            }
        }
    }

    Ok(())
}

/// Evaluate a program using the selected backend
fn evaluate_with_backend(
    program: &amoskeag::CompiledProgram,
    data: &HashMap<String, Value>,
    backend_type: &BackendType,
) -> Result<Value> {
    match backend_type {
        BackendType::Interpreter => {
            // Use the standard evaluate function (interpreter backend)
            evaluate(program, data).map_err(|e| anyhow::anyhow!("{}", e))
        }
        #[cfg(feature = "jit")]
        BackendType::Jit => {
            use amoskeag_jit::backend::JitBackend;
            use inkwell::context::Context;

            let backend = JitBackend::new();
            let expr = get_program_ast(program);

            // Check if the JIT supports this expression
            if !backend.supports(expr) {
                bail!("JIT backend does not support this expression. Try using --backend interpreter instead.\nNote: JIT currently only supports numeric expressions without runtime data access.");
            }

            // Compile and execute
            let result = backend.compile_and_execute(expr, &[], data)
                .map_err(|e| anyhow::anyhow!("JIT error: {}", e))?;

            Ok(result)
        }
    }
}

/// Helper to get the AST from a compiled program
#[cfg(feature = "jit")]
fn get_program_ast(program: &amoskeag::CompiledProgram) -> &Expr {
    program.ast()
}

fn parse_json_data(json: &str) -> Result<HashMap<String, Value>> {
    let json_value: serde_json::Value = serde_json::from_str(json)
        .with_context(|| "Failed to parse JSON data")?;

    let map = json_to_value_map(&json_value)?;
    Ok(map)
}

fn json_to_value(json: &serde_json::Value) -> Result<Value> {
    match json {
        serde_json::Value::Null => Ok(Value::Nil),
        serde_json::Value::Bool(b) => Ok(Value::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                Ok(Value::Number(f))
            } else {
                anyhow::bail!("Invalid number in JSON")
            }
        }
        serde_json::Value::String(s) => Ok(Value::String(s.clone())),
        serde_json::Value::Array(arr) => {
            let values: Result<Vec<Value>> = arr.iter().map(json_to_value).collect();
            Ok(Value::Array(values?))
        }
        serde_json::Value::Object(obj) => {
            let mut map = HashMap::new();
            for (key, value) in obj {
                map.insert(key.clone(), json_to_value(value)?);
            }
            Ok(Value::Dictionary(map))
        }
    }
}

fn json_to_value_map(json: &serde_json::Value) -> Result<HashMap<String, Value>> {
    match json {
        serde_json::Value::Object(obj) => {
            let mut map = HashMap::new();
            for (key, value) in obj {
                map.insert(key.clone(), json_to_value(value)?);
            }
            Ok(map)
        }
        _ => anyhow::bail!("Data must be a JSON object"),
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Boolean(b) => b.to_string(),
        Value::Nil => "nil".to_string(),
        Value::Symbol(s) => format!(":{}", s),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(format_value).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Dictionary(map) => {
            let items: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("{}: {}", k, format_value(v)))
                .collect();
            format!("{{{}}}", items.join(", "))
        }
    }
}

fn print_usage() {
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
    println!("  -b, --backend <name>   Select execution backend");
    println!("                         Available: interpreter{}",
             if cfg!(feature = "jit") { ", jit" } else { "" });
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
