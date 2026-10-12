#[derive(Debug)]
pub struct Mind {
    pub power: i32,
    pub will: i32,
    pub endurance: i32,
    pub perception: i32,
}

#[derive(Debug)]
pub struct Body {
    pub power: i32,
    pub will: i32,
    pub endurance: i32,
    pub perception: i32,
}

#[derive(Debug)]
pub struct Spirit {
    pub power: i32,
    pub will: i32,
    pub endurance: i32,
    pub perception: i32,
}

#[derive(Debug)]
pub struct CharacterStats {
    pub mind: Mind,
    pub body: Body,
    pub spirit: Spirit,
}

#[derive(Debug)]
pub struct PlayerStats {
    pub stats: CharacterStats,
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

/*
impl CharacterStats {
    pub fn new() -> Self {
        Self {

        }
    }
}
*/

impl PlayerStats {
    pub fn new() -> Self {
        Self {
            stats: CharacterStats {
                mind: Mind::new(),
                body: Body::new(),
                spirit: Spirit::new(),
            },
        }
    }
}

impl std::fmt::Display for Mind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Power: {}", self.power)?;
        writeln!(f, "Will: {}", self.will)?;
        writeln!(f, "Endurance: {}", self.endurance)?;
        writeln!(f, "Perception: {}", self.perception)?;
        Ok(())
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Power: {}", self.power)?;
        writeln!(f, "Will: {}", self.will)?;
        writeln!(f, "Endurance: {}", self.endurance)?;
        writeln!(f, "Perception: {}", self.perception)?;
        Ok(())
    }
}

impl std::fmt::Display for Spirit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Power: {}", self.power)?;
        writeln!(f, "Will: {}", self.will)?;
        writeln!(f, "Endurance: {}", self.endurance)?;
        writeln!(f, "Perception: {}", self.perception)?;
        Ok(())
    }
}

impl std::fmt::Display for CharacterStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "MIND")?;
        writeln!(f, "{}", self.mind)?;
        writeln!(f, "BODY")?;
        writeln!(f, "{}", self.body)?;
        writeln!(f, "SPIRIT")?;
        writeln!(f, "{}", self.spirit)?;
        Ok(())
    }
}

impl std::fmt::Display for PlayerStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "STATS")?;
        writeln!(f)?;
        writeln!(f, "{}", self.stats)?;
        Ok(())
    }
}

/*
pub fn character() {
    let mind = Mind::new();
    let body = Body::new();
    let spirit = Spirit::new();
}
*/
