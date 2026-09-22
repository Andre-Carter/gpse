use gpse::cli::commands::cli_commands;
use gpse::entities::celestial::{EARTH, MOON};
use gpse::physical::equations::gravitational_force;

fn main() {
    println!("WELCOME TO GPSE!");
    println!("DATE/TIME");
    println!("Type \"help\" for commands.");
    cli_commands();

    let g_force = gravitational_force(EARTH.mass_kg, MOON.mass_kg, 384_400_000.0);
    println!("{g_force}");
}
