use crate::programs::rpg_engine::player::player::Player;

pub struct GameState {
    pub player: Player,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new("Andre"),
        }
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }
}
