# GPSE Working Notes

This file contains temporary observations, questions, experiments, decisions-in-progress, and ideas that may later be promoted into formal GPSE documentation.

It is intentionally less formal than `principles.md`, `research.md`, and other permanent documentation.

---

## Current Questions

* What should GPSE's foundational data model look like?
* How should GPSE represent large bodies of structured text?
* How much data should be embedded directly into Rust?
* Where should external data formats fit?
* How should provenance be represented?
* How should GPSE handle large indexes?
* How should long-term preservation influence architecture?
* What foundational primitives can serve multiple domains?

---

## Current Architectural Experiments

### Bible

The Bible project is being explored as a large-scale Rust-native text/data implementation.

The immediate purpose is not simply Bible retrieval.

It is an opportunity to investigate:

* hierarchical data
* text representation
* indexing
* querying
* metadata
* provenance
* large static datasets
* CLI access
* long-term preservation
* reusable text infrastructure

---

## Research Leads

* Rust expressions
* Rust type system
* embedded resources
* large static datasets
* Unicode and UTF-8
* text indexing
* search algorithms
* data provenance
* reproducible builds
* software preservation
* long-lived software
* scientific data architecture
* existing scientific computing systems
* existing Rust scientific libraries

---

## Ideas Worth Investigating

* GPSE-native document model
* GPSE-native text model
* hierarchical identifiers
* generic collections
* reusable indexing infrastructure
* internal versus external representations
* deterministic data validation
* compile-time data generation
* data integrity checks
* large dataset benchmarks
* source-to-data provenance
* long-term compatibility strategy

---

## Things We Must Not Assume

* That something is original merely because we have not found it.
* That external formats are inherently bad.
* That embedded data is always better.
* That a large dataset automatically requires a database.
* That an abstraction is good merely because it is general.
* That today's hardware constraints will remain unchanged.
* That future developers will understand undocumented decisions.

---

## Historical Record

Important rejected approaches, mistakes, discoveries, and turning points may be recorded here before being promoted into formal documentation.

This file is allowed to be messy.

The polished documentation can come later.

---

## Future Promotion

When a note becomes sufficiently mature, determine whether it belongs in:

* `principles.md`
* `research.md`
* `sources.md`
* architecture documentation
* technical documentation
* code comments
* tests
* implementation

I've noted the GPSE symbolic-language idea: exploring pictograms, pictographs, glyphs, and pictoglyph-like representations as a future interface/language for GPSE, while keeping the underlying data machine-readable and authoritative.

The glyph represents the data — it does not become the data.

