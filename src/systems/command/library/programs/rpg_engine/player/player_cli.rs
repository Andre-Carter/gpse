use crate::programs::rpg_engine::gamestate::GameState;

//use std::io::{Write, stdin, stdout};

/*
fn read(input: &mut String) {
stdout().flush().expect("failed to flush");
stdin().read_line(input).expect("failed to read");
}
*/

pub fn print_player(state: &GameState) {
    print!("{}", state.player);
}


