// UNIT CIRCLE
use std::f64::consts::PI;

// COMPLETE ROTATION
pub const FULL_TURN: f64 = 1.0;
pub const FULL_TURN_DEGREES: f64 = 360.0;
pub const FULL_TURN_RADIANS: f64 = 2.0 * PI;

// HALF ROTATION
pub const HALF_TURN: f64 = 1.0 / 2.0;
pub const HALF_TURN_DEGREES: f64 = 180.0;
pub const HALF_TURN_RADIANS: f64 = PI;

// QUARTER ROTATION
pub const QUARTER_TURN: f64 = 1.0 / 4.0;
pub const QUARTER_TURN_DEGREES: f64 = 90.0;
pub const QUARTER_TURN_RADIANS: f64 = PI / 2.0;

// EIGHTH ROTATION
pub const EIGHTH_TURN: f64 = 1.0 / 8.0;
pub const EIGHTH_TURN_DEGREES: f64 = 45.0;
pub const EIGHTH_TURN_RADIANS: f64 = PI / 4.0;

pub const THIRD_TURN: f64 = 1.0 / 3.0;
pub const SIXTH_TURN: f64 = 1.0 / 6.0;
pub const TWELFTH_TURN: f64 = 1.0 / 12.0;

pub const SIXTEENTH_TURN: f64 = 1.0 / 16.0;
pub const THIRTY_SECOND_TURN: f64 = 1.0 / 32.0;

pub const THIRD_TURN_DEGREES: f64 =
    FULL_TURN_DEGREES * THIRD_TURN;

pub const SIXTH_TURN_DEGREES: f64 =
    FULL_TURN_DEGREES * SIXTH_TURN;

pub const TWELFTH_TURN_DEGREES: f64 =
    FULL_TURN_DEGREES * TWELFTH_TURN;

pub struct Angles {
    pub turns: f64,
}

pub fn calc_angles() {
    let angle = Angles {
        turns: 1.0 / 4.0,
    };
}
