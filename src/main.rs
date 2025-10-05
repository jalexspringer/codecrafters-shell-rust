use std::io::{self, Write};

const BUILTINS: [&str; 4] = ["exit", "echo", "type", "pwd"];

mod shell_command;
mod utils;

use shell_command::Command;


fn main() {
    loop {
	print!("$ ");
	io::stdout().flush().unwrap();

	// Wait for user input
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	let command = Command::from_input(input);
	match command {
	    Command::Exit => break,
		_ => Command::run_command(command),
	}
    }
}

