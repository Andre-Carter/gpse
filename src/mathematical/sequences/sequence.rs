use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn sequence_cli() {
    println!("[ COEFFICIENT START END ]");
    print!("GPSE> MATHEMATICAL> SEQUENCE> "); // NOTE: FUTURE REFACTOR OF CLI TREE

    let mut input = String::new();

    read(&mut input);

    let sequence_code: Vec<_> = input
        .trim()
        .split_whitespace()
        .collect();

    let coefficient = sequence_code[0].parse::<i32>().unwrap();
    let start = sequence_code[1].parse::<i32>().unwrap();
    let end = sequence_code[2].parse::<i32>().unwrap();
    println!("[ {} {} {} ]", coefficient, start, end);
    let _ = sequence_linear_i32(coefficient, start, end);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SequenceError {
    InvalidRange,
    ArithmeticOverflow,
}

pub fn sequence_linear_i32(
    coefficient: i32,
    start: i32, 
    end: i32
) -> Result<Vec<i32>, SequenceError> {
    let mut generated_sequence: Vec<i32> = Vec::new();

    if start > end {
        return Err(SequenceError::InvalidRange);
    }

    for n in start..=end {
        let generation = n + coefficient;

        generated_sequence.push(generation);
    }

    Ok(generated_sequence)
}

/*
pub fn sequence_linear_i32(
    coefficient: i32,
    start: i32,
    end: i32,
) -> Result<Vec<i32>, SequenceError> {
    let mut generated_sequence: Vec<i32> = Vec::new();

    if start > end {
        return Err(SequenceError::InvalidRange);
    }

    for n in start..=end {
        let generation = n
            .checked_add(coefficient)
            .ok_or(SequenceError::ArithmeticOverflow)?;

        generated_sequence.push(generation);
    }

    Ok(generated_sequence)
}
*/


