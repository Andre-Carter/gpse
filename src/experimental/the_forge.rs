use crate::mathematical::dimensional::dimensional::Dimensions;

pub fn test_suite() {
let generic = Dimensions::new(1, 1, -2);

println!("{:?}", generic);
}


fn one() {

}

fn two() {

}

fn three() {

}

pub fn main() {
    let functions: [fn() ; 3] = [
        one,
        two,
        three
    ];

    for functions in functions {
        println!("{:?}", functions())
    }
}

/*
Object design: where we can go next

Your struct already maps directly onto the lesson, because the exponent rules become operator overloads:

Multiply adds exponents, so impl Mul for Dimensions.
Divide subtracts exponents, so impl Div for Dimensions.
Power multiplies exponents, so a powi(n) method.
Add and subtract require equal dimensions (Rule 4), which raises the question of how to handle mismatches.
Derives: #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] gives you equality checks and the ability to use dimensions as map keys.
Fractional exponents: sqrt needs exponents like ½, which i8 can’t hold. That’s the pendulum result (√(l/g)), so you may want a small rational type instead.

The biggest design question is when mismatches get caught. A runtime check is simple: add returns a Result and fails if the dimensions differ. A compile-time check puts the dimensions in the type system, so meters + seconds won’t compile at all. That route is more powerful but much more complex in Rust. It’s what the uom crate does, and stable const generics can’t yet do arithmetic on exponents.

Which direction do you want to explore first: a runtime-checked Quantity { value: f64, dims: Dimensions } with operators, or the compile-time approach?
*/