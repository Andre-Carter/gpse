use gpse::bible::kjv_1611::test_bible;
use gpse::cli::commands::cli_commands;
use gpse::entities::celestial::{EARTH, MOON};
use gpse::mathematical::expressions::expressions_test;
use gpse::mathematical::geometry::area_square;
use gpse::mathematical::geometry::formula_area;
use gpse::mathematical::geometry::formula_volume;
use gpse::physical::equations::gravitational_force;


fn main() {
    test_bible();

    let g_force = gravitational_force(EARTH.mass_kg, MOON.mass_kg, 384_400_000.0);

    println!("{g_force}");

    expressions_test(27.0, 15.0);

    formula_area(27.0, 15.0);

    formula_volume(3.0, 4.0, 5.0);

    area_square(8.0);

    //for n in 0..=1 {
    //    let n = n as f64;
    //    let result = carter_constant(n);
    //    println!("n = {n}, CT = {result}");
    //}

    println!("Type \"help\" for commands.");
    cli_commands();
}
