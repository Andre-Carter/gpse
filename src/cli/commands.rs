use crate::cli::calculator::cli_calc;
use crate::cli::calculator::cli_sqrt;
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
                println!("GPSE CLI COMMANDS:");
                println!("calc");
                println!("cargo");
                println!("git");
                println!("machine-spirit inspection ritual");
                println!("machine-spirit debug ritual");
                println!("end");
            }

            "calc" => {
                cli_calc();
            }

            "sqrt" => {
                cli_sqrt();
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

            "machine-spirit inspection ritual" => {
                println!(
                    "cargo: fmt -> check -> test -> clippy -> git: diff -> status -> commit -> push"
                )
            }

            "machine-spirit debug ritual" => {
                println!(r"cargo: clean -> build -> run .\target\debug\gpse.exe")
            }

            "end" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Unknown command.")
            }
        }
    }
}
