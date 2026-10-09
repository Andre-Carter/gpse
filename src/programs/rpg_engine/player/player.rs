//use crate::rpg_engine::stats::Body;
//use crate::rpg_engine::stats::Mind;
//use crate::rpg_engine::stats::Spirit;

//use crate::rpg_engine::stats::CharacterStats;

use crate::programs::rpg_engine::player::stats::PlayerStats;

use crate::programs::rpg_engine::player::skills::Skill;
use crate::programs::rpg_engine::player::skills::Skills;

use crate::programs::rpg_engine::player::equipment::Equipment;

use crate::programs::rpg_engine::player::inventory::{Inventory, Item};

use crate::programs::rpg_engine::player::titles::Title;
use crate::programs::rpg_engine::player::titles::Titles;

use crate::programs::rpg_engine::player::effects::Effect;
use crate::programs::rpg_engine::player::effects::Effects;

pub struct Name {
    pub name: String,
}

pub struct Level {
    pub level: u32,
}

pub struct Player {
    pub name: String,
    pub level: u32,
    pub stats: PlayerStats,
    pub skills: Skills<Skill>,
    pub equipment: Equipment<Item>,
    pub inventory: Inventory<Item>,
    pub titles: Titles<Title>,
    pub effects: Effects<Effect>,
}

impl Name {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }
}

impl Level {
    pub fn new() -> Self {
        Self { level: 0 }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        write!(f, "{}", self.level)?;

        Ok(())
    }
}

pub fn player<Item: std::fmt::Display>() {
    let player = Player {
        name: "Andre".to_string(),
        level: 1,
        stats: PlayerStats::new(),
        skills: Skills::new(),
        equipment: Equipment::new(),
        inventory: Inventory::new(),
        titles: Titles::new(),
        effects: Effects::new(),
    };

    println!("{}", player.name);
    println!("{}", player.level);
    println!("{}", player.stats);
    println!("{}", player.equipment);
}
