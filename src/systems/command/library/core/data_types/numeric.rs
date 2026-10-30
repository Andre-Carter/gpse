use crate::core::data_types::numeric::USIZE_MAXIMUM_VALUE;
use crate::core::data_types::numeric::USIZE_MINIMUM_VALUE;

use crate::core::data_types::numeric::U32_MAXIMUM_VALUE;
use crate::core::data_types::numeric::U32_MINIMUM_VALUE;

use crate::core::data_types::numeric::U64_MAXIMUM_VALUE;
use crate::core::data_types::numeric::U64_MINIMUM_VALUE;

use crate::core::data_types::numeric::I32_MAXIMUM_VALUE;
use crate::core::data_types::numeric::I32_MINIMUM_VALUE;

use crate::core::data_types::numeric::I64_MAXIMUM_VALUE;
use crate::core::data_types::numeric::I64_MINIMUM_VALUE;

use crate::core::data_types::numeric::F32_MAXIMUM_VALUE;
use crate::core::data_types::numeric::F32_MINIMUM_VALUE;

use crate::core::data_types::numeric::F64_MAXIMUM_VALUE;
use crate::core::data_types::numeric::F64_MINIMUM_VALUE;

use crate::core::data_types::numeric::F32_MINIMUM_POSITIVE_NORMAL_VALUE;
use crate::core::data_types::numeric::F64_MAXIMUM_POSITIVE_NORMAL_VALUE;

use crate::core::data_types::numeric::Numeric;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn print_numeric() {
    println!("NUMERIC DATA TYPE MINIMUM MAXIMUM VALUES");
    println!();

    println!("usize minimum value: {}", USIZE_MINIMUM_VALUE);
    println!("usize maximum value: {}", USIZE_MAXIMUM_VALUE);

    println!("u32 minimum value: {}", U32_MINIMUM_VALUE);
    println!("u32 maximum value: {}", U32_MAXIMUM_VALUE);

    println!("u64 minimum value: {}", U64_MINIMUM_VALUE);
    println!("u64 minimum value: {}", U64_MAXIMUM_VALUE);

    println!("i32 minimum value: {}", I32_MINIMUM_VALUE);
    println!("i32 maximum value: {}", I32_MAXIMUM_VALUE);

    println!("i64 maximum value: {}", I64_MINIMUM_VALUE);
    println!("i64 minimum value: {}", I64_MAXIMUM_VALUE);

    println!("f32 minimum value: {}", F32_MINIMUM_VALUE);
    println!("f32 maximum value: {}", F32_MAXIMUM_VALUE);

    println!("f64 minimum value: {}", F64_MINIMUM_VALUE);
    println!("f64 maximum value: {}", F64_MAXIMUM_VALUE);

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
        print!("GPSE> COMMAND> ");

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