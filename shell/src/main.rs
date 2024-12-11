use std::{
    env,
    io::{self, Write},
    path::Path,
    process::Command,
};

// Constants defining shell limitations
const MAX_CMD_LENGTH: usize = 1000;
const MAX_ARGS: usize = 100;

// Custom error types for shell operations
#[derive(Debug)]
enum ShellError {
    MismatchedQuotes,
    TooManyArgs,
    CommandLineTooLong,
    CommandFailed(()),
    CdFailed,
    IoError(io::Error),
}

/// Parses a command string into a vector of arguments, handling quoted strings
/// 
/// This function implements shell-like parsing with the following features:
/// - Handles both single and double quotes
/// - Strips extra spaces between arguments
/// - Enforces maximum command length and argument count
/// - Returns error for mismatched quotes
fn parse_command(input: &str) -> Result<Vec<String>, ShellError> {
    let input = input.trim();
    
    if input.is_empty() {
        return Ok(Vec::new());
    }
    
    if input.len() > MAX_CMD_LENGTH {
        return Err(ShellError::CommandLineTooLong);
    }

    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\'' if !in_double_quotes => in_single_quotes = !in_single_quotes,
            '"' if !in_single_quotes => in_double_quotes = !in_double_quotes,
            ' ' if !in_single_quotes && !in_double_quotes => {
                if !current_token.is_empty() {
                    tokens.push(current_token);
                    current_token = String::new();
                }
                while chars.peek().map_or(false, |&next| next == ' ') {
                    chars.next();
                }
            }
            _ => {
                if in_single_quotes || in_double_quotes {
                    current_token.push(c);
                } else {
                    if c != ' ' || !current_token.is_empty() {
                        current_token.push(c);
                    }
                }
            }
        }
    }

    if in_single_quotes || in_double_quotes {
        return Err(ShellError::MismatchedQuotes);
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    if tokens.len() > MAX_ARGS {
        return Err(ShellError::TooManyArgs);
    }

    Ok(tokens)
}

/// Executes a parsed command with its arguments
/// 
/// Handles three cases:
/// 1. Built-in 'exit' command - terminates the shell
/// 2. Built-in 'cd' command - changes current directory
/// 3. External commands - spawns a new process using Command
fn execute_command(args: Vec<String>) -> Result<(), ShellError> {
    if args.is_empty() {
        return Ok(());
    }

    match args[0].as_str() {
        "exit" => std::process::exit(0),
        "cd" => {
            if args.len() != 2 {
                eprintln!("error: cd requires exactly one argument");
                return Err(ShellError::CdFailed);
            }
            match env::set_current_dir(Path::new(&args[1])) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("error: cd failed: {}", e);
                    Err(ShellError::CdFailed)
                }
            }
        }
        cmd => {
            let mut command = Command::new(cmd);
            command.args(&args[1..]);

            match command.status() {
                Ok(status) => {
                    if !status.success() {
                        let code = status.code().unwrap_or(1);
                        eprintln!("error: command exited with code {}", code);
                        Err(ShellError::CommandFailed(()))
                    } else {
                        Ok(())
                    }
                }
                Err(e) => {
                    eprintln!("error: command not found");
                    Err(ShellError::IoError(e))
                }
            }
        }
    }
}

/// Main shell loop that:
/// - Displays the prompt
/// - Reads user input
/// - Parses and executes commands
/// - Handles errors gracefully
/// - Exits on EOF (Ctrl+D)
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    loop {
        // Display shell prompt on stderr
        eprint!("$ ");
        let _ = io::stderr().flush();

        // Read command from stdin, handle EOF
        input.clear();
        if stdin.read_line(&mut input)? == 0 {
            break;  // EOF received (Ctrl+D)
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Parse and execute the command, handling any errors
        match parse_command(input) {
            Ok(args) => {
                if let Err(e) = execute_command(args) {
                    ()  // Error already printed in execute_command
                }
            }
            Err(e) => {
                match e {
                    ShellError::MismatchedQuotes => eprintln!("error: mismatched quotes"),
                    ShellError::TooManyArgs => eprintln!("error: too many arguments"),
                    ShellError::CommandLineTooLong => eprintln!("error: command line too long"),
                    _ => eprintln!("error: {:?}", e),
                }
            }
        }
    }

    Ok(())
} 