#[allow(unused_imports)]
use std::{env, fs};
use std::io::{self, Write};
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;

const BUILTINS: [&str; 3] = ["exit", "echo", "type"];

enum Command {
    Exit,
    Echo(String),
    Type(String),
    NotFound(String),
}

impl Command {
    fn from_input(input: String) -> Self {
	let input = input.trim();
	
	// Exit clause
	if input.starts_with("exit 0") { Self::Exit  }

	// Echo
	else if input.starts_with("echo") {
	    if let Some(to_echo) = input.strip_prefix("echo ") {
		Self::Echo(to_echo.trim().to_string())
	    } else {
		Self::Echo(String::from("What do you want me to echo?"))
	    }

	// Type
	} else if input.starts_with("type") {
	    if let Some(type_of) = input.strip_prefix("type ") {
		Self::Type(type_of.trim().to_string())
	    } else {
		Self::Type(String::from(""))}
	}

	// Not a builtin
	else {
	    Self::NotFound(input.to_string())
	}
    }
}


fn find_file_in_dir(file: &str, dir: &str) -> Option<PathBuf> {
    let Ok(entries) = fs::read_dir(dir) else {
        return None;
    };
    
    for entry in entries {
        let full_path = entry.expect("No path?").path();
        if full_path.file_stem().unwrap() == file {
            return Some(full_path);
        }
    }
    None
}


fn find_executable(x: &str) -> Option<PathBuf>{
    let path_string = env::var("PATH").expect("No path is set");
    let path: Vec<&str> = path_string.split(":").collect();
    for dir in path {
	if let Some(path_to_file) = find_file_in_dir(x, dir) {
	    let mode = fs::metadata(&path_to_file).unwrap().permissions().mode();
	    if  mode & 0o100 != 0 {
		return Some(path_to_file)
	    }
	}
    }
    // No executable in path
    None
}


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
	    Command::Echo(x) => println!("{}", x),
	    Command::Type(x) => {
		// Builtin takes priority
		if BUILTINS.contains(&x.as_str()) { println!("{} is a shell builtin", x) }

		// Match the path
		else if let Some(executable) = find_executable(&x) {
  			println!("{x} is {}", executable.display());	
  		    } else {
  			println!("{}: not found", x);
  		    }
	    },
	    Command::NotFound(x) => println!("{}: command not found", x),
	}
    }
}

