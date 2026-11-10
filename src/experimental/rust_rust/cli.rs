//use crate::foundation::TERMINATED;
//use crate::foundation::TERMINATION;
use crate::foundation::terminate;
use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn rust_rust_interface() {
    println!();
    println!("WELCOME TO RUST-RUST!");
    println!("Enter \"help\" for commands.");
    println!();

    loop {
        print!("GPSE> COMMAND> RUST RUST>");

        let mut command: String = String::new();

        read(&mut command);

        let command = command.trim().to_lowercase();

        match command.as_str() {
            "help" => {
                println!();
                println!("GPSE COMMANDS:");
                println!("-------------------------------------------------");
                println!("{:<20} (command descriptions & instructions)", "help");
                
                println!("{:<20} (exit gpse cli program)", "exit");
                println!("{:<20} (SHUTDOWN GPSE PROGRAM)", "terminate");
                println!();
            }

            

            "exit" => {
                println!("Exiting GPSE COMMAND SYSTEM...");
                break;
            }

            "terminate" => {
                terminate();
            }

            _ => {
                println!("UNKNOWN COMMAND");
            }
        }
    }

    println!();
}
