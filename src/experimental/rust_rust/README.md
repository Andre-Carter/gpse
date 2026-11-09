Rust Rust — A compiled, structured reference handbook for the Rust programming language.

src/
└── archive/
    └── rust_rust/
        ├── mod.rs
        ├── language/
        ├── standard_library/
        ├── cargo/
        └── reference/

pub enum RustDivision {
    Language,
    StandardLibrary,
    Cargo,
    Reference,
}

pub fn lookup_rust(division: RustDivision) {
    // Retrieve the requested division.
}

lookup_rust(Language)
lookup_rust(Language::Ownership)
lookup_rust(StandardLibrary::Collections)
lookup_rust(Cargo::Commands)

One important consideration: Rust itself changes over time. Rust Rust should identify the Rust version it documents, distinguish stable from experimental features, and retain source references. Otherwise, the handbook could become misleading as the language evolves.