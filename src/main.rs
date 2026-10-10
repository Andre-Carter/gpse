use gpse::systems::cli::commands::cli_commands;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    print!("GREETINGS, WELCOME BACK! ");
    println!("[ DATE / TIME ]");
    print!("GENERAL PURPOSE SIMULATION ENGINE ");
    println!("[ VERSION: 0.0.1 ]");
    println!("Enter \"command\" to start.");
    println!();

    loop {
        print!("GPSE> ");

        let mut command: String = String::new();

        read(&mut command);

        let command = command.trim().to_lowercase();

        match command.as_str() {
            "command" => {
                cli_commands();
            }

            "exit" => {
                println!("Exiting GPSE...");
                println!("BLESSED!");
                break;
            }

            _ => {
                println!("unknown command.");
            }
        }
    }

    println!();
}
