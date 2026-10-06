//RPG
//mod rpg;

pub struct Character {
    pub mind: Mind,
    pub body: Body,
    pub spirit: Spirit,
}

pub struct Mind {
    pub power: i32,
    pub will: i32,
    pub endurance: i32,
    pub perception: i32,
}

pub struct Body {
    pub power: i32,
    pub will: i32,
    pub endurance: i32,
    pub perception: i32,
}

pub struct Spirit {
    pub power: i32,
    pub will: i32,
    pub endurance: i32,
    pub perception: i32,
}

impl Mind {
    pub fn new() -> Self {
        Self {
            power: 0,
            will: 0,
            endurance: 0,
            perception: 0,
        }
    }
}

impl Body {
    pub fn new() -> Self {
        Self {
            power: 0,
            will: 0,
            endurance: 0,
            perception: 0,
        }
    }
}

impl Spirit {
    pub fn new() -> Self {
        Self {
            power: 0,
            will: 0,
            endurance: 0,
            perception: 0,
        }
    }
}

pub fn character() {
    let mind = Mind::new();
    let body = Body::new();
    let spirit = Spirit::new();

    println!("Mind Power: {}", mind.power);
    println!("Body Power: {}", body.power);
    println!("Spirit Power: {}", spirit.power);
}

pub struct Player {
    pub name: String,
    pub level: u32,
    pub stats: PlayerStats,
    pub skills: Vec<Skill>,
    pub equipment: Equipment<Item>,
    pub titles: Vec<Title>,
    pub effects: Vec<Effect>,
}

pub struct Name {
    pub name: String,
}

pub struct Level {
    pub level: u32,
}

pub struct PlayerStats {
    pub stats: Character,
}

pub struct Skills {
    pub skills: Vec<Skill>,
}

pub struct Equipment<Item> {
    pub weapon: Option<Item>,
    pub armor: Option<Item>,
    pub accessories: Vec<Item>,
}

pub struct Title {
    pub name: String,
    pub description: String,
}

pub struct Effect {
    pub name: String,
    pub duration: Option<u32>,
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
        Self {
            level: 0,
        }
    }
}

impl PlayerStats {
    pub fn new() -> Self {
        Self {
            stats: Character {
                mind: Mind::new(),
                body: Body::new(),
                spirit: Spirit::new(),
            },
        }
    }
}

impl Skills {
    pub fn new() -> Self {
        Self {
            skills: Vec::<Skill>::new(),
        }
    }
}

impl<Item> Equipment<Item> {
    pub fn new() -> Self {
        Self {
            weapon: None,
            armor: None,
            accessories: Vec::new(),
        }
    }
}

pub struct Item {
    pub name: String,
}

impl Title {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

impl Effect {
    pub fn new(name: &str, duration: Option<u32>) -> Self {
        Self {
            name: name.to_string(),
            duration, 
        }
    }
}

pub fn title() {
    
}

pub fn effect() {
    //let blessing = Effect::new("Blessing", Some(60));
}

pub fn player<Item>() {
    let player_name = Name::new();
    let player_level = Level::new();
    let player_stats = PlayerStats::new();
    let player_skills = Skills::new();
    let player_equipment = Equipment::<Item>::new();
    
    let king = Title::new(
        "King",
        "A title granted to a recognized ruler.",
    );
    
    let player_effects = Effect::new(
        "Blessing",
        None,
    );

    let player = Player {
        name: "Andre".to_string(),
        level: 1,
        stats: PlayerStats::new(),
        skills: Vec::new(),
        equipment: Equipment::new(),
        titles: Vec::new(),
        effects: Vec::new(),
    };
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
    //pub level: u32,
    //pub experience: u64,
}

impl Skill {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

pub fn create_skill() {
    let skills: Vec<Skill> = Vec::new();

    let stealth = Skill::new("Stealth");

    let skills: Vec<Skill> = vec![
        Skill::new("Stealth"),
    ];
}




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



//pub titles: Vec<Title>,

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

//Eventually effects could modify stats dynamically

// PLAYER 

//let player = Player {
//    skills: Vec::new(),
//};

