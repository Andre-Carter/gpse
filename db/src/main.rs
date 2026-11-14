use db::cli::command_system;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    loop {
        print!("GPSE> DB>");

        let mut command: String = String::new();

        read(&mut command);

        let command = command.trim().to_lowercase();

        match command.as_str() {
            "help" => {
                println!("\n{:<20} (navigate the command system)", "command");
                println!("{:<20} (navigate experimental systems)", "experimental");
                println!("{:<20} (you will exit the program here)", "exit");
                println!("{:<20} (close program)", "terminate");
                println!();
            }

            "command" => {
                command_system();
            }

            "exit" => {
                println!("\nYOU HAVE LEFT THE SYSTEM");
                break;
            }
            // If you want a clean shutdown instead of a panic, std::process::exit(0); leaves without the panic message and without exit code 101. panic! is fine as a stand-in while you build.
            "terminate" => {
                panic!("\nSYSTEM TERMINATED!\n");
            }

            _ => {
                println!("\nUNKNOWN COMMAND.\n");
            }
        }
    }
}

