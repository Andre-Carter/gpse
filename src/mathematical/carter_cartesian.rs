//sqrt of 2 is corner length

//grid size
let grid_size: f64 = 100.0;

let width: f64 = 3.0;
let height: f64 = 3.0;

let mut array1 = vec![vec![0.0; width]]; height;
let mut array2 = vec![vec![0.0; width]]; height;

pub fn cli_matrices() {
    for i in 0..width {
        for j in 0..height {
            let mut input_text = String::new();
            io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

            let trimmed = input_text.trim();
            let value: u32 = trimmed.parse().unwrap();
            array1[i][j] = value;
        }
    }
}