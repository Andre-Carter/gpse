use std::f64::consts::PI;
use std::io::{Write, stdin, stdout};

pub const PI_SQ: f64 = PI * PI;
//sequencial mathematics vs individal math """"

//CC: Carter Constant
//SQ: squared
//COMPL: COMPLEMENT
//RECIP: RECIPROCAL

pub const CC_2D: f64 = 4.0;
pub const CC_3D: f64 = 6.0;

pub const QUARTER_ANGLE: f64 = 90.0 / PI;

pub const PI_FOURTH: f64 = PI / 4.0;
pub const PI_SIXTH: f64 = PI / 6.0;

pub const ONE_SUB_PI: f64 = 1.0 - PI;
pub const FOUR_SUB_PI: f64 = 4.0 - PI;

pub fn cc_variable(n: f64) -> f64 {
    n - PI
}

//CARTER CONSTANT
pub const CC: f64 = 1.0 - PI_FOURTH;

//CARTER CONSTANT EQUATION IS EQUAL TO 1
pub const CC_EQ: f64 = CC + PI_FOURTH;

pub const CC_A: f64 = 1.0 + PI_FOURTH;
pub const CC_B: f64 = (1.0 + PI) / 4.0;

pub const CC_C: f64 = 1.0 - PI_SIXTH;

//CC OPERATIONS

pub const CC_ADD: f64 = CC + CC;
pub const CC_SUB: f64 = CC - CC;
pub const CC_SQ: f64 = CC * CC;
pub const CC_SQ_EQ: f64 = (1.0 - PI_FOURTH) * (1.0 - PI_FOURTH);

//CC COMPLEMENT & RECIPROCAL

pub const CC_COMPL: f64 = 1.0 - CC;
pub const CC_COMPL_EQ: f64 = PI / 4.0;

pub const CC_RECIP: f64 = 1.0 / CC;
pub const CC_RECIP_EQ: f64 = 4.0 / FOUR_SUB_PI;

pub const CC_TO_COMPL_RATIO: f64 = CC / CC_COMPL;
pub const CC_TO_COMPL_RATIO_EQ: f64 = FOUR_SUB_PI / PI;

pub const COMPL_TO_CC_RATIO: f64 = CC_COMPL / CC;
pub const COMPL_TO_CC_RATIO_EQ: f64 = PI / FOUR_SUB_PI;

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn carter_formula_cli() {
    print!("Enter h-input: ");

    let mut h_input: String = String::new();
    read(&mut h_input);

    let h_input: f64 = match h_input.trim().parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid Input");
            return;
        }
    };

    carter_formula(h_input);
}

pub fn carter_formula_repeat() {
    let iterations = 1;

    for h in 1..=iterations {
        let h = h as f64;
        carter_formula(h);
    }
}

pub fn carter_formula(h: f64) -> f64 {
    println!("h = {h}");

    let ds: f64 = h + h;
    println!("(2h) Diameter-Side: {ds}");

    let h2: f64 = h * h;

    println!();

    let corner_length: f64 = h * 2.0_f64.sqrt();
    println!("Corner-Length: {corner_length}");

    //let corner_length: f64 = (h2 + h2).sqrt();
    //println!("Corner-Length: {corner_length}");

    println!();

    let cc: f64 = (2.0 * PI) * h;
    println!("Circle Circumfrence: {cc}");

    //let cc_pi: f64 = ds * PI;
    //println!("Circle Circumfrence From PI: {cc_pi}");

    let sp: f64 = 4.0 * ds;
    println!("Square Perimeter: {sp}");

    let ca: f64 = PI * h2;
    println!("Circle Area: {ca}");

    let sa: f64 = ds * ds;
    println!("Square Area: {sa}");

    //let sa2: f64 = h2 * 4.0;
    //println!("Square Area (Alternative): {sa2}");

    println!();

    let qa_eq: f64 = sa / 4.0;
    println!("Quarter-Area: {qa_eq}");

    //let sp: f64 = ds * 4.0;
    //println!("Square Perimeter: {sp}");

    println!();

    let agap: f64 = sa - ca;
    println!("A-Gap: {agap}");

    //let cc_agap: f64 = (sa - ca) / sa;

    let agap_eq: f64 = sa * CC;
    println!("A-Gap Eq.: {agap_eq}");

    let pgap: f64 = sp - cc;
    println!("P-Gap: {pgap}");

    let pgap_sp_ratio: f64 = pgap / sp;
    println!("Perimeter-Gap / Square-Area Ratio: {pgap_sp_ratio}");

    //RATIOS RATIOS RATIOS RATIOS RATIOS RATIOS RATIOS RATIOS RATIOS RATIOS RATIOS RATIOS

    println!();

    println!("Ratios:");

    let agap_eq: f64 = agap / sa;
    println!("CARTER-CONSTANT: {agap_eq}");

    let cc_eq: f64 = (agap / sa) - CC;
    println!("CC CHECK: {cc_eq}");

    println!();

    let corner_ratio: f64 = h / corner_length;
    println!("Corner-Ratio: {corner_ratio}");

    println!();

    let ca_sa_ratio: f64 = ca / sa;
    println!("Circle/Square Area Ratio: {ca_sa_ratio}"); //is equal to pi over 4

    //let ca_sa_ratio_eq: f64 = ca_sa_ratio - PI_FOURTH;
    //println!("Circle/Square Area Ratio Equation: {ca_sa_ratio_eq}");

    let cc_sp_ratio: f64 = cc / sp;
    println!("Circle/Square Perimeter Ratio: {cc_sp_ratio}"); //is equal to pi over 4

    let sa_ca_ratio: f64 = sa / ca;
    println!("Square/Circle Area Ratio: {sa_ca_ratio}"); //is equal to 4 over pi

    let sp_cc_ratio: f64 = sp / cc;
    println!("Square/Circle Perimeter Ratio: {sp_cc_ratio}"); //is equal to 4 over pi

    println!();

    let qtr_c: f64 = cc / 4.0;
    println!("Quarter-Circumfrence: {qtr_c}");

    let qtr_c_fraction: f64 = 1.0 / qtr_c;
    println!("Quarter-Circumfrence Fraction {qtr_c_fraction}");

    //let qtr_c_ratio: f64 = 1.0 / qtr_c;
    let qtr_c_ratio: f64 = h / (cc / 4.0);
    println!("Quarter-Circumfrence Ratio {qtr_c_ratio}");

    let d: f64 = cc / PI;
    println!("Diameter From Circumfrence: {d}");

    //3-Dimensional Calculations

    println!();
    println!("3-D");
    println!("h = {h}");
    println!();

    let sphere_sfa: f64 = (4.0 * PI) * h2;
    println!("Sphere Surface Area: {sphere_sfa}");

    let cube_sfa: f64 = ds * ds * 6.0;
    println!("Cube Surface Area: {cube_sfa}");

    let sphere_pi: f64 = (4.0 / 3.0) * PI;
    let sphere_v: f64 = sphere_pi * (h * h * h);
    println!("Sphere Volume: {sphere_v}");

    let cube_v: f64 = ds * ds * ds;
    println!("Cube Volume: {cube_v}");

    println!();

    //note: 

    let v_gap: f64 = cube_v / sphere_v;
    println!("Cube/Sphere Volume Ratio: {v_gap}");

    let sfa_gap: f64 = cube_sfa / sphere_sfa;
    println!("Cube/Sphere Surface-Area Ratio: {sfa_gap}");

    let inv_v_gap: f64 = sphere_v / cube_v;
    println!("Inverse V-Gap, Sphere/Cube Volume: {inv_v_gap}");

    let inv_sfa_gap: f64 = sphere_sfa / cube_sfa;
    println!("Inverse SFA Gap, Sphere/Cube Surface-Area: {inv_sfa_gap}");

    inv_sfa_gap
}

//pub const CARTER_RATIO: f64 =
pub fn carter_ratio(n: f64) -> f64 {
    n / (n + PI_SQ)
}
//CIRCLE SQUARE FOUNDATION
pub fn carter_gap_area(side: f64) -> f64 {
    CC * side.powi(2)
}
