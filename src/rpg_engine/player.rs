use crate::rpg_engine::stats::Body;
use crate::rpg_engine::stats::Mind;
use crate::rpg_engine::stats::Spirit;

use crate::rpg_engine::stats::CharacterStats;

use crate::rpg_engine::stats::PlayerStats;

use crate::rpg_engine::skills::Skill;
use crate::rpg_engine::skills::Skills;

use crate::rpg_engine::equipment::Equipment;

use crate::rpg_engine::inventory::{Inventory, Item};

use crate::rpg_engine::titles::Title;
use crate::rpg_engine::titles::Titles;

use crate::rpg_engine::effects::Effect;
use crate::rpg_engine::effects::Effects;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

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
    let king = Title::new("King", "A title granted to a recognized ruler.");

    let player_effects = Effect::new("Blessing", None);

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
