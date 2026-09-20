use std::f64::consts::PI;

pub const PI_SQ: f64 = PI * PI;
//sequencial mathematics vs individal math """"
//pub const CC_ONE: f64: = 

//CC: Carter Constant
//SQ: squared
//COMPL: COMPLEMENT
//RECIP: RECIPROCAL

pub const CC_KEY: f64 = 4.0;
pub const CC_RADIAN: f64 = PI / 4.0;

pub const CC_ONE: f64 = 1.0 - PI;
pub const CC_FOUR: f64 = 4.0 - PI;

pub fn cc_variable(n: f64) -> f64 {
    n - PI
}

//CARTER CONSTANT

pub const CC: f64 = 1.0 - CC_RADIAN;
pub const CC_EQ: f64 = CC_RADIAN + CC;

pub const CC_A: f64 = 1.0 + CC_RADIAN;
pub const CC_B: f64 = (1.0 + PI) / 4.0;

//CC OPERATIONS

pub const CC_CC: f64 = CC + CC;
pub const CC_EQ_ZERO: f64 = CC - CC;
pub const CC_SQ: f64 = CC * CC;

pub const CC_COMPL: f64 = 1.0 - CC;
pub const CC_COMPL_EQ: f64 = PI / 4.0;

pub const CC_RECIP: f64 = 1.0 / CC;
pub const CC_RECIP_EQ: f64 = 4.0 / CC_FOUR;

pub const CC_TO_COMPL_RATIO: f64 = CC / CC_COMPL;
pub const CC_TO_COMPL_RATIO_EQ: f64 = CC_FOUR / PI;

pub const COMPL_TO_CC_RATIO: f64 = CC_COMPL / CC;
pub const COMPL_TO_CC_RATIO_EQ: f64 = PI / CC_FOUR;

pub fn carter_formula(h: f64) -> f64 {
    println!("h = {h}");

    let ds: f64 = h + h;
    println!("Diameter/Side: {ds}");

    let ds2: f64 = h * h;
    println!("Radius Squared: {ds2}");

    let sa: f64 = ds * ds;
    println!("Square Area: {sa}");

    let ca: f64 = PI * ds2;
    println!("Circle Area: {ca}");

    let sp: f64 = h + h + h + h + h + h + h + h;
    println!("Square Perimeter: {sp}");

    let cc: f64 = (2.0 * PI) * h;
    println!("Circle Circumfrence: {cc}");

    let gap: f64 = sa - ca;
    println!("Gap: {gap}");

    gap
}

//pub const CARTER_RATIO: f64 = 
pub fn carter_ratio(n: f64) -> f64 {
    n / (n + PI_SQ)
}
//CIRCLE SQUARE FOUNDATION
pub fn carter_gap_area(side: f64) -> f64 {
    CC * side.powi(2)
}

pub fn test_carter() {
    println!("{}", CC_EQ_ZERO);
}