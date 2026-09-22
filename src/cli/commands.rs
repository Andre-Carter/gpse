use crate::bible::kjv_1611_cli::kjv_1611_cli;
use crate::cli::calculator::cli_calc;
use crate::cli::calculator::cli_sqrt;
use crate::cli::ledger::open_ledger;
use crate::mathematical::carter_family::carter_formula_cli;
use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn cli_commands() {
    println!("Commands system online!");

    loop {
        let mut command: String = String::new();

        print!("> ");
        read(&mut command);

        let command = command.trim();

        match command {
            "help" => {
                println!("GPSE COMMANDS:");
                println!("-------------------------------------------------");
                println!("{:<20} Open Bible.", "bible");
                println!("{:<20} Open Calculator.", "calc");
                println!("{:<20} Open square-root function.", "sqrt");
                println!("{:<20} Open Carter mathematics.", "carter");
                println!("{:<20} Open Ledger.", "ledger");
                println!("{:<20} Cargo commands.", "cargo");
                println!("{:<20} Git commands", "git");
                println!("{:<20} Inspection steps.", "inspection ritual");
                println!("{:<20} Debugging steps", "debugging ritual");
                println!("{:<20} Exit GPSE.", "exit");
            }

            "read" => loop {
                let mut command = String::new();

                print!("READ> ");

                read(&mut command);

                let command = command.trim();

                match command {
                    "kjv 1611" => {
                        print!("READ> KJV 1611> ");
                        kjv_1611_cli();
                    }

                    "help" => {
                        println!("READ commands:");
                        println!("kjv 1611");
                        println!("exit");
                    }

                    "exit" => {
                        break;
                    }

                    _ => {
                        println!("Unknown READ command.")
                    }
                }
            },

            "calc" => {
                cli_calc();
            }

            "sqrt" => {
                cli_sqrt();
            }

            "ledger" => {
                open_ledger();
            }

            "carter" => {
                carter_formula_cli();
            }

            "cargo" => {
                println!("cargo build");
                println!("cargo build --release");
                println!("cargo check");
                println!("cargo clean");
                println!("cargo clippy");
                println!("cargo clippy -- -D warnings");
                println!("cargo doc");
                println!("cargo doc --open");
                println!("cargo fmt");
                println!("cargo fmt -- --check");
                println!("cargo new");
                println!("cargo run");
                println!("cargo run --release");
                println!("cargo test");
                println!("cargo test --nocapture");
                println!("cargo tree");
            }

            "git" => {
                println!("git add .");
                println!("git add src/\"cli/calculator.rs\"");
                println!("git branch");
                println!("git branch -a");
                println!("git commit -m \"your message\"");
                println!("git diff");
                println!("git diff --staged");
                println!("git fetch");
                println!("git log");
                println!("git log --oneline");
                println!("git merge");
                println!("git push");
                println!("git pull");
                println!("git tag");
                println!("git remote -v");
                println!("git restore");
                println!("git stash");
                println!("git switch");
            }

            "debug" => {
                println!(r"cargo: clean -> build -> run .\target\debug\gpse.exe")
            }

            "inspection" => {
                println!(
                    "cargo: fmt -> check -> test -> clippy -> git: diff -> status -> commit -> push"
                )
            }

            "exit" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Unknown command.")
            }
        }
    }
}
