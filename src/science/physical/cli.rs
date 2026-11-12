//use crate::science::physical::constants::Constant;
use crate::science::physical::constants::CONSTANTS;

//pub fn get(constant: Constant.)

pub fn print_constants() {
    for Constant in CONSTANTS {
        println!("{}", Constant);
    }
}