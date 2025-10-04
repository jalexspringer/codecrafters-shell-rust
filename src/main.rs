#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
	print!("$ ");
	io::stdout().flush().unwrap();

	// Wait for user input
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	
	// Exit clause
	if input.contains("exit 0") { break }
	else if input.starts_with("echo") {
	    println!("{}", input.strip_prefix("echo ")
		     .expect("Nothing to echo!")
		     .to_string()
		     .trim());
	    
	    continue;
	}

	println!("{}: command not found", input.trim());
    }
}
