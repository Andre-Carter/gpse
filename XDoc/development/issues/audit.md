High	pow_* in arithmetic.rs uses ^ (XOR) instead   of checked_pow

High	Arithmetic overflow checks are mostly wrong (mul_* divides by zero when left == 0, div_* compares against MAX, add_i* underflow checks are no-ops). checked_add and friends fix all of it in one line each

High	Arithmetic tests assert wrong values (mul_u8(10, 11) == 21) and sub_u8(1, -127) doesn’t compile
Medium	NumericType::range() hits todo!() for floats, and main.rs calls it for U32 only, so it’s a panic waiting for F32

Medium	numeric.rs tests reference u32_to_u64 etc. without importing them, and f32_to_f64(100) passes an integer

Medium	Element data: Darmstadtium Dr should be Ds, Actinium 277 should be about 227, “Nobel Gas” and “Nickle” typos, Scandium and Zinc use their own names as chemical_group

Medium	celestial.rs: velocity_m holds km/h and radius_m holds km, which contradicts your own unit standard

Medium	constants.rs: several entries have the wrong name (e.g. ANOMALOUS_MAGNETIC_MOMENT_OF_THE_MUON is named “atomic mass constant”, COMPTON_WAVELENGTH_OVER_2_PI is named “conductance quantum”)

Low	Library code that prints instead of returning values

Low	Duplicate read() helper in about a dozen files; carter_family copy.rs has a space in the filename

// section

Bugs, in priority order

1. cargo test doesn’t compile. test_mul_i8 is defined twice (lines 1145 and 1151), which is error E0428. cargo check doesn’t build test code, so your green check was hiding it. Rename the second one to test_mul_i8_negative.

2. Every integer mul_* is wrong.

u8::MAX / left panics when left == 0. Your own test_mul_u8_two (mul_u8(0, 5)) would crash.
i8::MIN / right panics when right == 0 or -1.
For negatives the comparisons are backwards. mul_i8(-1, 5) computes 5 > 127 / -1, which is 5 > -127, so it returns Overflow for a perfectly valid product.
mul_i64 uses right < i64::MIN / left while the others use left < i8::MIN / right, so the copies have drifted apart.

3. Float mul_* has the same flaw. mul_f32(-2.0, 3.0) computes 3.0 > MAX / -2.0, which is true, and returns Overflow. Float sub_* copies the add checks, which are the wrong checks for subtraction.

4. pow_f32 and pow_f64 use made-up limits. pow_f32(2.0, 100.0) is a valid 1.27e30 but returns Overflow because the exponent is over 32. pow_f32(-40.0, 2.0) is 1600 but returns Underflow. The check left < f64::MIN in pow_f64 can never be true.

// section

Replace the todo!() in NumericType::range() for floats.
Fix the misnamed F64_MAXIMUM_POSITIVE_NORMAL_VALUE, which is really f64::MIN_POSITIVE.
Add EPSILON, radix and digits, per your to-do.
Move println! out of Numeric::minimum_values and maximum_values. They should return data, and the CLI should print it.
Add tests whose expected values come from your data_types.md tables.

# Phase 2: 
conversion that’s worth having. issues.md calls the conversion CLI useless because u32 → u64 always succeeds. Make it a report: source type, target type, and a result that says lossless, lossy (precision) or failed (overflow). Use TryFrom for narrowing, and test the five rows of your earlier table (u64 → u32, i32 → u32, f64 → u32, u64 → f64).

# Phase 3: 
unify errors. Create foundation::errors, and implement Display and std::error::Error for ArithmeticError, SequenceError and the conversion errors. Do this after Phase 2 so you know what the third error looks like.

# Phase 4: 
text. Decide input vs text, then add tests for characters and capitalize_first_letter, and handle EOF in read_text().

# Phase 5+: 
on demand. Anything else in the tree is added when a real caller needs it.

Definition of done for each item

This is your development_procedures.md checklist, shortened:

The domain function returns data and errors, and doesn’t print.
Tests cover expected values, edge cases and errors.
It can be exercised from the CLI.
The XGPSE note has a status header.
cargo fmt, check, test and clippy pass, and you commit once.
Where it starts

Phase 1 depends on cargo test being green for arithmetic, so that’s the first thing to run when you’re back. After that, Phase 0 is documentation only, which suits a low-energy session.