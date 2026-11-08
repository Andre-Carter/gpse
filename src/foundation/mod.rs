pub mod data_types;
pub mod format;
pub mod input;

// gpse\foundation\data_types\boolean

pub const TRUE: bool = true;
pub const FLASE: bool = false;

pub fn initialize() {
    println!("GPSE online");
}

pub fn terminate() -> ! {
    panic!("System terminated");
}