// Unit identity
//       ↓
// Dimension
//       ↓
// Conversion
//       ↓
// Canonical representation
//       ↓
// Display representation

//canonical mass = kilogram
//canonical distance = meter
//canonical time = second

//DIMENSIONS

pub struct Dimensions {
    pub mass: i8,
    pub distance: i8,
    pub time: i8,
}

//FUNDAMENTAL DIMENSIONS

pub const MASS: Dimensions = Dimensions {
    mass: 1,
    distance: 0,
    time: 0,
};

pub const DISTANCE: Dimensions = Dimensions {
    mass: 0,
    distance: 1,
    time: 0,
};

pub const TIME: Dimensions = Dimensions {
    mass: 0,
    distance: 0,
    time: 1,
};

//DERIVED DIMENSIONS

pub const VELOCITY: Dimensions = Dimensions {
    mass: 0,
    distance: 1,
    time: -1,
};

pub const ACCELERATION: Dimensions = Dimensions {
    mass: 0,
    distance: 1,
    time: -2,
};

pub const FORCE: Dimensions = Dimensions {
    mass: 1,
    distance: 1,
    time: -2,
};

//QUANTITIES

pub struct Quantity {
    pub value: f64,
    pub dimensions: Dimensions,
}

//let distance = Quantity {
//    value: 384_400_000.0,
//    dimensions: DISTANCE,
//};

//UNITS

pub struct Unit {
    pub name: &'static str,
    pub symbol: &'static str,
    pub system: &'static str,
    pub dimensions: Dimensions,
    pub scale: f64,
}

//MASS(METRIC)
//microgram  1e-9 kg
//milligram  1e-6 kg
//gram       1e-3 kg
//kilogram   1e0  kg
//tonne      1e3  kg

pub static MICROGRAM: Unit = Unit {
    name: "Microgram",
    symbol: "mcg",
    system: "metric",
    dimensions: MASS,
    scale: 0.000_000_001,
};

pub static MILLIGRAM: Unit = Unit {
    name: "Milligram",
    symbol: "mg",
    system: "metric",
    dimensions: MASS,
    scale: 0.000_001,
};

pub static GRAM: Unit = Unit {
    name: "Gram",
    symbol: "g",
    system: "metric",
    dimensions: MASS,
    scale: 0.001,
};

pub static KILOGRAM: Unit = Unit {
    name: "Kilogram",
    symbol: "kg",
    system: "metric",
    dimensions: MASS,
    scale: 1.0,
};

pub static TONNE: Unit = Unit {
    name: "Tonne",
    symbol: "t",
    system: "metric",
    dimensions: MASS,
    scale: 1_000.0,
};

//MASS(IMPERIAL)

pub static OUNCE: Unit = Unit {
    name: "Ounce",
    symbol: "oz",
    system: "imperial",
    dimensions: MASS,
    scale: 0.028_349_523_125,
};

pub static POUND: Unit = Unit {
    name: "Pound",
    symbol: "lb",
    system: "imperial",
    dimensions: MASS,
    scale: 0.453_592_37,
};

pub static SHORT_TON: Unit = Unit {
    name: "Short Ton",
    symbol: "ton",
    system: "imperial",
    dimensions: MASS,
    scale: 907.184_74,
};

//DISTANCE(METRIC)

pub static MILLIMETER: Unit = Unit {
    name: "Millimeter",
    symbol: "mm",
    system: "metric",
    dimensions: DISTANCE,
    scale: 0.001,
};

pub static CENTIMETER: Unit = Unit {
    name: "Centimeter",
    symbol: "cm",
    system: "metric",
    dimensions: DISTANCE,
    scale: 0.01,
};

pub static METER: Unit = Unit {
    name: "Meter",
    symbol: "m",
    system: "metric",
    dimensions: DISTANCE,
    scale: 1.0,
};

pub static KILOMETER: Unit = Unit {
    name: "Kilometer",
    symbol: "km",
    system: "metric",
    dimensions: DISTANCE,
    scale: 1000.0,
};

//DISTANCE(IMPERIAL)
pub static INCH: Unit = Unit {
    name: "Inch",
    symbol: "in",
    system: "imperial",
    dimensions: DISTANCE,
    scale: 0.025_4,
};

pub static FOOT: Unit = Unit {
    name: "Foot",
    symbol: "ft",
    system: "imperial",
    dimensions: DISTANCE,
    scale: 0.304_8,
};

pub static YARD: Unit = Unit {
    name: "Yard",
    symbol: "yd",
    system: "imperial",
    dimensions: DISTANCE,
    scale: 0.914_4,
};

pub static MILE: Unit = Unit {
    name: "Mile",
    symbol: "mi",
    system: "imperial",
    dimensions: DISTANCE,
    scale: 1_609.344,
};

//TIME(METRIC)
//femtosecond??

pub static PICOSECOND: Unit = Unit {
    name: "picosecond",
    symbol: "ps",
    system: "metric",
    dimensions: TIME,
    scale: 0.000_000_000_001,
};

pub static NANOSECOND: Unit = Unit {
    name: "Nanosecond",
    symbol: "ns",
    system: "metric",
    dimensions: TIME,
    scale: 0.000_000_001,
};

pub static MICROSECOND: Unit = Unit {
    name: "Microsecond",
    symbol: "mcs", //_u v
    system: "metric",
    dimensions: TIME,
    scale: 0.000_001,
};

pub static MILLISECOND: Unit = Unit {
    name: "Millisecond",
    symbol: "ms",
    system: "metric",
    dimensions: TIME,
    scale: 0.001,
};

pub static SECOND: Unit = Unit {
    name: "Second",
    symbol: "s",
    system: "metric",
    dimensions: TIME,
    scale: 1.0,
};

pub static KILOSECOND: Unit = Unit {
    name: "Kilosecond",
    symbol: "ks",
    system: "metric",
    dimensions: TIME,
    scale: 1_000.0,
};

pub static MEGASECOND: Unit = Unit {
    name: "Megasecond",
    symbol: "Ms",
    system: "metric",
    dimensions: TIME,
    scale: 1_000_000.0,
};

pub static GIGASECOND: Unit = Unit {
    name: "Gigasecond",
    symbol: "Gs",
    system: "metric",
    dimensions: TIME,
    scale: 1_000_000_000.0,
};

//TIME(CALENDAR)
pub static MINUTE: Unit = Unit {
    name: "Minute",
    symbol: "m",
    system: "calendar",
    dimensions: TIME,
    scale: 60.0,
};

pub static HOUR: Unit = Unit {
    name: "Hour",
    symbol: "hr",
    system: "calendar",
    dimensions: TIME,
    scale: 3_600.0,
};

pub static DAY: Unit = Unit {
    name: "Day",
    symbol: "d",
    system: "calendar",
    dimensions: TIME,
    scale: 86_400.0,
};

pub static WEEK: Unit = Unit {
    name: "Week",
    symbol: "w",
    system: "calendar",
    dimensions: TIME,
    scale: 604_800.0,
};

/*
pub static MONTH: Unit = Unit {
    name: "Month",
    symbol: "m",
    system: "calendar",
    dimensions: TIME,
    scale: 2_629_743.0,
}

pub static YEAR: Unit = Unit {
    name: "Year",
    symbol: "y",
    system: "calendar",
    dimensions: TIME,
    scale: 31_557_600.0,
}

pub static DECADE: Unit = Unit {
    name: "Decade",
    symbol: "dc",
    system: "calendar",
    dimensions: TIME,
    scale: 315_576_000.0,
}

pub static CENTURY: Unit = Unit {
    name: "Century",
    symbol: "cn",
    system: "calendar",
    dimensions: TIME,
    scale: 31_557_600_000.0,
}

pub static MILLENNIUM: Unit = Unit {
    name: "Millennium",
    symbol: "me",
    system: "calendar",
    dimensions: TIME,
    scale: 315_576_000_000.0,
}
//1 Million Years
pub static MEGAANNUM: Unit = Unit {
    name: "Megaannum",
    symbol: "Ma",
    system: "calendar",
    dimensions: TIME,
    scale: 31_577_600_000_000.0,
}
*/
