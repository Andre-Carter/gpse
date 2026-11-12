use crate::foundation::input::read_text;

pub fn checklist(input: &str) {
    let parts: Vec<&str> = input.split(',').collect();

    for part in parts {
        println!("[ ] {part}");
    }
}

pub fn checklist_cli() {
    let input = read_text();

    checklist(&input);
}
