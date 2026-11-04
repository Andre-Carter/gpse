use crate::core::input::text::characters;

// SYMBOLICS

//ONE
//TWO
//THR
//FOU
//FIV
//SIX
//SEV
//EIG
//NIN
//TEN
//TWE
//THI

//pub const POI: &str = ".";

//pub const VOI: u8 = 0;
//pub const ONE: u8 = 1;


// a,e,i,o,u


pub fn alphabet() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();

    let letters = characters(&alphabet);

    println!("{:?}", letters);
}
