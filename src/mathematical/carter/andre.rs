use std::f64::consts::PI;

//use crate::mathematical::symbolic::runes::PI_FOURTH;

pub fn andre(h: f64) -> f64 {
    let square = square(h);
    let quarter_circle = quarter_circle_area(h);

    square / quarter_circle
}

pub fn diameter_side(h: f64) -> f64 {
    2.0 * h
}

pub fn square(h: f64) -> f64 {
    h * h
}

pub fn diagonal(h: f64) -> f64 {
    (2.0 * square(h)).sqrt()
}

pub fn corner_length(h: f64) -> f64 {
    h
}

pub fn circle_area(h: f64) -> f64 {
    PI * square(h)
}

pub fn quarter_circle_area(h: f64) -> f64 {
    circle_area(h) / 4.0
}

pub fn andre_area(h: f64) -> f64 {
    square(h) - quarter_circle_area(h)
}

pub fn andre_area_ratio(h: f64) -> f64 {
    andre_area(h) / quarter_circle_area(h)
}