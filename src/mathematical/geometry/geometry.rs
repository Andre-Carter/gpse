pub fn formula_area(dimension_1: f64, dimension_2: f64) {
    let sum_area = dimension_1 * dimension_2;
    println!("{}", sum_area);
}

pub fn formula_volume(dimension_1: f64, dimension_2: f64, dimension_3: f64) {
    let sum_volume = dimension_1 * dimension_2 * dimension_3;
    println!("{}", sum_volume);
}

//SHAPES: SQUARE, RECTANGLE, TRIANGLE, CIRCLE, TRAPEZOID,

//AREA

// what if we can pass a generic or rather 
// pass a numeric data-type wrapper into functions
// (data_value)
// ex. length: data_value, width: data_value

pub fn square_area(dimension: f64) -> f64 {
    let sum_area_square = dimension.powi(2);
    sum_area_square
}

pub fn square_perimeter(dimension: f64) -> f64 {
    let square_perimeter = dimension * 4.0;
    square_perimeter
}

pub fn rectangle() {

}

//VOLUME
//2D

//3D

//PERIMETER
