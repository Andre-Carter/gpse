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

here is the_forge:.rs in experimental, hooked up to experimental cli. with test_suite. im trying to build a dynamic testing environment. 

pub fn test_suite() {
let generic_one = Dimensions::new(1, 1, -2);
let generic_two = 2;
let generic_three = 3;

let variables: [fn() ; 3] = [
    generic_one,
    generic_two,
    generic_three,
];

for variables in variables {
    println!("{:?}", variables())
}

}

use crate::mathematical::dimensional::dimensional::Dimensions;

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

how do we fix this, is there generic data we can fill in when not in use? what small improvements can we make to the testing environment? Then we can tackle this, 

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

Three things stop this from compiling or being useful, and there’s a better shape for it.

What’s going wrong

[fn(); 3] is an array of function pointers, but generic_one, 2, and 3 are values, not functions. A Dimensions and an integer also can’t share one array, because arrays hold a single type.
fn() returns (), so println!("{:?}", functions()) prints () every time. The functions need to return something.
{:?} needs #[derive(Debug)] on Dimensions.
The loop variable shadows the array name (for variables in variables). That’s legal but confusing.

Placeholders

The idiomatic placeholder is a test that reports “skipped”. todo!() also works, but it panics, so it shows up as a failure. A skip is cleaner. You can keep numbered empty slots and fill them in as you build.

A small dynamic harness

Each test is a named function returning an Outcome. A registry lists them, and a runner handles counting, filtering, and panics:

rust
use crate::mathematical::dimensional::dimensional::Dimensions;
use std::{fmt::Debug, panic};

pub enum Outcome {
    Pass,
    Fail(String),
    Skip(&'static str),
}

pub struct Case {
    pub name: &'static str,
    pub run: fn() -> Outcome,
}

// Works for any type that has == and Debug: Dimensions, i32, String...
fn expect_eq<T: PartialEq + Debug>(left: T, right: T) -> Outcome {
    if left == right {
        Outcome::Pass
    } else {
        Outcome::Fail(format!("left:  {left:?}\n    right: {right:?}"))
    }
}

fn placeholder() -> Outcome {
    Outcome::Skip("not written yet")
}

fn display_force() -> Outcome {
    expect_eq(Dimensions::new(1, 1, -2).to_string(), "M L T⁻²".to_string())
}

fn display_dimensionless() -> Outcome {
    expect_eq(Dimensions::DIMENSIONLESS.to_string(), "1".to_string())
}

fn constants_match_new() -> Outcome {
    expect_eq(Dimensions::MASS, Dimensions::new(1, 0, 0))
}

const CASES: &[Case] = &[
    Case { name: "display_force",         run: display_force },
    Case { name: "display_dimensionless", run: display_dimensionless },
    Case { name: "constants_match_new",   run: constants_match_new },
    Case { name: "slot_4",                run: placeholder },
    Case { name: "slot_5",                run: placeholder },
];

pub fn test_suite(filter: Option<&str>) -> bool {
    let (mut pass, mut fail, mut skip) = (0, 0, 0);

    let selected = CASES
        .iter()
        .filter(|c| filter.map_or(true, |f| c.name.contains(f)));

    for case in selected {
        // catch_unwind so one panic doesn't kill the whole run
        let outcome = panic::catch_unwind(case.run)
            .unwrap_or_else(|_| Outcome::Fail("panicked".into()));

        match outcome {
            Outcome::Pass => { pass += 1; println!("  ok    {}", case.name); }
            Outcome::Skip(why) => { skip += 1; println!("  skip  {} ({why})", case.name); }
            Outcome::Fail(msg) => { fail += 1; println!("  FAIL  {}\n    {msg}", case.name); }
        }
    }

    println!("\n{pass} passed, {fail} failed, {skip} skipped");
    fail == 0
}

This relies on Dimensions deriving Debug, Clone, Copy, PartialEq, Eq, Hash. PartialEq is what lets expect_eq compare two Dimensions.

Hooking it to the CLI

Pass an optional filter and return a non-zero exit code on failure:

rust
// in your CLI dispatch
let filter = std::env::args().nth(2);      // e.g. `experimental test display`
let ok = the_forge::test_suite(filter.as_deref());
if !ok {
    std::process::exit(1);
}

Small improvements worth considering

Filtering by name is already in: test display runs only the display tests.
Test groups: add a group: &'static str field to Case ("display", "ops", "calculus") and filter on it too.
Timing: wrap each run in std::time::Instant to spot slow tests.
Probe mode: if you want to eyeball values of different types, have a probe return a String, as in fn() -> String { format!("{:?}", value) }. A Vec<(&str, fn() -> String)> then holds anything printable, since the type is erased at the point you format.
A failing-test macro later on, so case!(display_force, expect_eq(...)) removes the boilerplate.

Once Mul and Div exist, tests like “velocity × time = length” slot straight into CASES. That makes the harness a good safety net for the design work.

Next up is the object design. Do you want to start with operators on Dimensions (Mul, Div, powi) and a runtime-checked Quantity, or first decide how to handle fractional exponents for sqrt?