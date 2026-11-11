//use crate::science::physical::units::Units;
//use crate::science::physical::constants::SPEED_OF_LIGHT_IN_VACUUM;
use::std::fmt;

pub struct Mass {

}

#[derive(Debug)]
pub struct Dimensions {
    pub mass: i8,
    pub length: i8,
    pub time: i8,
}

impl Dimensions {
    pub fn new(mass: i8, length: i8, time: i8) -> Self {
        Self { mass, length, time }
    }

    pub fn mass() {
        Self::new(1, 0, 0);
    }
}

impl fmt::Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "M^{}, L^{}, T^{}", self.mass, self.length, self.time)
    }
}

//FUNDAMENTAL DIMENSIONS

pub const MASS: Dimensions = Dimensions {
    mass: 1,
    length: 0,
    time: 0,
};

pub const LENGTH: Dimensions = Dimensions {
    mass: 0,
    length: 1,
    time: 0,
};

pub const TIME: Dimensions = Dimensions {
    mass: 0,
    length: 0,
    time: 1,
};

//DERIVED DIMENSIONS

pub const VELOCITY: Dimensions = Dimensions {
    mass: 0,
    length: 1,
    time: -1,
};

pub const ACCELERATION: Dimensions = Dimensions {
    mass: 0,
    length: 1,
    time: -2,
};

pub const FORCE: Dimensions = Dimensions {
    mass: 1,
    length: 1,
    time: -2,
};



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


