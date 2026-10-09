use crate::archive::bible::kjv1611::kjv_1611_cli::kjv_1611_cli_funk;

use crate::archive::ledger::ledger_cli::open_ledger;

use crate::cli::calculator::cli_calc;
use crate::cli::calculator::cli_sqrt;
use crate::cli::math::math_cli;

use crate::mathematical::carter::carter_family::carter_formula_cli;
use crate::mathematical::carter::carter_family::carter_formula_repeat;
use crate::mathematical::sequences::sequence::sequence_cli;

use crate::programs::games::marvel_rivals_random_hero::marvel_rivals_random_hero;

use crate::programs::rpg_engine::player::inventory::Item;
use crate::programs::rpg_engine::player::player::player;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn captialize_first_letter(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

pub fn cli_commands() {
    println!();
    println!("COMMAND SYSTEMS ONLINE!");
    println!("Enter \"help\" for commands.");
    println!();

    loop {
        print!("GPSE> COMMAND> ");

        let mut command: String = String::new();

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
                println!("{:<20} (open carter mathematics repeat)", "carter repeat");
                println!("{:<20} (open rpg)", "rpg");
                println!("{:<20} (debugging steps)", "debug");
                println!("{:<20} (inspection steps)", "inspection");
                println!("{:<20} (cargo commands listing)", "cargo");
                println!("{:<20} (git commands listing)", "git"); //pending removal
                println!("{:<20} (exit gpse cli program)", "exit");
                println!();
            }

            "read" => loop {
                let mut command = String::new();

                print!("GPSE> COMMAND> READ> ");

                read(&mut command);

                let command = command.trim().to_lowercase();

                match command.as_str() {
                    "help" => {
                        println!("GPSE> COMMAND> READ> HELP> ");
                        println!();
                        println!("{:<20} (command descriptions & instructions)", "help");
                        println!("{:<20} (read directory list)", "library");
                        println!("{:<20} (exit the read directory)", "exit");
                    }

                    "library" => {
                        println!("GPSE> COMMAND> READ> LIBRARY> ");
                        println!("{:<20} (open kjv 1611)", "kjv1611");
                    }

                    "kjv1611" => {
                        kjv_1611_cli_funk();
                    }

                    "exit" => {
                        println!("exiting the read directory...");
                        break;
                    }

                    _ => {
                        println!("unknown read command.")
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

            "carter repeat" => {
                carter_formula_repeat();
            }

            "math" => {
                math_cli();
            }

            "sequence" => {
                sequence_cli();
            }

            "random hero" => {
                marvel_rivals_random_hero();
            }

            "rpg" => {
                player::<Item>();
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

    println!();
}
