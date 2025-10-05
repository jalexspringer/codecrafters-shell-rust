use std::io::{self, Write};
// use std::env;

const BUILTINS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

mod shell_command;
mod utils;

use shell_command::Command;


fn build_prompt(token: String) -> String {
    // let pwd = env::current_dir().unwrap();
    // let user = env::var("USER").unwrap();
    
//     format!("\n{}
// {} {token} ", pwd.display(), user)

    format!("{token} ")
}


fn main() {
    loop {
	let token = String::from("$");
	print!("{}", build_prompt(token));
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

