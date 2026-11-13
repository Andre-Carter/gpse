pub mod cargo;
pub mod compiler;
pub mod environment;
pub mod language;
pub mod reference;
pub mod standard_library;
pub mod troubleshooting;

pub mod cli;
pub mod metadata;

pub enum RustDivision {
    Cargo,
    Compiler,
    Environment,
    Language,
    Reference,
    StandardLibrary,
    Troubleshooting,
}
