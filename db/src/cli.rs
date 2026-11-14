use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn command_system() {
    println!();
    println!("COMMAND SYSTEM\n");
    println!("Enter \"help\" for commands.");
    println!();

    loop {
        print!("GPSE> COMMAND> ");

        let mut command: String = String::new();

        read(&mut command);

        let command = command.trim().to_lowercase();

        match command.as_str() {
            "help" => {
                println!("\nCOMMANDS:");
                println!("-------------------------------------------------");
                println!("{:<20} (command descriptions & instructions)", "help");
           
                println!("{:<20} (exit gpse cli program)", "exit");
                println!("{:<20} (close the program)", "terminate");
                println!();
            }

            "db 001" => {

            }

            "db 002" => {

            }

            "db 003" => {
                
            }

            "exit" => {
                println!("\nGOING HOME\n");
                break;
            }

            "terminate" => {
                panic!("\nSYSTEM TERMINATED!\n")
            }

            _ => {
                println!("\nUNKNOW COMMAND.\n")
            }
        }
    }

    println!();
}