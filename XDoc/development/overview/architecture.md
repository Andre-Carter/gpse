A MAP OF GPSE
or future architecture ideas

# Future Architecture Idea: Mirrored Command Libraries

**Status:** Deferred research idea — not part of the current implementation.

## Concept

Explore whether GPSE command modules should eventually have mirrored organization between the command-handler hierarchy and the reusable command library hierarchy.

## Current decision

Use a single command/library organization for the current CLI foundation. Avoid maintaining duplicate structures before there is a demonstrated need.

## Revisit when

* Multiple interfaces consume the same command families.
* Shared command helpers become difficult to locate.
* The command hierarchy grows enough that mirrored organization provides measurable benefits.

Any future mirror must have a clearly distinct responsibility and must not duplicate implementation merely for structural symmetry.

# Proposed Future System: CONTROL

**Status:** Exploratory concept
**Priority:** Low — deferred
**Implementation:** Not approved
**Scope:** Future GPSE architecture

## 1. Purpose

`CONTROL` is a proposed future major system within the General Purpose Simulation Engine (GPSE).

Its eventual purpose, responsibilities, and relationship with existing systems have not yet been established. This document preserves the concept for future research and architectural discussion without prematurely committing GPSE to an implementation.

The current priority is to advance the existing GPSE mission. CONTROL must not distract from or displace higher-priority work.

## 2. Initial Concept

CONTROL may eventually provide an operational coordination layer across GPSE systems. Potential areas of investigation include:

* Coordination of operations across multiple systems.
* Mission and objective management.
* Command validation and execution workflows.
* Scheduling, sequencing, and dependency management.
* Monitoring operational state and responding to failures.
* Optional automated or autonomous behavior within defined constraints.

These are research possibilities, not approved requirements.

## 3. Possible Architectural Relationships

Future investigation should establish how CONTROL relates to existing GPSE components, including:

* `SYSTEMS` — capabilities such as navigation, communication, and telemetry.
* `SIMULATION` — entities and simulated state.
* `cli` and command infrastructure — user interaction and command execution.
* Future mission, automation, or operational-management components.

CONTROL must not duplicate existing responsibilities without a demonstrated architectural need.

If developed, its boundaries should be explicit: it should coordinate other systems where appropriate rather than independently reimplement their domain logic.

## 4. Open Questions

The following questions remain unresolved:

1. Is CONTROL primarily a mission-management system, a system-orchestration layer, an autonomous-control system, or some combination?
2. What responsibilities cannot be adequately handled by existing GPSE systems?
3. Should CONTROL be a top-level system, a subsystem, or a capability distributed across existing components?
4. What interfaces would it require?
5. What level of automation or autonomy, if any, should it support?
6. What validation, safety, and operational constraints would apply?
7. What concrete GPSE requirement would justify beginning implementation?

These questions should be answered through future research and practical experience, not assumption.

## 5. Development Policy

Until a concrete need emerges:

* Do not create a CONTROL implementation solely to reserve its place in the architecture.
* Do not introduce dependencies on CONTROL into current development.
* Do not allow this concept to delay active GPSE priorities.
* Record relevant findings and revisit the concept when existing work establishes a genuine need.

If implementation is eventually proposed, begin with a defined problem, documented responsibilities, alternatives, and testable requirements.

## 6. Review Conditions

Revisit this document when:

* Existing systems demonstrate a recurring need for cross-system coordination.
* A future simulation or mission requires operational behavior that current architecture cannot cleanly provide.
* Research establishes a clear responsibility and useful interface for CONTROL.

The concept may be revised, split into separate systems, incorporated into existing systems, or abandoned if it provides no distinct value.

## 7. Guiding Principle

**Preserve the idea without committing the architecture prematurely.**

CONTROL remains a possible future direction for GPSE. Its name and potential scope are recorded for continuity, while its actual purpose must be established through demonstrated needs and deliberate architectural decisions.

check above ^ for systems> systems on mission/control/orchestration/automation

- SYSTEMS> CONTROL
    - main purpose: evaluate mission request, approve/deny mission request 

- SYSTEMS> MISSION
    - main purpose: define missions, protocols,

- SYSTEMS> MISSION> PROTOCOLS> no_man_left_behind_protocol

- mission-control
    
- SYSTEMS> PERSONNEL 
    - id, role assignment, availibitiy, accountability status
    - ledger    Record assignments, status changes, incidents, actions taken, and resolution
    - communitcations (Send check-ins, alerts, requests for assistance, and status reports)
    - security  (Verify identity and authorization, protect personnel records, control access)
    - naviation (last-known or reported locations where relevant)
    - telemetry (Report system-derived status or location observations, with source and timestamp)
    - mission/simulation (Define operational objectives, accountability requirements, and completion conditions)

- SIMULATION> CORE

- SIMULATION> ECONOMY

- SYSTEMS> SECURITY

So where does no_man_left_behind_protocol belong?

My revised recommendation: under the mission/protocol architecture, with personnel as a domain it uses.

The protocol defines what must happen; the personnel system provides the data and capabilities needed to carry it out.

For example, a mission definition could specify:

Mission type: personnel deployment.

Personnel accountability: mandatory.

Required protocol: no_man_left_behind_protocol.

Completion requirements: all assigned personnel accounted for, or unresolved cases formally handled according to the applicable rules.

Authorization: evaluated by CONTROL against mission requirements.

The protocol could then coordinate with communication, ledger, security, navigation, and telemetry without owning those systems.

One subtle but important distinction: CONTROL's approval should evaluate whether the mission meets its authorization requirements. It shouldn't magically guarantee that the mission is safe or that all protocol conditions will remain satisfied during execution. Those conditions may need continuous monitoring and explicit handling during the mission.

## D-XXX — Mission Authorization and Personnel Protocols

**Status:** Proposed architectural direction
**Implementation:** Deferred

### Decision Direction

CONTROL is envisioned as the system responsible for evaluating mission requests and approving or denying authorization.

MISSIONS defines mission types, parameters, requirements, and applicable operational protocols.

The `no_man_left_behind_protocol` should be investigated as a mission-level protocol that relies on personnel accountability capabilities and may coordinate with communication, ledger, security, navigation, and telemetry systems.

### Architectural Principle

Mission definitions specify which protocols apply. Protocols define required behavior. Domain systems provide the capabilities and data needed to execute those protocols. CONTROL evaluates mission authorization.

### Follow-up

Define the boundaries between CONTROL, MISSIONS, personnel, and protocol execution before implementation. Preserve this direction as an architectural concept without elevating it above current GPSE priorities.

The key distinction is: CONTROL decides whether a mission may proceed; MISSIONS defines what proceeding entails. The protocol specifies obligations that must be honored during that mission.

2. What each kernal would own

System
- Potential responsibility of its kernal

command
- Dispatch commands and coordinate command handlers

communication
- Coordinate message processing, routing, and delivery

control
- Evaluate authorization and coordinate operational decisions

mission
- Define, validate, and manage mission specifications and lifecycle

navigation
- Coordinate navigation requests and route or trajectory calculations

personnel
- Coordinate personnel records, assignments, and accountability

security
- Coordinate authentication, authorization, and security policy enforcement

telemetry
- Coordinate collection, processing, and reporting of measurements and status

command> formatting
or 
command> kernal> formatting
and 
example.
command> library> programs> rpg_engine> formmating

// section

# Geometry — Organization & Architecture

**Project:** GPSE  
**Domain:** Mathematics  
**Status:** PROPOSED  
**Version:** 0.0.1

## 1. Objective

Establish an extensible organizational structure for GPSE's geometry domain.

The organization should reflect mathematical relationships rather than create arbitrary directories for anticipated functionality.

## 2. Foundational Organization

Proposed hierarchy:

```text
mathematical/
└── geometry/
    ├── primitives/
    ├── shapes/
    │   ├── two_dimensional/
    │   └── three_dimensional/
    └── transformations/
```

This hierarchy is conceptual and is not yet an approved implementation.

## 3. Shapes

Shapes are the primary organizational domain for geometric objects.

Potential two-dimensional shapes:
- Circle
- Triangle
- Square
- Rectangle
- Polygon

Potential three-dimensional shapes:
- Sphere
- Cube
- Cuboid
- Cylinder
- Cone
- Polyhedron

## 4. Shape Measurements

Measurements belong conceptually to their respective shapes.

Examples:

**Circle**
- Radius
- Diameter
- Circumference
- Area

**Triangle**
- Side lengths
- Angles
- Perimeter
- Area

**Sphere**
- Radius
- Diameter
- Surface area
- Volume

## 5. Shape Properties

Properties describe characteristics of geometric objects.

Examples:
- Symmetry
- Number of sides
- Curvature
- Regularity
- Angles
- Faces
- Edges
- Vertices

Distinguish descriptive properties from quantitative measurements.

## 6. Architectural Considerations

1. Avoid duplicating shared mathematical operations.
2. Keep dimensional distinctions explicit.
3. Preserve reusable geometric primitives.
4. Avoid empty modules and premature abstractions.
5. Allow future higher-dimensional geometry without requiring its immediate implementation.
6. Keep mathematical definitions independent of CLI presentation.

## 7. Outstanding Decisions

- Should measurements be implemented within individual shape modules?
- Which geometric primitives should be shared?
- How should dimensionality be represented in the underlying types?
- Should higher-dimensional geometry eventually be supported?
- How should geometry integrate with physical and simulation domains?

## 8. Implementation Status

No implementation is authorized by this proposal alone.

Architecture must be evaluated before creating the corresponding source modules.

// section

src\foundation
foundation\format (format, display, spacing, indentation)

src/
└── foundation/
    └── parsing/
        ├── mod.rs
        │
        ├── input/
        │   ├── mod.rs
        │   └── input.rs
        │
        ├── lexical/
        │   ├── mod.rs
        │   ├── tokenizer.rs
        │   └── tokens.rs
        │
        ├── syntax/
        │   ├── mod.rs
        │   └── grammar.rs
        │
        ├── interpretation/
        │   ├── mod.rs
        │   └── expressions.rs
        │
        └── errors/
            ├── mod.rs
            └── parsing_error.rs

the intent: gpse> foundation\text> parsing; is the home for general purpose reusable parsing infrastructure that other GPSE systems are built with. Pro-error handling. From this we can extend our command parser and expression (math) parser.

foundation/
└── input/
    ├── normalization/
    └── parsing/

// the refined system here >

src/
└── foundation/
    └── input/
        ├── mod.rs
        │
        └── text/
            ├── mod.rs
            ├── normalization/
            ├── tokenization/
            ├── parsing/
            └── validation/

foundation/
└── input/
    ├── text/
    ├── binary/
    ├── numerical/
    ├── structured/
    └── ...

The separation of responsibilities
Layer	Responsibility
foundation::input	General input infrastructure
foundation::input::text	Text input processing
normalization	Standardizes representations
tokenization	Identifies individual tokens
parsing	Interprets structure and meaning
validation	Checks structural correctness
systems::command	Coordinates command requests and dispatch

One important architectural rule: foundation::input should not depend on systems::command. The dependency should flow in the opposite direction.

// section

/*
let mut command: String = String::new();

read(&mut command);

let command = command.trim();

NEW IMPLEMENTATION

Now you can replace the three lines with:

let command = read_text();

And import it:

use crate::foundation::input::text::read_text;

2. Why this is useful

We have consolidated a repeated operation:

Operation	        Responsibility

Create String	    Input acquisition
Flush stdout	    Ensure prompt visibility
Read stdin	        Receive input
Trim whitespace	    Basic text normalization
Return String	    Provide normalized input

3. An important architectural consideration
Eventually, we might want to separate these two operations:
pub fn read_text() -> io::Result<String>


and
pub fn normalize_text(input: &str) -> String


Why? Because text might come from a terminal, a file, a network connection, or another GPSE system. Normalization shouldn't require terminal access.

For now, though, I'd actually implement the error-aware read_text() signature above rather than using expect(). It gives callers the ability to handle EOF and I/O errors without crashing.

My recommendation: establish read_text() as the standard input-acquisition function, and keep normalization independently reusable. That gives GPSE a clean foundation for future CLI standardization.

// section symobolics

pub struct Symbol {
    pub id: &'static str,
    pub name: &'static str,
    pub glyph: &'static str,
    pub category: SymbolCategory,
    pub description: &'static str,
}


But I would not finalize this structure yet.

A real symbolic record may also need:
- Alternative names and aliases.
- Unicode code point or sequence.
- Origin and historical period.
- Language or writing system.
- Domain associations.
- Multiple interpretations.
- Sources and references.
- Rendering variants.
And critically, we should not assume that every symbol has exactly one meaning.

Production Ready Scalable Maintainable

bin/ (additional binaries e.g. worker, cli, cron jobs)
config/ (configuration, loading, and management(toml, env, etc.))
domains/ (core domain models and business entities)
handlers/ (request handlers e.g. HTTP endpoints)
services/ (business logic and usecases)
repository/ (data access layer. DB, cache, external API)
routes/ (route definitions and api wiring)
middleware/ (cross-cutting concerns. auth, logging, CORS)
utils/ (reusable utilites. errors, helpers, etc.)
migrations/ (database migrations e.g. SQL with sqlx or diesel)
tests/ (unit and intergration test)
benches/ (performance benchmarks. Criterion)
scripts/ (build, setup, and deployment scripts)
deploy/ (container and orchestration files. Docker, KBs)

Status: Reference only, not adopted. Source: web-service template (handlers/routes/middleware don’t apply). Possibly adopt: tests/, benches/, examples/, src/bin/. Revisit when GPSE has integration tests or benchmarks worth separating.

I’d pick a different name than test. What’s in the folder isn’t tests, and the word is overloaded in Rust.

What theory holds today: test_theory.rs has a Switch type and logic_cli, an interactive logic-gate toy. It’s a sandbox for ideas, not a test suite. Nothing in it uses #[test] or assert! except test_switch(), which is called from nowhere.

Why test is a poor fit

Rust already gives “test” a strong meaning: #[cfg(test)] modules, #[test] functions, and the tests/ directory for integration tests. A module named test next to those would make “where are the tests?” ambiguous.
There’s also a built-in test crate (libtest, nightly-only). A root mod test usually compiles fine, but it’s the same kind of name overlap as core, which you just removed.
Your own naming principle is to name things by what they contain. Real tests belong in tests/ and inline #[cfg(test)] modules, so a test module would also be a trap for the wrong code.

Better names, closest to your project’s language

Name	Why
experiments	Mirrors XDoc/research/experiments/, so code and docs line up. My pick.
furnace	Your lore (“ingots go in the furnace before they become permanent architecture”). Fun, but less discoverable for outsiders.
sandbox	Clear and conventional.
lab	Short.

// section

1. Manifesto principles
These define what GPSE believes and stands for.

They govern the identity and philosophy of the project.

Examples:
- Long-term vision.
- Organization and accessibility.
- Preservation of knowledge.
- Openness and interoperability.
- Deterministic design as a project philosophy.
- Accessibility of scientific and technical knowledge.
2. Development principles
These define how GPSE is engineered.

They govern technical decisions and implementation.

Examples:
- Correctness before optimization.
- Prefer immutable data.
- Compile-time constants where appropriate.
- Explicit error handling.
- Test critical behavior.
- Avoid unnecessary dependencies.
- Measure before optimizing.

3. Naming candidates

Manifesto document	    Development document

project_principles.md	engineering_principles.md
core_beliefs.md	        development_principles.md
founding_principles.md	engineering_doctrine.md
project_philosophy.md	technical_principles.md

My preferred pairing:
Manifesto
project_principles.md

The philosophy, values, and foundational beliefs of GPSE.


Development
engineering_principles.md

The technical rules and decision-making principles governing GPSE engineering.


I particularly like engineering principles over development principles because engineering covers architecture, testing, verification, maintenance, and implementation—not just writing code.
One deeper distinction
A manifesto principle might say:
GPSE prioritizes correctness and reliability.

An engineering principle would define how that belief is enforced:
Mission-critical operations MUST have explicit correctness requirements and appropriate verification.

One expresses the belief. The other translates it into engineering practice.

My vote: project_principles.md + engineering_principles.md.

Neither rename is finalized yet. What do you think, brother?

// section

The important thing is that we're separating two different questions:
- Manifesto: What does GPSE believe, value, and aspire to become?
- Development: How do we actually design, build, test, maintain, and improve GPSE?
Those are related, but they should not be mixed.
1. Principles vs. standards
I would establish this distinction before moving any files.
Concept	Purpose	Example
Principle	Guides decisions and judgment	Prefer deterministic behavior.
Standard	Defines a requirement or convention	Mission-critical calculations MUST have defined units.
Procedure	Defines how work is performed	Run the test suite before committing.
Review	Evaluates work against expectations	Inspect correctness, safety, and maintainability.
Overview	Explains organization and purpose	Describes the development system and its components.

The distinction I particularly want to preserve:

Principles guide. Standards govern. Procedures instruct. Reviews evaluate.

A principle can allow judgment. A standard should have a clearly defined expectation, scope, and means of verification.

// section

Concept	        Purpose	                                

* Principle     Guides decisions and judgment (Ex. Prefer deterministic behavior)
* Standard	    Defines a requirement or convention (Ex. Mission-critical calculations MUST have defined units)
* Procedure	    Defines how work is performed (Ex. Run the test suite before committing)
* Review	    Evaluates work against expectations (Ex. Inspect correctness, safety, and maintainability)
* Overview	    Explains organization and purpose (Ex. Describes the development system and its components)

// section

* gpse\vault\compiler_test
* gpse\vault\experiments
* gpse\vault\prototypes
* gpse\vault\retired

* gpse\vault\templates\textbook

// section

* gpse\systems\command
- Terminal: prompts, read loops, menus, colors, dispatch

* gpse\foundation\format

// section

* gpse::foundation::text::unicode

// section date_time, date/time, date time, calendar, temporal

physical::dimensions::time.rs

temporal -> physics 
not physics -> temporal

warning: mathematical::dimensional may contain time

note: system_time, mathmatical time calculations, units and math time dimensions, physical time dimensions, physical time calculations, and sim time

notice: future development of simulation::time, ticks, temporal

log: 
Gregorian calendar: general civil calendar.
Julian calendar: historical calendar.
Lunar calendar: astronomical/cultural calendar system.
Astrological calendar: domain-specific interpretation and organization of dates and celestial events.

temporal/
├── calendar/
├── date/
├── time/
├── duration/
├── instant/
├── clock/
└── timezone/

important: avoid accidentally making Gregorian assumptions universal

7. My proposed architectural principles
Before implementation, I'd like us to establish these:
1. Calendar independence: Temporal calculations should not be restricted to one calendar system.
2. Unit clarity: Durations and physical time measurements must have explicit units.
3. Representation independence: A date, instant, duration, and time-of-day should not be confused.
4. Determinism: Calendar calculations and temporal conversions should produce predictable results under defined rules.
5. Domain independence: Physics, astrology, simulation, and programs should be able to consume temporal functionality without owning its core implementation.

Do you envision GPSE's temporal system as a general-purpose date/time library, or as a comprehensive temporal engine?

The latter could eventually handle calendar systems, astronomical time, time scales, synchronization, historical dates, simulation clocks, and temporal calculations.

I strongly suspect the second direction is worth investigating. It would justify a dedicated temporal/ domain rather than foundation/utilities/date_time/.

And I would preserve the existing programs/date_time implementation until we've inspected its contents. No migration before we understand what we've already built.

* Brother, I think we're approaching one of GPSE's most important architectural boundaries. Your note identifies the right concern: time is used throughout GPSE, but that does not mean every system should implement time independently, nor does it mean every concept involving time belongs in one central module.

I want to preserve your central intention:
Build a temporal foundation that other domains can depend on without taking ownership of their domain-specific calculations, data, or behavior.

That is the direction I'd investigate.

There is one particularly important distinction I want to make throughout this review:

A shared temporal foundation is not the same thing as a universal time calculator.

GPSE should provide shared representations, well-defined conversions, calendar rules, and temporal primitives. Mathematics, physics, science, and simulation should retain their own domain-specific responsibilities.

* temporal owns general temporal representations and operations.

physical owns physical definitions, measurements, dimensional analysis, and physical equations.

mathematical owns mathematical operations and abstractions.

simulation owns simulation-specific progression and scheduling.

// section 

Name	Purpose	Example
logs	Records events as they happen	Application started; calculation failed
ledger	Maintains an organized, deliberate record	Architecture decision; module completed
archive	Preserves historical material	Old designs, snapshots, research
journal	Records chronological observations and thoughts	Development notes, experiments

gpse/records/

This is a proposed umbrella for information-management systems, not a directory we're creating now.
Future component	Proposed home	Responsibility
Documentation	docs/	Manuals, guides, technical documentation
Journal	records/journal/	Chronological observations and development entries
Logs	records/logs/	Operational events, errors and execution history
Ledger	records/ledger/	Structured records and accounting of information
Research	records/research/	Research findings and supporting references

// gpse::systems::communication

src/
└── systems/
    └── communication/
        ├── mod.rs
        │
        ├── identity/
        │   ├── mod.rs
        │   ├── user.rs
        │   └── address.rs
        │
        ├── messaging/
        │   ├── mod.rs
        │   ├── message.rs
        │   ├── conversation.rs
        │   ├── participant.rs
        │   └── attachment.rs
        │
        ├── protocol/
        │   ├── mod.rs
        │   ├── packet.rs
        │   ├── serialization.rs
        │   └── routing.rs
        │
        ├── transport/
        │   ├── mod.rs
        │   ├── network.rs
        │   └── connection.rs
        │
        ├── security/
        │   ├── mod.rs
        │   ├── encryption.rs
        │   ├── authentication.rs
        │   └── permissions.rs
        │
        └── storage/
            ├── mod.rs
            ├── message_store.rs
            └── conversation_store.rs
1. messaging
    - A conventional messaging application: conversations, contacts, messages, timestamps, attachments, read status, and notifications.
2. communication infrastructure
    - The underlying engine: message delivery, routing, identity, addressing, protocols, serialization, encryption, and transport.
3. machine-to-machine communication
    - GPSE instances communicating with other GPSE instances, simulation entities, external programs, and eventually autonomous agents.

Build level 2 as the foundation, level 1 as the first application and level 3 as a longer-term capability

Identity	Who is communicating?
Messaging	What is being communicated?
Protocol	How is information represented and interpreted?
Transport	How does information travel?
Security	Who can access or modify information?
Storage	What information is retained?

pub struct Message {
    pub id: MessageId,
    pub sender: UserId,
    pub content: String,
    pub timestamp: Timestamp,
}

Later, the protocol layer can serialize that message, the transport layer can deliver it, and the storage layer can preserve it.

// SECTION

Establish systems/command/ as the shared command-system home.
Keep library beside core.
Preserve mathematical_cli() -> numeric_cli() as the handler hierarchy.
Keep the double-mirror concept documented as deferred.
Update module declarations and verify the refactor with cargo fmt -- --check and cargo check.

// SECTION

General Purpose Simulation Engine *
                    ┌──────────────────────┐
                    │     Application      │
                    │   / Simulation Model │
                    └──────────┬───────────┘
                               │
                    ┌──────────▼───────────┐
                    │    GPSE Runtime      │
                    │                      │
                    │  lifecycle           │
                    │  scheduling          │
                    │  time                │
                    │  execution           │
                    │  checkpoints         │
                    └──────────┬───────────┘
                               │
             ┌─────────────────┼─────────────────┐
             │                 │                 │
      ┌──────▼──────┐  ┌──────▼──────┐  ┌──────▼──────┐
      │    State    │  │   Events    │  │   Systems   │
      │             │  │             │  │             │
      │ entities    │  │ scheduled   │  │ dynamics    │
      │ resources   │  │ external    │  │ rules       │
      │ components  │  │ internal    │  │ solvers     │
      └─────────────┘  └─────────────┘  └─────────────┘
             │                 │                 │
             └─────────────────┼─────────────────┘
                               │
                    ┌──────────▼───────────┐
                    │     Infrastructure   │
                    │                      │
                    │ RNG                  │
                    │ serialization        │
                    │ telemetry            │
                    │ logging              │
                    │ checkpointing        │
                    │ parallel execution   │
                    └──────────────────────┘

// SECTION

# Architectural Decision Records

Significant architectural decisions should eventually be recorded.

Potential location:

```text
docs/
    architecture/
    decisions/
```

An architectural decision record should ideally explain:

```text
Decision:
Why:
Alternatives considered:
Tradeoffs:
Consequences:
Date:
Status:
```

The goal is to preserve reasoning, not merely conclusions.