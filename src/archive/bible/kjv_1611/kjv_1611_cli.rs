use crate::archive::bible::kjv_1611::kjv_1611::lookup;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn kjv_1611_cli_funk() {
    loop {
        let mut command = String::new();

        read(&mut command);

        let command = command.trim();

        match command {
            "help" => {
                println!("Bible commands.");
            }

            "exit" => {
                println!("Jesus loves you.");
                break;
            }

            _ => {
                let parts: Vec<&str> = command.split_whitespace().collect();

                if parts.len() != 2 {
                    println!("Invalid reference.");
                }

                let book = parts[0];

                let reference: Vec<&str> = parts[1].split(':').collect();

                if reference.len() != 2 {
                    println!("Invalid reference. Use chapter:verse.");
                    continue;
                }

                let chapter: u8 = match reference[0].parse() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Invalid chapter.");
                        continue;
                    }
                };

                let verse: u8 = match reference[1].parse() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Invalid chapter.");
                        continue;
                    }
                };

                let result = lookup(book, chapter, verse);

                match result {
                    Some(verse) => println!("{}", verse),
                    None => println!("Verse not found."),
                }
            } //END OF _ => PARSE
        } //END OF MATCH COMMAND
    } //END OF LOOP
} //END OF BIBLE CLI FUNCTION
