use std::path::{Path,PathBuf};
use std::{ env, process };

use crate::{BUILTINS,HOME_SYMBOL};
use crate::utils::find_executable;

pub(crate) enum Command {
    Exit,
    Pwd,
    Cd(PathBuf),
    Echo(String),
    Type(String),
    Executable {
        path_to_executable: PathBuf,
        input: String
    },
    NotFound(String),
}

impl Command {
    pub(crate) fn from_input(input: String) -> Self {
        let input = input.trim();

        // Exit clause
        if input.starts_with("exit 0") { Self::Exit  }

        // pwd
        else if input.starts_with("pwd") { Self::Pwd }

	// cd
	else if input.starts_with("cd") {
	    if let Some(target_dir_string) = input.strip_prefix("cd ") {
		if target_dir_string.starts_with(HOME_SYMBOL) {
		    let mut target_dir_path = PathBuf::new();
		    target_dir_path.push(env::home_dir().expect("User home directory is not set"));

		    let home_strip_string = format!("{HOME_SYMBOL}/");
		    if let Some(relative_path) = target_dir_string.strip_prefix(&home_strip_string) {
			target_dir_path.push(relative_path);
		    }
		    
		    Self::Cd(target_dir_path)
		} else {
		    let target_dir_path = Path::new(target_dir_string);
		    Self::Cd(target_dir_path.to_path_buf())
		}
		

	    } else if let Some(home) = env::home_dir() {
     		    Self::Cd(home)
     		} else {
     		    let mut root = PathBuf::new();
     		    root.push("/");
     		    Self::Cd(root)
     		}
	}

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

        // Executable    
        } else if let Some(executable) = find_executable(input.split(" ").next().unwrap()) {
            Self::Executable {
                path_to_executable: executable,
                input: input.to_string()
            }
        }

        // Not a builtin
        else {
            Self::NotFound(input.to_string())
        }
    }

    pub(crate) fn run_command(command: Self) {
        match command {
            Self::Exit => (),
            Self::Pwd => println!("{}", env::current_dir().unwrap().display()),
	    Self::Cd(x) => {
		if env::set_current_dir(x.as_path()).is_ok() {}
		else {println!("cd: {}: No such file or directory", x.display())}
	    },
            Self::Echo(x) => println!("{}", x),
            Self::Type(x) => {
                // Builtin takes priority
                if BUILTINS.contains(&x.as_str()) { println!("{} is a shell builtin", x) }

                // Match the path
                else if let Some(executable) = find_executable(&x) {
                    println!("{x} is {}", executable.display());	
                } else {
                    println!("{}: not found", x);
                }
            },
            Self::Executable { path_to_executable, input } => {
                let mut execution = process::Command::new(path_to_executable.as_path().file_stem().unwrap())
                    .args(input.split(" ").skip(1))
                    .spawn()
                    .expect("{path_to_executable} binary failed");
                execution.wait().unwrap();
            },
            Self::NotFound(x) => println!("{}: command not found", x),
        }
    }
}
