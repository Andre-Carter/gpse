pub fn andre() {
    let mut raw_input_string: String = String::new();
    read(&mut raw_input_string);

    let input_chars: Vec<char> = input
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}