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

pub struct CharacterStats {
    pub mind: Mind,
    pub body: Body,
    pub spirit: Spirit,
}

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl std::fmt::Display for CharacterStats {
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

impl std::fmt::Display for PlayerStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "{}", self.stats);

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
