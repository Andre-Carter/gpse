Definition: What the type represents.

Range: Minimum and maximum values, where applicable.

Literals: How to specify values explicitly in Rust.

Operations: Relevant arithmetic behavior and overflow considerations.

Conversions: Casting, conversion risks, and type compatibility.

GPSE applications: Where the type might be useful.

Examples and tests: Small, compilable Rust examples.

Numeric Types

x. type     (note: category,            width,                  typical use)

1.  usize    (note: unsigned integer,    platform-dependent,     indexes, collection lengths, and memory-related sizes)     (min: 0 max: platform-dependent)

2.  u32      (note: unsigned integer,    32-bit,                 non-negative integer values)                               (min: 0 max: 4,294,967,295)

3.  u64      (note: unsigned integer,    64-bit,                 larger non-negative integer values)                        (min: 0 max: 18,446,744,073,709,551,615)

4.  i32      (note: signed integer,      32-bit,                 general whole numbers)                                     (min: -2,147,483,648 max: 2,147,483,647)

5.  i64      (note: signed integer,      32-bit,                 larger whole numbers)                                      (min: -9,223,372,036,854,775,808 max: 9,223,372,036,854,775,807)

6.  f32      (note: floating-point,      32-bit,                 approximate decimal values)                                (min: max: )

minimum finite value: −3.4028235×10^38 
maximum finite value: 3.4028235×10^38 
smallest positive normal value: 1.17549435×10^−38
smallest positive subnormal value: ≈1.40129846×10^−45 
approx. decimal significant digits: 6–7

7.  f64      (note: floating-point,      64-bit,                 higher-precision approximate calculations)                 (min: max: )

minimum finite value: −1.7976931348623157×10^30
maximum finite value: 1.7976931348623157×10^308
smallest positive normal value: 2.2250738585072014×10^−308
smallest positive subnormal value: ≈4.9406564584124654×10^−324
approx. decimal significant digits: 15–16

u32::MIN
u32::MAX

u64::MIN
u64::MAX

f32::INFINITY
f64::INFINITY

f32::NEG_INFINITY
f64::NEG_INFINITY

f32::NAN
f64::NAN

Rust's f32 and f64 use IEEE 754 binary floating-point formats

Property
f32 
f64

Bits
32
64

Minimum finite value
−3.4028235×10^38
−1.7976931348623157×10^308

Maximum finite value
3.4028235×10^38
1.7976931348623157×10^308

Smallest positive normal value
1.17549435×10^−38
2.2250738585072014×10^−308

Smallest positive subnormal value
≈1.40129846×10^−45
≈4.9406564584124654×10^−324

Approx. decimal significant digits
6–7
15–16