use crate::foundation::terminate;

use crate::experimental::the_forge::test_suite;

use crate::experimental::foundation::gagets::switch::switch_cli;
use crate::experimental::ledger::ledger_cli::open_ledger;
use crate::experimental::programs::checklist::checklist_cli;
use crate::experimental::mathematical::carter::carter_family::carter_formula_cli;
use crate::experimental::mathematical::carter::carter_family::carter_formula_repeat;

use crate::experimental::systems::command::library::rpg_engine::rpg_engine_cli::rpg_engine_cli;

use crate::experimental::rust_rust::cli::rust_rust_interface;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn experimental_command_system() {
    println!();
    println!("*** EXPERIMENTAL *** COMMAND SYSTEMS ONLINE!\n");
    println!("Enter \"help\" for commands.");
    println!();

    loop {
        print!("GPSE> EXPERIMENTAL> ");

        let mut input: String = String::new();

        read(&mut input);

        let command = input.trim().to_lowercase();

        match command.as_str() {
            "help" => {
                println!("\nGPSE COMMANDS:");
                println!("-------------------------------------------------");
                println!("{:<20} (command descriptions & instructions)", "help");
                println!("{:<20} (ignite the forge, initiate test suite)", "test");
                println!("{:<20} (back directory)", "exit");
                println!("{:<20} (close program)", "terminate");
                println!("");
            }

            "test" => {
                test_suite();
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

            "rust-rust" => {
                rust_rust_interface();
            }

            "exit" => {
                println!("\nGOING HOME\n");
                break;
            }

            "terminate" => {
                terminate();
            }

            _ => {
                println!("\nUNKNOWN COMMAND.\n")
            }
        }
    }

    println!();
}