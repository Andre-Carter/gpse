pub struct Dimensions {
    pub mass: i8,
    pub length: i8,
    pub time: i8,
}

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