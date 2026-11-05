Yes, all 126 tests pass, up from 87 passing and 39 failing. Every div, rem and pow test is green, so pow_u8, pow_u16 and pow_u32 must have been fixed too.

Green here means the tests pass, not that the module is correct, because the tests only cover easy cases. These bugs are still in the code, and no test touches them:

mul_*:
mul_u8(0, 5) panics.
mul_i8(-1, 5) returns Overflow (the answer is -5).
mul_i8(i8::MIN, -1) panics.
mul_i64 has left and right swapped in its second check.
Float add, sub and mul: sub_f32(f32::MAX, f32::MAX) returns Overflow instead of 0.0, and sub_f32(f32::MAX, -f32::MAX) returns Ok(inf).
pow_f32 and pow_f64: the bounds are arbitrary, so pow_f64(10.0, 100.0) is wrongly rejected, and powf(-8.0, 0.5) returns Ok(NaN).

Next, I’d add tests that expose those bugs and turn the suite red, then fix them with checked_mul and the compute-then-is_infinite pattern you already used in div_f32. These would be the new tests:

rust
mul_u8(0, 5) == Ok(0)
mul_i8(-1, 5) == Ok(-5)
mul_i8(i8::MIN, -1) == Err(Overflow)
sub_f32(f32::MAX, f32::MAX) == Ok(0.0)
pow_f64(10.0, 100.0) == Ok(1e100)

Before you commit, also run cargo clippy to check that the three unused_comparisons warnings are gone, and cargo fmt.

Want me to write those tests and the mul, float and pow_f* fixes?

Fixes: 

mul_*(0, 5) == Ok(0)
mul_i8(-1, 5) == Ok(-5)
mul_i8(i8::MIN, -1) == Err(Overflow)
div_*(1, 0) == Err(DivisionByZero)
div_i8(i8::MIN, -1) == Err(Overflow)
rem_*(1, 0) == Err(DivisionByZero)
pow_i8(2, -1) == Err(InvalidExponent)
sub_f32(f32::MAX, f32::MAX) == Ok(0.0)
pow_f64(10.0, 100.0) == Ok(1e100)