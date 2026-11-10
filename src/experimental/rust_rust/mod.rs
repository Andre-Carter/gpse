pub mod cargo;
pub mod language;
pub mod standard_library;

pub enum RustDivision {
    Language,
    StandardLibrary,
    Cargo,
    Reference,
}