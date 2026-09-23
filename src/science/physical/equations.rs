use crate::science::physical::constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;

pub fn gravitational_force(mass_1: f64, mass_2: f64, distance: f64) -> f64 {
    let _g = NEWTONIAN_CONSTANT_OF_GRAVITATION.value;

    let g_force = _g * mass_1 * mass_2 / distance.powi(2);

    g_force
}
