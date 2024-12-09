use std::{
    env,
    io::{self, Write},
    path::Path,
    process::Command,
};

const MAX_CMD_LENGTH: usize = 1000;
const MAX_ARGS: usize = 100;

#[derive(Debug)]
enum ShellError {
    MismatchedQuotes,
    TooManyArgs,
    CommandLineTooLong,
    CommandFailed(()),
    CdFailed,
    IoError(io::Error),
}

fn find_command_in_path(cmd: &str) -> Option<std::path::PathBuf> {
    if cmd.contains('/') {
        return Some(Path::new(cmd).to_path_buf());
    }
    
    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            let full_path = path.join(cmd);
            if full_path.is_file() {
                return Some(full_path);
            }
        }
    }
    None
}

fn parse_command(input: &str) -> Result<Vec<String>, ShellError> {
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
                // Skip any subsequent spaces
                while let Some(&next_char) = chars.peek() {
                    if next_char == ' ' {
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
            _ => current_token.push(c),
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
            let command_path = match find_command_in_path(cmd) {
                Some(path) => path,
                None => {
                    eprintln!("error: command not found: {}", cmd);
                    return Err(ShellError::IoError(io::Error::new(
                        io::ErrorKind::NotFound,
                        "command not found"
                    )));
                }
            };
            
            let mut command = Command::new(command_path);
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
                    ()
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