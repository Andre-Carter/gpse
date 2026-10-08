use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn sequence_cli() {
    println!("[ START-NUMBER END-NUMBER VARIABLE ]");
    print!("GPSE> MATHEMATICAL> SEQUENCE> "); // NOTE: FUTURE REFACTOR OF CLI TREE

    let mut input = String::new();

    read(&mut input);

    let sequence_code: Vec<_> = input
        .trim()
        .split_whitespace()
        .collect();

    let start_number = sequence_code[0].parse::<usize>().unwrap();

    let end_number = sequence_code[1].parse::<usize>().unwrap();

    let variable = sequence_code[2].parse::<usize>().unwrap();

    println!("[ {} {} {} ]", start_number, end_number, variable);

    build_sequence(start_number, end_number, variable);
}

pub fn build_sequence(
    start_number: usize, 
    end_number: usize, 
    variable: usize
) {
    let mut new_sequence: Vec<usize> = Vec::new();

    for n in start_number..=end_number {
        let generator = n * variable;

        new_sequence.push(generator);
    }

    println!("{:?}", new_sequence);
}