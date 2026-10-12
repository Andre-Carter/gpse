use crate::programs::rpg_engine::player::player::Player;

pub fn print_player() {
    let player = Player::new("Andre");
    print!("{player}");
}