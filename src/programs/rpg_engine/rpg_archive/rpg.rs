use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
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

impl std::fmt::Display for Mind {
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Power:")?;
        writeln!(f, "{}", self.power)?;

        writeln!(f, "Will:")?;
        writeln!(f, "{}", self.will)?;

        writeln!(f, "Endurance:")?;
        writeln!(f, "{}", self.endurance)?;

        writeln!(f, "Perception:")?;
        writeln!(f, "{}", self.perception)?;

        Ok(())
    }
}

impl std::fmt::Display for Body {
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Power:")?;
        writeln!(f, "{}", self.power)?;

        writeln!(f, "Will:")?;
        writeln!(f, "{}", self.will)?;

        writeln!(f, "Endurance:")?;
        writeln!(f, "{}", self.endurance)?;

        writeln!(f, "Perception:")?;
        writeln!(f, "{}", self.perception)?;

        Ok(())
    }
}

impl std::fmt::Display for Spirit {
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Power:")?;
        writeln!(f, "{}", self.power)?;

        writeln!(f, "Will:")?;
        writeln!(f, "{}", self.will)?;

        writeln!(f, "Endurance:")?;
        writeln!(f, "{}", self.endurance)?;

        writeln!(f, "Perception:")?;
        writeln!(f, "{}", self.perception)?;

        Ok(())
    }
}

pub struct Character {
    pub mind: Mind,
    pub body: Body,
    pub spirit: Spirit,
}

pub fn character() {
    let mind = Mind::new();
    let body = Body::new();
    let spirit = Spirit::new();
}

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Mind:")?;
        writeln!(f, "{}", self.mind)?;

        writeln!(f, "Body:")?;
        writeln!(f, "{}", self.body)?;

        writeln!(f, "Spirit:")?;
        writeln!(f, "{}", self.spirit)?;

        Ok(())
    }
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

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        write!(f, "{}", self.level)?;
        
        Ok(())
    }
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

pub struct Skill {
    pub name: String,
    //pub level: u32,
    //pub experience: u64,
}

pub struct Equipment<Item> {
    pub weapon: Option<Item>,
    pub armor: Option<Item>,
    pub accessories: Vec<Item>,
}

pub struct Item {
    pub name: String,
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

impl Skill {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
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

impl Title {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

impl std::fmt::Display for PlayerStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stats);

        Ok(())
    }
}

impl std::fmt::Display for Skills {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Skills:")?;

        for skill in &self.skills {
            writeln!(f, "- {}", skill)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name);

        Ok(())
    }
}

impl<Item: std::fmt::Display for Equipment<Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Weapon:")?;

        match &self.weapon {
            Some(item) => writeln!(f, "{}", item)?,
            None => writeln!(f, "None")?,
        }

        writeln!(f, "Equipment:")?;

        match &self.equipment {
            Some(item) => writeln!(f, "{}", item)?,
            None => writeln!(f, "None")?,
        }

        writeln!(f, "Accessories:")?;

        match &self.accessories {
            Some(item) => writeln!(f, "{}", item)?,
            None => writeln!(f, "None")?,
        }

        Ok(())
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
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

    println!("{}", player.name);
    println!("{}", player.level);
    println!("{}", player.stats);
    println!("{}", player_skills);
    println!("{}", player_equipment);
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

