use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn cli_calc() {
    println!("* GPSE CLI Calculator *");

    let mut operand_1: String = String::new();
    let mut operand_2: String = String::new();
    let mut operator: String = String::new();

    print!("Operand 1: ");
    read(&mut operand_1);

    let operand_1 = operand_1.trim().replace('_', "").replace(',', "");

    let operand_1: f64 = match operand_1.parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid Operand 1.");
            return;
        }
    };

    //MATCH OPERAND 1
    //CHECK FOR CONSTANT, DO MATH WITH CONSTANTS

    print!(
        "Enter Operator | (+)(-)(*)(/)(^)(%) or \"add\", \"subtract\", \"multiply\", \"divide\", \"power\", \"modulo\":"
    );
    read(&mut operator);

    print!("Operand 2: ");
    read(&mut operand_2);

    let operand_2 = operand_2.trim().replace('_', "").replace(',', "");
    let operand_2: f64 = match operand_2.parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid Operand 2.");
            return;
        }
    };

    //MATCH OPERAND 2

    let operator = operator.trim().to_lowercase();

    let solution = match operator.as_str() {
        "+" | "add" => operand_1 + operand_2,
        "-" | "subtract" => operand_1 - operand_2,
        "*" | "multiply" => operand_1 * operand_2,
        "/" | "divide" => operand_1 / operand_2,
        "^" | "power" => operand_1.powf(operand_2),
        "%" | "modulo" => operand_1 % operand_2,
        _ => {
            println!("Unknown operator.");
            return;
        }
    };

    println!(
        "Solution: {} {} {} = {} <---",
        operand_1, operator, operand_2, solution
    );
}
