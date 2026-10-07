use gpse::cli::commands::cli_commands;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    //MAIN SYSTEMS
    loop {
        println!("GENERAL PURPOSE SIMULATION ENGINE");
        println!("[ DATE / TIME ]");
        println!();

        let mut command: String = String::new();

        print!("> ");

        read(&mut command);

        let command = command.trim().to_lowercase();

        match command.as_str() {
            "command" => {
                cli_commands();
            }

            "exit" => {
                println!("exiting gpse ...");
                break;
            }

            _ => {
                println!("unknown command.");
            }
        }
    }
}
