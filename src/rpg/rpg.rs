//RPG

pub struct Player {
    pub name: String,
    pub level: u32,
    pub stats: PlayerStats,
    pub skills: Skills,
    pub equipment: Equipment,
    pub titles: Vec<Title>,
}

pub struct PlayerStats {
    // CORE
    pub mind: i32,
    pub body: i32,
    pub spirit: i32,

    // MIND
    pub intelligence: i32,
    pub knowledge: i32,
    pub wisdom: i32,
    pub determination: i32,
    pub creativity: i32,

    // BODY
    pub health: i32,
    pub strength: i32,
    pub speed: i32,
    pub agility: i32,
    pub dexterity: i32,
    pub endurance: i32,
    pub reflex: i32,

    // SPIRIT
    pub soul: i32,
    pub will: i32,

    pub faith: i32,
    pub hope: i32,
    pub love: i32,
    

    pub discernment: i32,
    pub intuition: i32,
}

// DERIVED COMBAT STATS
/*
hp
block
dodge
parry
counter
accuracy
precision
resistance
regeneration
*/

pub struct Skill {
    pub name: String,
    pub level: u32,
    pub experience: u64,
}

pub struct Skills {
    pub navigation: Skill,
    pub cooking: Skill,
    pub crafting: Skill,
    pub healing: Skill,
    pub investigation: Skill,
}

//But I'd actually eventually consider:

pub skills: Vec<Skill>

//Archetype ≠ Role ≠ Build

pub enum Archetype {
    Tank,
    Damage,
    Healer,
    Defender,
    Support,
    Strategist,
}

pub enum Role {
    Navigator,
    Pilot,
    Medic,
    Soldier,
    Chef,
    Strategist,
    Chaplain,
}

//Build

//The player's actual combination of choices.

//A player could become:

/*Human
+ Ninja
+ Spy
+ Thief
+ Navigator
+ High Dexterity
+ High Stealth
*/


//Race can provide starting traits or modifiers, 
//but shouldn't necessarily dictate the entire character

pub enum Race {
    Human,
    Maiden,
    Oni,
}

//Your title idea is especially cool because it can be earned dynamically.

pub struct Title {
    pub name: String,
    pub description: String,
}

pub titles: Vec<Title>,

// examples
/*
Farmer
King
Mailman
Butler
Millionaire
Berserker
Explorer
*/

// a title can have effects
// might affect reputation/status/social interactions 
// without being a combat stat

//EFFECTS
/*
AURA
PASSIVE EFFECTS
BUFFS
DEBUFFS
CURSES
BLESSINGS
*/

pub struct Effect {
    pub name: String,
    pub duration: Option<u32>,
}

pub struct Player {
    pub effects: Vec<Effect>,
}

//Eventually effects could modify stats dynamically

pub struct Equipment {
    pub weapon: Option<Item>,
    pub armor: Option<Item>,
    pub accessories: Vec<Item>,
}