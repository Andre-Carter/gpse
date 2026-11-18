
use std::io::{self, Write};

fn main() {
    println!("================================");
    println!("       GPSE TERMINAL");
    println!("================================");

    loop {
        print!("GPSE > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(error) => {
                println!("Input error: {error}");
                continue;
            }
        }

        let command = input.trim();

        match command {
            "hello" => println!("Greetings, operator."),
            "help" => println!("Commands: hello, help, exit"),
            "exit" => break,
            "" => continue,
            _ => println!("Unknown command: {command}"),
        }
    }

    println!("GPSE TERMINAL CLOSED.");
}
