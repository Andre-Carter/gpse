use crate::science::physical::constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;
//use crate::science::astrological::celestial::{EARTH, MOON};

pub fn gravitational_force(mass_1: f64, mass_2: f64, distance: f64) -> f64 {
    let _g = NEWTONIAN_CONSTANT_OF_GRAVITATION.value;
    let g_force = _g * mass_1 * mass_2 / distance.powi(2);

    g_force

    //let earth_moon_g: f64 = 
        //gravitational_force(EARTH.mass_kg, MOON.mass_kg, 384_400_000.0);
    
    //println!("{}", earth_moon_g);
}
