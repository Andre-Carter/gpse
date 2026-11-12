//use crate::science::physical::constants::Constant;
use crate::science::physical::constants::CONSTANTS;

//pub fn get(constant: Constant.)

pub fn print_constants() {
    for constant in CONSTANTS {
        println!("{}", constant);
    }
}