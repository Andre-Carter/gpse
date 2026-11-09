use crate::foundation::terminate;

use crate::experimental::foundation::gagets::switch::switch_cli;
use crate::experimental::ledger::ledger_cli::open_ledger;
use crate::experimental::programs::checklist::checklist_cli;
use crate::experimental::mathematical::carter::carter_family::carter_formula_cli;
use crate::experimental::mathematical::carter::carter_family::carter_formula_repeat;

use crate::experimental::systems::command::library::rpg_engine::rpg_engine_cli::rpg_engine_cli;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn cli_commands() {
    println!();
    println!("* EXPERIMENTAL * COMMAND SYSTEMS ONLINE!");
    println!("Enter \"help\" for commands.");
    println!();

    loop {
        print!("GPSE> COMMAND> EXPERIMENTAL>");

        let mut input: String = String::new();

        read(&mut input);

        let command = input.trim().to_lowercase();

        match command.as_str() {
            "help" => {
                println!();
                println!("GPSE COMMANDS:");
                println!("-------------------------------------------------");
                println!("{:<20} (command descriptions & instructions)", "help");
                println!("{:<20} (back directory)", "exit");
                println!("{:<20} (close gpse)", "terminate");
                println!();
            }

            "carter" => {
                carter_formula_cli();
            }

            "carter repeat" => {
                carter_formula_repeat();
            }

            "checklist" => {
                checklist_cli();
            }

            "ledger" => {
                open_ledger();
            }

            "switch" => {
                switch_cli();
            }

            "rpg" => {
                rpg_engine_cli();
            }

            "exit" => {
                println!("Exiting GPSE COMMAND SYSTEM...");
                break;
            }

            "terminate" => {
                terminate();
            }

            _ => {
                println!("Unknown command.")
            }
        }
    }

    println!();
}