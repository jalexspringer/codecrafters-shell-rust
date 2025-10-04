#[allow(unused_imports)]
use std::io::{self, Write};

enum BuiltIn {
    Exit(String),
    Echo,
    Type,
}

fn main() {
    let builtin = vec!["exit", "echo", "type"];
    loop {
	print!("$ ");
	io::stdout().flush().unwrap();

	// Wait for user input
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	
	// Exit clause
	if input.contains("exit 0") { break }

	// Echo
	else if input.starts_with("echo") {
	    println!("{}", input.strip_prefix("echo ")
		     .expect("Nothing to echo!")
		     .to_string()
		     .trim());
	    continue;
	} else if input.starts_with("type") {
	    let type_of = input.strip_prefix("type ")
		.expect("Type of what mate?").trim();
	    if builtin.contains(&type_of) {
		println!("{} is a shell builtin", type_of);
	    } else {
		println!("{}: not found", type_of);
	    }
	    continue;
	}

	println!("{}: command not found", input.trim());
    }
}
