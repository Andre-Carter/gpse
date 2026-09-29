use crate::archive::bible::kjv1611::kjv_1611_cli::kjv_1611_cli_funk;
use crate::archive::ledger::ledger_cli::open_ledger;
use crate::cli::calculator::cli_calc;
use crate::cli::calculator::cli_sqrt;
use crate::mathematical::carter_family::carter_formula_cli;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn cli_commands() {
    println!("COMMAND SYSTEMS ONLINE!");
    println!("Type \"help\" for commands.");
    println!();

    loop {
        let mut command: String = String::new();

        print!("> ");

        read(&mut command);

        let command = command.trim().to_lowercase();

        match command.as_str() {
            "help" => {
                println!();
                println!("GPSE COMMANDS:");
                println!("-------------------------------------------------");
                println!("{:<20} (command descriptions & instructions)", "help");
                println!("{:<20} (open gpse read directory)", "read");
                println!("{:<20} (open ledger)", "ledger");
                println!("{:<20} (open cli calculator)", "calc");
                println!("{:<20} (open square-root function)", "sqrt");
                println!("{:<20} (open carter mathematics)", "carter");
                println!("{:<20} (debugging steps)", "debug");
                println!("{:<20} (inspection steps)", "inspection");
                println!("{:<20} (cargo commands listing)", "cargo");
                println!("{:<20} (git commands listing)", "git"); //pending removal
                println!("{:<20} (exit gpse cli program)", "exit");
            }

            "read" => loop {
                let mut command = String::new();

                print!("read> ");

                read(&mut command);

                let command = command.trim().to_lowercase();

                match command.as_str() {
                    "help" => {
                        println!("read> help> ");
                        println!();
                        println!("{:<20} (command descriptions & instructions)", "help");
                        println!("{:<20} (read directory list)", "library");
                        println!("{:<20} (exit the read directory)", "exit");
                    }

                    "library" => {
                        println!("read> library> ");
                        println!("{:<20} (open kjv 1611)", "kjv 1611");
                    }

                    "kjv 1611" => {
                        kjv_1611_cli_funk();
                    }

                    "exit" => {
                        println!("exiting the read directory...");
                        break;
                    }

                    _ => {
                        println!("  unknown read command.")
                    }
                }
            },

            "ledger" => {
                open_ledger();
            }

            "calc" => {
                cli_calc();
            }

            "sqrt" => {
                cli_sqrt();
            }

            "carter" => {
                carter_formula_cli();
            }

            "debug" => {
                println!(r"cargo: clean -> build -> run .\target\debug\gpse.exe")
            }

            "inspection" => {
                println!(
                    "cargo: fmt -> check -> test -> clippy -> git: diff -> status -> commit -> push"
                )
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

            "exit" => {
                println!("successfully exited GPSE...");
                break;
            }

            _ => {
                println!("Unknown command.")
            }
        }
    }
}
