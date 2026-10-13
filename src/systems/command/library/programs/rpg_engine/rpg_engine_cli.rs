use crate::programs::rpg_engine::gamestate::GameState;
use crate::programs::rpg_engine::player::player::player_set_name;
use crate::systems::command::library::programs::rpg_engine::player::player_cli::print_player;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn rpg_engine_cli() {
    let mut state = GameState::new();
    
    println!();
    println!("RPG CLI!");
    println!("Enter \"help\" for commands.");
    println!();

    loop {
        print!("GPSE> COMMAND> RPG> ");

        let mut command: String = String::new();

        read(&mut command);

        let command = command.trim().to_lowercase();

        match command.as_str() {
            "help" => {
                println!();
                println!("RPG COMMANDS:");
                println!("-------------------------------------------------");
                println!("{:<20} (command descriptions & instructions)", "help");
                println!("{:<20} (read and modify player)", "player");
                println!("{:<20} (exit gpse cli program)", "exit");
                println!();
            }

            "player" => {
                print!("GPSE> COMMAND> RPG> PLAYER> ");

                let mut command: String = String::new();

                read(&mut command);

                let command = command.trim().to_lowercase();

                match command.as_str() {
                    "help" => {
                        println!();
                        println!("RPG COMMANDS:");
                        println!("-------------------------------------------------");
                        println!("{:<20} (command descriptions & instructions)", "help");
                        println!("{:<20} (get player information)", "print");
                        println!("{:<20} (give player a new name)", "rename");
                        println!("{:<20} (exit gpse cli program)", "exit");
                        println!();
                    }

                    "print" => {
                        print_player(&state);
                    }

                    "rename" => {
                        let mut command: String = String::new();

                        print!("Enter new player name: ");
                        read(&mut command);

                        let command = command.trim();

                        player_set_name(&mut state, command);
                    }


                    "exit" => {
                        println!("Exiting RPG Player...");
                        break;
                    }

                    _ => {
                        println!("Unknow RPG Player command.")
                    }
                }
            }

            "exit" => {
                println!("Exiting RPG...");
                break;
            }

            _ => {
                println!("Unknown RPG command.");
            }
        }
    }
}