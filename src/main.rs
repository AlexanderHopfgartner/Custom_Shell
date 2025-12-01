mod shell;
mod command;

use crate::shell::{prompt_input, Shell};
use crate::command::{execute_command};


fn main() {
    let mut shell = Shell::new();
    prompt_input();
    
    loop {
        shell.read_user_input();

        if let Some(command) = shell.parse_input() {
            execute_command(&shell, command)
        }
        prompt_input();
    }
}


