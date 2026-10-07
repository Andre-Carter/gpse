use crate::rpg_engine::inventory::Item;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub struct Equipment<Item> {
    pub primary: Option<Item>,
    pub secondary: Option<Item>,

    pub helmet: Option<Item>,
    pub chestplate: Option<Item>,
    pub gauntlets: Option<Item>,
    pub gloves: Option<Item>,
    pub leggings: Option<Item>,
    pub boots: Option<Item>,

    pub accessory_1: Option<Item>,
}

impl<Item> Equipment<Item> {
    pub fn new() -> Self {
        Self {
            primary: None,
            secondary: None,

            helmet: None,
            chestplate: None,
            gauntlets: None,
            gloves: None,
            leggings: None,
            boots: None,

            accessory_1: None,
        }
    }
}

impl<Item: std::fmt::Display> std::fmt::Display for Equipment<Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Primary:")?;
        match &self.primary {
            Some(item) => writeln!(f, "{item}")?,
            None => writeln!(f, "None")?,
        }

        writeln!(f, "Secondary:")?;
        match &self.secondary {
            Some(item) => writeln!(f, "{item}")?,
            None => writeln!(f, "None")?,
        }

        writeln!(f, "Helmet:")?;
        match &self.helmet {
            Some(item) => writeln!(f, "{item}")?,
            None => writeln!(f, "None")?,
        }

        writeln!(f, "Chestplate:")?;
        match &self.chestplate {
            Some(item) => writeln!(f, "{item}")?,
            None => writeln!(f, "None")?,
        }

        writeln!(f, "Gauntlets:")?;
        match &self.gauntlets {
            Some(item) => writeln!(f, "{item}")?,
            None => writeln!(f, "None")?,
        }

        writeln!(f, "Gloves:")?;
        match &self.gloves {
            Some(item) => writeln!(f, "{item}")?,
            None => writeln!(f, "None")?,
        }

        writeln!(f, "Leggings:")?;
        match &self.leggings {
            Some(item) => writeln!(f, "{item}")?,
            None => writeln!(f, "None")?,
        }

        writeln!(f, "Boots:")?;
        match &self.boots {
            Some(item) => writeln!(f, "{item}")?,
            None => writeln!(f, "None")?,
        }

        writeln!(f, "Accessory_1:")?;
        match &self.accessory_1 {
            Some(item) => writeln!(f, "{item}")?,
            None => writeln!(f, "None")?,
        }

        Ok(())
    }
}
