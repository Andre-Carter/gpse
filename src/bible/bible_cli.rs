use crate::bible::kjv_1611::GENESIS_001_001;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}
//kjv 1611 genesis(1,1)
pub fn bible_cli() {
    println!("{}", GENESIS_001_001);

    let mut bible: String = String::new();
    read(&mut bible);

    // let match
}
