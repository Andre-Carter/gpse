use crate::cell;

pub fn brain() {
    let mut cells = Cells::new();

    cells.add(Cell::new(3.14));
    cells.add(Cell::new(2.71));

    cells
}

print_brain() {
    println!("Cell count: {}", cells.len());
}