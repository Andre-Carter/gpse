The important edge cases

Case	        GPSE error

u8::MAX + 1	    Overflow
0_u8 - 1	    Underflow
i8::MAX + 1	    Overflow
i8::MIN - 1	    Underflow
10_u32 / 0	    DivisionByZero
10_u32 % 0	    DivisionByZero
i32::MIN / -1	Overflow
i32::MIN % -1	Overflow
2_u8.pow(8)	    Overflow
0_u8.pow(0)	    Ok(1) under Rust's integer-power convention

