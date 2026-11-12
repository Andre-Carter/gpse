//use crate::mathematical::arithmetic::arithmetic::checked_add;
use crate::mathematical::sequences::sequence::sequence_cli;
use crate::experimental::mathematical::test_mathematical::test_mathematical;

#[cfg(feature = "marvel-rivals")]
use crate::programs::library::marvel_rivals_random_hero::marvel_rivals_random_hero;
use crate::systems::command::library::archive::bible::kjv1611::cli::kjv_1611_interface;
use crate::systems::command::library::core::data_types::data_types_cli::data_types_cli;
use crate::systems::command::tools::calculator::cli_calc;
use crate::systems::command::tools::calculator::cli_sqrt;
use crate::systems::command::tools::math::math_cli;

use crate::science::physical::cli::print_constants;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn command_system() {
    println!();
    println!("COMMAND SYSTEMS ONLINE!\n");
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
                println!("{:<20} (close the program)", "terminate");
                println!("");
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

                    "1611 kjv" => {
                        kjv_1611_interface();
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

            "calc" => {
                cli_calc();
            }

            "all physical constants" => {
                print_constants();
            }

            "data types" => {
                data_types_cli();
            }

            "sqrt" => {
                cli_sqrt();
            }

            "math" => {
                math_cli();
            }

            "test mathematical" => {
                test_mathematical();
            }

            "sequence" => {
                sequence_cli();
            }

            #[cfg(feature = "marvel-rivals")]
            "random hero" => {
                marvel_rivals_random_hero();
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
