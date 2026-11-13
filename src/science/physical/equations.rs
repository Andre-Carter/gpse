use crate::science::physical::constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;
//use crate::science::astrological::celestial::{EARTH, MOON};

pub fn gravitational_force(mass_1: f64, mass_2: f64, distance: f64) -> f64 {
    let g = NEWTONIAN_CONSTANT_OF_GRAVITATION.value;
    g * mass_1 * mass_2 / distance.powi(2)
}
