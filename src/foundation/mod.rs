pub mod data_types;
pub mod input;

pub fn initialize() {
    println!("GPSE online");
}

pub fn terminate() -> ! {
    panic!("System terminated");
}