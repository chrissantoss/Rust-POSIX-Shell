use std::{
    env,
    io::{self, Write},
    path::Path,
    process::Command,
};

const MAX_CMD_LENGTH: usize = 1000;
const MAX_ARGS: usize = 100;

#[derive(Debug)] // derive a custom error type
enum ShellError { // allows the enum to be printed
    MismatchedQuotes,
    TooManyArgs,
    CommandLineTooLong,
    CommandFailed(()),
    CdFailed,
    IoError(io::Error),
}


// parse the command line into a vector of arguments
fn parse_command(input: &str) -> Result<Vec<String>, ShellError> {
    // check if the command line is too long
    if input.len() > MAX_CMD_LENGTH {
        return Err(ShellError::CommandLineTooLong);
    }
    
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut chars = input.chars().peekable();

    // iterate over the characters in the command line
    while let Some(c) = chars.next() {
        match c {
            '\'' if !in_double_quotes => in_single_quotes = !in_single_quotes,
            '"' if !in_single_quotes => in_double_quotes = !in_double_quotes,
            ' ' if !in_single_quotes && !in_double_quotes => {
                if !current_token.is_empty() {
                    tokens.push(current_token);
                    current_token = String::new();
                }
            }
            // add the character to the current token
            _ => current_token.push(c),
        }
    }

    // check if the quotes are mismatched
    if in_single_quotes || in_double_quotes {
        return Err(ShellError::MismatchedQuotes);
    }

    // add the last token to the vector if it's not empty
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    // check if the number of arguments is too many
    if tokens.len() > MAX_ARGS {
        return Err(ShellError::TooManyArgs);
    }

    Ok(tokens)
}

// execute the command
fn execute_command(args: Vec<String>) -> Result<(), ShellError> {
    // check if the command is empty
    if args.is_empty() {
        return Ok(());
    }

    // match the command
    match args[0].as_str() {
        "exit" => std::process::exit(0),
        "cd" => {
            // check if the number of arguments is correct
            if args.len() != 2 {
                eprintln!("error: cd requires exactly one argument");
                return Err(ShellError::CdFailed);
            }
            // change the current directory
            if env::set_current_dir(Path::new(&args[1])).is_err() {
                return Err(ShellError::CdFailed);
            }
            Ok(())
        }
        cmd => {
            // check if the command contains a slash
            let mut command = if cmd.contains('/') {
                Command::new(cmd)
            } else {
                Command::new(cmd)
            };

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
                    eprintln!("error: failed to execute command: {}", e);
                    Err(ShellError::IoError(e))
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    loop {
        eprint!("$ ");
        let _ = io::stderr().flush();

        input.clear();
        if stdin.read_line(&mut input)? == 0 {
            // EOF received
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match parse_command(input) {
            Ok(args) => {
                if let Err(e) = execute_command(args) {
                    match e {
                        ShellError::CdFailed => eprintln!("error: cd failed"),
                        ShellError::CommandFailed(_) => (), // Already printed
                        ShellError::IoError(e) => eprintln!("error: {}", e),
                        _ => eprintln!("error: {:?}", e),
                    }
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