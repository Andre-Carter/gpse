use gpse::programs::date_time::time::seconds_since_unix_epoc;

use gpse::systems::command::kernel::command_core::cli_commands;

use gpse::foundation::data_types::numeric::NumericType;

use gpse::mathematical::arithmetic::arithmetic::add_u8;

use gpse::lang::alphabet;

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

    //

    alphabet();

    // End of Tests (Implementation Phase) 

    // Refactor Test to systems::command::core

    print!("GREETINGS, WELCOME BACK! ");
    println!("[ DATE / {:?} ]", seconds_since_unix_epoc());
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
