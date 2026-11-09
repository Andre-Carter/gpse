use crate::experimental::programs::checklist::checklist_cli;

use crate::experimental::foundation::gagets::switch::switch_cli;

use crate::foundation::terminate;
use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn cli_commands() {
    println!();
    println!("* EXPERIMENTAL * COMMAND SYSTEMS ONLINE!");
    println!("Enter \"help\" for commands.");
    println!();

    loop {
        print!("GPSE> COMMAND> EXPERIMENTAL>");

        let mut input: String = String::new();

        read(&mut input);

        let command = input.trim().to_lowercase();

        match command.as_str() {
            "help" => {
                println!();
                println!("GPSE COMMANDS:");
                println!("-------------------------------------------------");
                println!("{:<20} (command descriptions & instructions)", "help");
                println!("{:<20} (back directory)", "exit");
                println!("{:<20} (close gpse)", "terminate");
                println!();
            }

            "nested" => loop {
                print!("GPSE> COMMAND> READ> ");
                let mut command = String::new();
                read(&mut command);
                let command = command.trim().to_lowercase();
                match command.as_str() {
                    "help" => {
                        
                    }
                    "exit" => {
                        println!("exiting...");
                        break;
                    }
                    "terminate" => {
                        terminate();
                    }
                    _ => {
                        println!("unknown command")
                    }
                }
            },

            "checklist" => {
                checklist_cli();
            }

            "switch" => {
                switch_cli();
            }

            "exit" => {
                println!("Exiting GPSE COMMAND SYSTEM...");
                break;
            }

            "terminate" => {
                terminate();
            }

            _ => {
                println!("Unknown command.")
            }
        }
    }

    println!();
}