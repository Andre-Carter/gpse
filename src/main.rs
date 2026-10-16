use gpse::systems::command::core::command_core::cli_commands;
use gpse::mathematical::data_types::numeric::u32_to_u64;
use gpse::mathematical::data_types::numeric::i32_to_i64;
use gpse::mathematical::data_types::numeric::f32_to_f64;

//use gpse::mathematical::data_types::numeric::NumericType;
use gpse::mathematical::data_types::numeric::Numeric;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    // Start of Tests
    let unsigned = u32_to_u64(100);
    let integer = i32_to_i64(100);
    let floater = f32_to_f64(100.0);

    println!("{}", unsigned);
    println!("{}", integer);
    println!("{}", floater);

    println!();

    Numeric::minimum_values();

    println!();

    Numeric::maximum_values();

    println!();
    // End of Tests

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
