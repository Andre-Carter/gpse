use std::f64::consts::PI;

pub fn volume_sphere(radius: f64) {
    let volume = 4.0 / 3.0 * PI * radius.powi(3);
    println!("{}", volume);
}

//pub fn formula_carter_ratio (n: f64) -> f64 {
//    let carter_input = n.powi(2) - n - PI / 4.0;
//    let carter_ratio = carter_input.powi(2) / (2.0 * PI);
//    println!("{}", carter_ratio);
//}

//pub fn carter_theory (n: f64) -> f64 {
//    let ct_input_1 = n - 1.0 / 2.0;
//    let ct_input_2 = ct_input_1.powi(2);

//test
//let ct_input_3 = (1.0 + PI) / 4.0;

//    let ct_input_3 = 1.0 + (PI / 4.0);

//    let ct_input_4 = ct_input_2 - ct_input_3;

//    let ct_input_5 = ct_input_4.powi(2);

//    let ct_input_6 = ct_input_5 / (2.0 * PI);

//    let ct_input_7 = ct_input_6 / n.powi(4);

//    ct_input_7
//}

//pub fn carter_constant(n: f64) -> f64 {
//  let carter_constant = 1.0 - (PI / 4.0);
// let cc_1 = n.powi(2) - carter_constant;
//  cc_1
//}
