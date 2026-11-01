let numbers = [1, 2, 3, 4, 5];

Type and length:
let numbers: [i32; 5] = [1, 2, 3, 4, 5];

Repeat array values:
let zeros: [f64; 5] = [0.0; 5];

Produces:
[0.0, 0.0, 0.0, 0.0, 0.0]

Print:
for value in numbers {
    println!("{value}");
}