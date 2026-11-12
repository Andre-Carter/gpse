use crate::mathematical::dimensional::dimensional::Dimensions;

pub fn test_suite() {
    println!("\nTHE TEST SUITE\n");

    let functions: [fn(); 9] = [one, two, three, four, five, six, seven, eight, nine];

    for f in functions {
        f();
        println!("\n");
    }

    println!("");
}

fn one() {
    let one = Dimensions::new(1, 1, -2);
    println!("{:?}", one)
}

fn two() {
    let two = 2;
    println!("{:?}", two);
}

fn three() {
    let three = 3;
    println!("{:?}", three);
}

fn four() {
    let four = 4;
    println!("{:?}", four);
}

fn five() {
    let five = 5;
    println!("{:?}", five);
}

fn six() {
    let six = 6;
    println!("{:?}", six);
}

fn seven() {
    let seven = 7;
    println!("{:?}", seven);
}

fn eight() {
    let eight = 8;
    println!("{:?}", eight);
}

fn nine() {
    let nine = 9;
    println!("{:?}", nine);
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
