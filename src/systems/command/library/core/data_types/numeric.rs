use crate::core::data_types::numeric::Numeric;

use crate::core::data_types::numeric::F32_MINIMUM_POSITIVE_NORMAL_VALUE;
use crate::core::data_types::numeric::F64_MAXIMUM_POSITIVE_NORMAL_VALUE;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn positive_normal_values() {
    println!(
        "f32 minimum positive normal value: {}",
        F32_MINIMUM_POSITIVE_NORMAL_VALUE
    );
    println!(
        "f64 minimum positive normal value: {}",
        F64_MAXIMUM_POSITIVE_NORMAL_VALUE
    );

    println!();
}

pub fn numeric_cli() {
    loop {
        print!("GPSE> COMMAND> NUMERIC> ");

        let mut command: String = String::new();

        read(&mut command);

        let command = command.trim().to_lowercase();

        match command.as_str() {
            "minimum values" => {
                Numeric::minimum_values();
            }

            "maximum values" => {
                Numeric::maximum_values();
            }

            "positive normal values" => {
                positive_normal_values();
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