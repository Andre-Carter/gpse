//use crate::rpg_engine::stats::Body;
//use crate::rpg_engine::stats::Mind;
//use crate::rpg_engine::stats::Spirit;

//use crate::rpg_engine::stats::CharacterStats;

use std::fmt;

use crate::programs::rpg_engine::player::stats::PlayerStats;
use crate::programs::rpg_engine::player::skills::{Skill, Skills};
use crate::programs::rpg_engine::player::equipment::Equipment;
use crate::programs::rpg_engine::player::inventory::{Inventory, Item};
use crate::programs::rpg_engine::player::titles::{Title, Titles};
use crate::programs::rpg_engine::player::effects::{Effect, Effects};

#[derive(Debug)]
pub struct PlayerName {
    pub name: String,
}

#[derive(Debug)]
pub struct PlayerLevel {
    pub level: u32,
}

#[derive(Debug)]
pub struct Player {
    pub name: PlayerName,
    pub level: PlayerLevel,
    pub stats: PlayerStats,
    pub skills: Skills<Skill>,
    pub equipment: Equipment<Item>,
    pub inventory: Inventory<Item>,
    pub titles: Titles<Title>,
    pub effects: Effects<Effect>,
}

impl PlayerName {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl PlayerLevel {
    pub fn new(level: u32) -> Self {
        Self { level }
    }
}

impl fmt::Display for PlayerName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for PlayerLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.level)
    }
}

impl Player {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: PlayerName::new(name),
            level: PlayerLevel::new(1),
            stats: PlayerStats::new(),
            skills: Skills::new(),
            equipment: Equipment::new(),
            inventory: Inventory::new(),
            titles: Titles::new(),
            effects: Effects::new(),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "PLAYER")?;
        writeln!(f)?;
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Level: {}", self.level)?;
        writeln!(f, "{}", self.stats)?;
        writeln!(f, "{}", self.equipment)?;
        Ok(())
    }
}

pub fn print_player() {
    let player = Player::new("Andre");
    println!("{player}");
}


