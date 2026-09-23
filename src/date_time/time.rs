use crate::science::astrological::celestial::EARTH;
use crate::science::physical::constants::SPEED_OF_LIGHT_IN_VACUUM;

pub fn physical_time(_distance: f64) -> f64 {
    let c: f64 = SPEED_OF_LIGHT_IN_VACUUM.value;

    let distance: f64 = EARTH.velocity_m;

    let time: f64 = distance / c;

    time
}
