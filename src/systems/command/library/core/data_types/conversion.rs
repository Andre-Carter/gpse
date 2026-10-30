use crate::core::data_types::conversion::convert_to_u32;


//use crate::core::data_types::conversion::f32_to_f64;
//use crate::core::data_types::conversion::i32_to_i64;
use crate::core::data_types::conversion::u32_to_u64;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

// THIS IS A USELESS CLI!!!! whats the point of printing the same number?

pub fn conversion_cli() {
    loop {
        print!("GPSE> COMMAND> DATA_TYPES> CONVERSION> ");

        let mut command: String = String::new();

        read(&mut command);

        let command = command.trim();

        match command {
            "u32 to u64" => {
                let mut command: String = String::new();
                
                print!("Enter a u32: ");
                read(&mut command);

                let command = command.trim();

                match convert_to_u32(command) {
                    Ok(value) => {
                        let output = u32_to_u64(value);
                        println!("Output: {output}");
                    }

                    Err(error) => {
                        println!("Conversion failed: {error}");
                    }
                }
            }

            "i32 to i64" => {
                
            }

            "f32 to f64" => {
                
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