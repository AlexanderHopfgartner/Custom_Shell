use std::{io, env};
use std::fs::metadata;
use std::io::Write;
use std::path::{Path, MAIN_SEPARATOR};
use std::string::ToString;
use crate::command::RCommand;


#[cfg(target_os = "linux")]
const PATH_SEPARATOR: char = ':';
#[cfg(target_os = "windows")]
const PATH_SEPARATOR: char = ';';


#[cfg(target_os = "linux")]
const COMMAND_ENDIGN: &str = "";
#[cfg(target_os = "windows")]
const COMMAND_ENDIGN: &str = ".exe";


pub struct Shell {
    input: String,
    pub path_env: Vec<String>,
}

impl Shell {
    
    pub fn read_commands_from_path() -> Vec<String> {
        match env::var("PATH") {
            Ok(path) => read_commands(path),
            Err(_) => Vec::new(),
        }
    }
    
    pub fn new() -> Shell {
        Shell {
            input: String::new(),
            path_env: Shell::read_commands_from_path(),
        }
    }

    pub fn read_user_input(&mut self) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        self.input = input;
    }

    // Parse the input string into a command and its arguments
    pub fn parse_input(&self) -> Option<RCommand> {
        let input = self.input.trim();

        // Find the first space to separate command and arguments
        let mut splitter = input.splitn(2, char::is_whitespace);
        let command = splitter.next()?;
        let args_str = splitter.next().unwrap_or("").trim();

        let args = split_argument(args_str);

        Some(RCommand {
            command: command.parse().unwrap(),
            args,
        })
    }

    // checks if the given command is available in the PATH environment variable
    pub fn has_command(&self, command: &String) -> Option<String> {
        for path in self.path_env.iter() {
            let full_path = format!("{}{}{}{}", path, MAIN_SEPARATOR, command, COMMAND_ENDIGN);
            println!("{}", full_path);
            if metadata(&full_path).is_ok() {
                return Some(full_path)
            }
        }
        None
    }
    
    pub fn change_dir(&self, path: &Path) -> io::Result<()> {
        if path.starts_with("~") {
            env::set_current_dir(env::var("HOME").unwrap())
        } else {
            env::set_current_dir(path) 
        }
    }
    
    pub fn current_dir(&self) -> String {
        env::current_dir().unwrap().to_str().unwrap().to_string()
    }
}


fn split_argument(arguments: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let mut quote_is_open = false;
    let mut next_argument: Vec<&str> = Vec::new();

    let arguments_chars: Vec<&str> = arguments.split("").collect();

    for letter in arguments_chars {
        match letter {
            "\"" => {
                argument_quotation_helper(&mut args, &mut quote_is_open, &mut next_argument);
            }

            "'" => {
                argument_quotation_helper(&mut args, &mut quote_is_open, &mut next_argument);
            }

            _ => {
                if (letter == " ") & (next_argument.last() != Some(&" ")) {
                    next_argument.push(letter);
                } else if quote_is_open {
                    next_argument.push(letter);
                } else if letter != " " {
                    next_argument.push(letter);
                }
            }
        }
        quote_is_open ^= letter == "\"" || letter == "'";
    }
    if !next_argument.is_empty() {
        args.push(next_argument.join(""));
    }
    args
}

fn argument_quotation_helper(args: &mut Vec<String>, quote_is_open: &bool, next_argument: &mut Vec<&str>) {
    if !quote_is_open ^ true {
        if !next_argument.is_empty() {
            let next_letter = next_argument.join("");
            args.push(next_letter);
            next_argument.clear();
        }
    }
}

pub fn prompt_input() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn read_commands(paths: String) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    
    for path in paths.split(PATH_SEPARATOR) {
        commands.push(path.to_string());
    }
    commands
}
