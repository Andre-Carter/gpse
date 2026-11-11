pub mod dispatch;

use crate::foundation::input::read_text;

use std::io::{self, Write};

pub fn confirm() -> bool {
    loop {
        print!("CONFIRM: YES / NO > ");
        io::stdout().flush().unwrap();

        match read_text().trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Type yes or no."),
        }
    }
}
