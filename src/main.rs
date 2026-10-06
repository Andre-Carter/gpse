use gpse::cli::commands::cli_commands;
use gpse::date_time::time::physical_time;
use gpse::mathematical::carter::carter_family::carter_formula_repeat;
use gpse::mathematical::x_bash::num_x;
use gpse::mathematical::x_bash::x_power;
use gpse::rpg_engine::rpg::character;
use gpse::science::astrological::celestial::{EARTH, MOON};
use gpse::science::physical::equations::gravitational_force;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    carter_formula_repeat();

    println!();

    println!(
        "{}",
        gravitational_force(EARTH.mass_kg, MOON.mass_kg, 384_400_000.0)
    );

    println!("{}", physical_time(EARTH.velocity_m));

    println!("{}", num_x(10.0, 2.0));
    println!("{}", x_power(10.0, 2));

    //rpg

    character();

    println!();

    //MAIN SYSTEMS
    loop {
        println!("GENERAL PURPOSE SIMULATION ENGINE");
        println!("[ DATE / TIME ]");
        println!();

        let mut command: String = String::new();

        print!("> ");

        read(&mut command);

        let command = command.trim().to_lowercase();

        match command.as_str() {
            "command" => {
                cli_commands();
            }

            "exit" => {
                println!("exiting gpse ...");
                break;
            }

            _ => {
                println!("unknown command.");
            }
        }
    }
}
