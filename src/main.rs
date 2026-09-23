use gpse::cli::commands::cli_commands;
use gpse::science::astrological::celestial::{EARTH, MOON};
use gpse::science::physical::equations::gravitational_force;
use gpse::date_time::time::physical_time;

fn main() {
    println!("WELCOME TO GPSE!");
    println!("[DATE/TIME]");

    println!(
        "{}",
        gravitational_force(EARTH.mass_kg, MOON.mass_kg, 384_400_000.0)
    );

    println!(
        "{}", physical_time(EARTH.velocity_m)
    );
    

    println!("Type \"help\" for commands.");
    cli_commands();

    //let g_force = gravitational_force(EARTH.mass_kg, MOON.mass_kg, 384_400_000.0);
    //println!("{g_force}");

    
}
