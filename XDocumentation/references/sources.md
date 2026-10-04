# GPSE Sources
https://doc.rust-lang.org/rust-by-example/fn/methods.html

https://www.bible.com/

https://www.kingjamesbibleonline.org/1611-Bible/

https://pml.nist.gov/cuu/Constants/Table/allascii.txt
## Source Identification

Significant sources should receive stable identifiers.

Example:

* S001
* S002
* S003

Research documents should reference these identifiers when practical.

---

## Source Categories

### Rust

Official language and tooling documentation.

Examples of source types:

* Rust Reference
* The Rust Book
* Rust standard library documentation
* Rustonomicon
* Cargo documentation
* Rust compiler documentation
* Rust RFCs
* Rust Project repositories

---

### Mathematics

Sources concerning:

* mathematical definitions
* numerical methods
* equations
* constants
* mathematical notation
* numerical analysis
* dimensional analysis
* statistics
* probability

---

### Physics and Science

Sources concerning:

* physical constants
* mechanics
* thermodynamics
* electromagnetism
* quantum mechanics
* relativity
* astrophysics
* planetary science
* chemistry
* aerospace engineering

---

### Repositories

For significant repositories, record:

* source ID
* project name
* repository
* language
* purpose
* license
* relevant subsystem
* what GPSE learned
* research status

---

### Papers and Publications

Record:

* title
* author(s)
* publication
* date
* identifier/link
* subject
* relevance to GPSE

---

### Videos and Lectures

Record:

* title
* speaker/creator
* organization
* date
* platform
* subject
* relevant timestamps when useful
* GPSE relevance

---

### Standards and Specifications

Record standards that influence:

* units
* measurements
* data formats
* text encoding
* scientific representation
* interoperability
* engineering
* computing

---

## Source Evaluation

A source should be evaluated according to its role.

Possible classifications:

### PRIMARY

Original specification, paper, dataset, implementation, or authoritative documentation.

### SECONDARY

Analysis or explanation of primary material.

### EDUCATIONAL

Tutorial, lecture, course, or instructional material.

### COMMUNITY

Discussion, forum, issue tracker, or community knowledge.

### HISTORICAL

Material useful for understanding prior development.

A source can belong to more than one practical category when appropriate.

---

## Provenance

When GPSE incorporates factual data, the source should be identifiable whenever practical.

For important values, record enough provenance that a future developer can answer:

> Where did this value come from?

and:

> Why does GPSE represent it this way?

---

## Licensing

Before incorporating external material into GPSE, investigate its license and usage requirements.

Record relevant licensing information for incorporated material.

Open source does not mean license-free.

Publicly accessible does not automatically mean freely redistributable.

---

## Research Traceability

Where practical:

```text
Source
  ↓
Research Finding
  ↓
Architectural Decision
  ↓
Implementation
  ↓
Test
```

This creates a traceable path from external knowledge to GPSE implementation.

---

## Source Quality

GPSE should prefer authoritative and primary sources when available.

Secondary sources may be useful for explanation and discovery.

Community sources may be valuable for identifying practical problems, historical context, and implementation experience.

Important technical claims should be verified against stronger sources whenever practical.

---

## Source Maintenance

Sources may disappear, move, or change.

For important references, GPSE should consider recording:

* title
* author
* organization
* publication date
* version
* repository commit or release
* DOI or permanent identifier where applicable
* access date
* license

The goal is to preserve enough context that a future maintainer can identify the referenced work even if a URL changes.
