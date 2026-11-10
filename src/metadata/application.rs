pub const DISPLAY_NAME: &str = "GPSE";
pub const WELCOME_TITLE: &str = "WELCOME TO GPSE";

// move data from foundation: termination, and elsewhere to metadata home

pub const ACRONYM: &str = "g.p.s.e";
pub const CAP_ACRONYM: &str = "G.P.S.E";

//pub const APPLICATION_DESCRIPTION: &str =
    //"General Purpose Simulation Engine";
pub const FULL_NAME: &str = "general purpose simulation engine";
pub const FULL_DISPLAY_NAME: &str = "GENERAL PURPOSE SIMULATION ENGINE";

/*
pub struct MetadataEntry {
    pub key: &'static str,
    pub value: &'static str,
}

pub static METADATA: &[MetadataEntry] = &[
    MetadataEntry {
        key: "project.name",
        value: "gpse",
    },
    MetadataEntry {
        key: "application.welcome",
        value: "WELCOME TO GPSE",
    },
    MetadataEntry {
        key: "command.title",
        value: "COMMAND SYSTEM",
    },
];

pub fn get(key: &str) -> Option<&'static str> {
    METADATA
        .iter()
        .find(|entry| entry.key == key)
        .map(|entry| entry.value)
}

let title = get("application.welcome");
*/