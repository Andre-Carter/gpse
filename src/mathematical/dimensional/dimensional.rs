//use crate::science::physical::units::Units;
//use crate::science::physical::constants::SPEED_OF_LIGHT_IN_VACUUM;
use ::std::fmt;

pub struct Mass {}

#[derive(Debug)]
pub struct Dimensions {
    pub mass: i8,
    pub length: i8,
    pub time: i8,
}

impl Dimensions {
    pub const fn new(mass: i8, length: i8, time: i8) -> Self {
        Self { mass, length, time }
    }

    // Base dimensions
    pub const DIMENSIONLESS: Self = Self::new(0, 0, 0);
    pub const MASS: Self = Self::new(1, 0, 0);
    pub const LENGTH: Self = Self::new(0, 1, 0);
    pub const TIME: Self = Self::new(0, 0, 1);

    // Derived dimensions
    pub const VELOCITY: Self = Self::new(0, 1, -1);
    pub const ACCELERATION: Self = Self::new(0, 1, -2);
    pub const FORCE: Self = Self::new(1, 1, -2);
    pub const ENERGY: Self = Self::new(1, 2, -2);
    pub const POWER: Self = Self::new(1, 2, -3);
    pub const PRESSURE: Self = Self::new(1, -1, -2);
}

fn superscript(n: i8) -> String {
    n.to_string()
        .chars()
        .map(|c| match c {
            '-' => '⁻',
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            other => other,
        })
        .collect()
}

impl fmt::Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts = [("M", self.mass), ("L", self.length), ("T", self.time)];
        let mut wrote_any = false;

        for (symbol, exp) in parts {
            if exp == 0 {
                continue;
            }
            if wrote_any {
                write!(f, " ")?;
            }
            write!(f, "{symbol}")?;
            if exp != 1 {
                write!(f, "{}", superscript(exp))?;
            }
            wrote_any = true;
        }

        if !wrote_any {
            write!(f, "1")?;
        }
        Ok(())
    }
}

//FUNDAMENTAL DIMENSIONS

//DIMENSION

/*
// test

pub fn _dimensions(dimensions: Dimensions) {
    let mass_power = Dimensions.mass;
    let length_power = Dimensions.length;
    let time_power = Dimensions.time;

    let m = m.powi(mass_power);
    let l = l.powi(length_power);
    let t = t.powi(time_power);

    let dimensions = m * l * t;

    println!("{}", dimensions);
}

pub fn _dimension(value: f64, dimensions: Dimensions) {
    DIMENSION(SPEED_OF_LIGHT_IN_VACUUM.value, VELOCITY);


}

pub fn _dimensionless(Units) {

}

pub struct Dimension {
    value: f64,
    dimensions: Dimensions,
}

pub struct Dimensionless {
    value: Dimensions,
}
*/
