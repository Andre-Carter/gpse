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
        print!("RUST-RUST>");

        let mut command: String = String::new();

        read(&mut command);

        let command = command.trim();

        match command {
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
                println!("Exiting Rust-Rust");
                break;
            }

            "terminate" => {
                panic!("Terminated");
            }

            _ => {
                println!("UNKNOWN COMMAND");
            }
        }
    }

    println!();
}
