use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use crate::shell::Shell;


type ReturnCode = i32;

const ALL_COMMANDS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

#[derive(Eq, PartialEq, Debug)]
pub struct RCommand {
    pub command: String,
    pub args: Vec<String>,
}

impl RCommand {
    pub fn get_first_arg(&self) -> String {
        self.args.first().unwrap().to_string()
    }
}

// List of Available Commands
pub enum CommandAction {
    Echo(String),
    Error(String),
    Exit(ReturnCode),
    Type(String),
    NotFound(String),
    Command(String),
    PWD(String),
    None,
}

// Execute Command corresponding to the given command
pub fn execute_command(shell: &Shell, command: RCommand) {
    match match_command(shell, command) {
        CommandAction::Echo(s) => println!("{}", s),
        CommandAction::Error(e) => println!("{}", e),
        CommandAction::Type(s) => println!("{}", s),
        CommandAction::NotFound(s) => println!("{}", s),
        CommandAction::Command(s) => println!("{}", s),
        CommandAction::PWD(s) => println!("{}", s),
        CommandAction::Exit(code) => std::process::exit(code),
        CommandAction::None => ()
    }
}

// Match Command to the corresponding function
fn match_command(shell: &Shell, command: RCommand) -> CommandAction {
    println!("{}", command.command.as_str());
    match command.command.as_str() {
        "exit" => execute_exit(command),
        "echo" => execute_echo(command),
        "type" => execute_type(shell, command),
        "pwd" => execute_pwd(shell),
        "cd" => execute_cd(shell, command),
        _ => {
            println!("yay othzer");
            execute_programm(shell, command)
        },

    }
}

pub fn execute_cd(shell: &Shell, command: RCommand) -> CommandAction {
    let args0 = command.get_first_arg();
    let goal_path = Path::new(&args0);
    if let Ok(_) = shell.change_dir(goal_path) {
        CommandAction::None
    } else {
        CommandAction::Error(
            format!("{}: {}: No such file or directory", command.command, args0)
        )
    }
}


pub fn execute_pwd(shell: &Shell) -> CommandAction {
    CommandAction::PWD(shell.current_dir())
}


pub fn execute_programm(shell: &Shell, command: RCommand) -> CommandAction {
    if let Some(_) = shell.has_command(&command.command) {
        if let Ok(process_output) = Command::new(&command.command)
            .stdout(std::process::Stdio::piped())
            .args(&command.args)
            .output()
        {
            let error = String::from_utf8(process_output.stderr).expect("Failed to convert output to string");
            return if error.len() < 1 {
                let output = String::from_utf8(process_output.stdout).expect("Failed to convert output to string");
                CommandAction::Command(output.trim().to_string())
            } else {
                CommandAction::Error(error.trim().to_string())
            }
        }
    }
    execute_error(command)
}

// Intended exit of the program
pub fn execute_exit(command: RCommand) -> CommandAction {
    CommandAction::Exit(
        i32::from_str(command.args.get(0).expect("missing argument"))
            .expect("argument should be a number"))
}


pub fn execute_echo(command: RCommand) -> CommandAction {
    CommandAction::Echo(command.args.join(""))
}

// Get hold of the first Argument to map it to build in, or existing command get CommandAction
pub fn execute_type(shell: &Shell, command: RCommand) -> CommandAction {
    if let args0 = command.get_first_arg() {
        print!("{}", args0);
        if ALL_COMMANDS.contains(&&*args0) {
            CommandAction::Type(format!("{} is a shell builtin", args0))
         } else if let Some(command_path) = shell.has_command(&args0) {
            CommandAction::Type(format!("{} is {}", args0, command_path))
        } else {
            execute_not_found(args0.to_string())
        }
    } else {
        CommandAction::Type(format!("{} is a shell builtin", command.command))
    }
}

pub fn execute_error(command: RCommand) -> CommandAction {
    CommandAction::Error(format!("{}: command not found", command.command))
}

pub fn execute_not_found(command: String) -> CommandAction {
    CommandAction::NotFound(format!("{}: not found", command))
}