use crate::programs::rpg_engine::player::player::Player;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub struct GameState {
    pub player: Player,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new("Andre"),
        }
    }
}

pub fn print_player(state: &GameState) {
    print!("{}", state.player);
}

pub fn player_set_name(state: &mut GameState, name: &str) {
    //print("GPSE> COMMAND> RPG> PLAYER> RENAME> 

    state.player.set_name(name);
}
