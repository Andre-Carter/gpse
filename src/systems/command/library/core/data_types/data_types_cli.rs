
use crate::systems::command::library::core::data_types::conversion::conversion_cli;
use crate::systems::command::library::core::data_types::numeric::numeric_cli;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn data_types_cli() {
    loop {
        print!("GPSE> COMMAND> DATA_TYPES> ");

        let mut command: String = String::new();

        read(&mut command);

        let command = command.trim().to_lowercase();

        match command.as_str() {
            "conversion" => {
                conversion_cli();
            }

            "numeric" => {
                numeric_cli();
            }


            "exit" => {
                println!("exiting the numeric...");
                break;
            }

            _ => {
                println!("unknown numeric command.");
            }
        }
    }
}