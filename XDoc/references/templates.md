use std::io::{Write, stdin, stdout};

fn read(cli_input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(cli_input).expect("failed to read");
}

pub fn arithmetic_cli() {
    let mut cli_input = String::new();

    read(&mut cli_input);
}