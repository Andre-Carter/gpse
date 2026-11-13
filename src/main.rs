// IMPORTS

// COMMAND SYSTEMS
use gpse::experimental::systems::command::kernal::dispatch::experimental_command_system;
use gpse::systems::command::kernel::confirm;
use gpse::systems::command::kernel::dispatch::command_system;

// TEST MODULES
use gpse::foundation::data_types::numeric::NumericType;
use gpse::mathematical::arithmetic::ops::add_u8;
use gpse::programs::date_time::time::seconds_since_unix_epoc;

// STANDARD LIBRARY
use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    // Start of Tests (Implementation Phase)

    println!("RANGE");
    println!("{:?}", NumericType::U32.range());

    println!();

    println!("{:?}", add_u8(u8::MAX, 1));

    // End of Tests (Implementation Phase)

    // Refactor Test to systems::command::core

    println!("\nGREETINGS, WELCOME BACK!\n");
    println!("[ DATE / {:?} ]", seconds_since_unix_epoc());
    print!("GENERAL PURPOSE SIMULATION ENGINE ");
    println!("[ VERSION: 0.0.1 ]");
    println!("\nEnter \"help\" for commands.\n");

    loop {
        print!("GPSE> ");

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

            "experimental" => {
                experimental_command_system();
            }

            "exit" => {
                println!("\nYOU HAVE LEFT THE SYSTEM");
                break;
            }
            // If you want a clean shutdown instead of a panic, std::process::exit(0); leaves without the panic message and without exit code 101. panic! is fine as a stand-in while you build.
            "terminate" => {
                if confirm() {
                    panic!("\nSYSTEM TERMINATED!\n");
                }
            }

            _ => {
                println!("\nUNKNOWN COMMAND.\n");
            }
        }
    }

    println!();
}
