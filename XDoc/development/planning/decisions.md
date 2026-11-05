What we should deliberately postpone

Automatic type conversion: I32(5) plus F64(2.5) needs a defined policy.

Generic arithmetic: establish the typed operations and their overflow behavior before routing every operation through a universal numeric object.

Dimensions: keep physical dimensions and units separate from primitive numeric storage.

Serialization and display: add these once we define the public interface and representation requirements.

## D-XXX — Defer the Proposed CONTROL System

**Status:** Deferred / Unresolved
**Priority:** Low
**Date:** 2026-10-10

### Context

CONTROL has been identified as a possible future major GPSE system. Its purpose, boundaries, and relationship with existing systems remain unclear.

### Decision

Preserve CONTROL as an exploratory architectural concept. Do not approve implementation or reserve development resources at this time.

Current GPSE mission priorities take precedence.

### Rationale

Prematurely defining CONTROL could introduce unnecessary complexity or duplicate responsibilities already handled by existing systems. Further development should be driven by demonstrated requirements.

### Follow-up

Maintain the concept in `architecture.md`. Revisit it when a concrete GPSE need or sufficient architectural research justifies further work.

**Related reference:** Check XDoc > architecture documentation > `architecture.md`.

## D-XXX — Proposed Personnel System and No-Man-Left-Behind Protocol

**Status:** Proposed / Deferred
**Priority:** Exploratory

### Context

GPSE may benefit from a reusable personnel system and a `no_man_left_behind_protocol` for personnel accountability, missing-person response, and coordinated assistance across missions and simulations.

### Proposed Direction

Investigate `systems/personnel/` as the potential home for reusable personnel-domain logic and protocol definitions.

Programs and missions may consume these capabilities without owning the underlying personnel rules.

Potential integrations include the ledger, communication, security, navigation, telemetry, and mission or simulation systems.

### Guiding Principles

* Maintain explicit, auditable personnel-status transitions.
* Distinguish confirmed facts from unknown or unverified information.
* Respect authorization, privacy, and access controls.
* Coordinate existing systems rather than duplicate their responsibilities.
* Keep implementation deferred until concrete requirements justify it.

### Follow-up

Research use cases, define the protocol's scope, and determine whether personnel warrants a dedicated system before approving implementation.

// section

debate: rename rpg_engine -> the_lost_manuscripts
current conclusion: leave the generic naming of rpg_engine as it perfectly describes the purpose of the code. Futute extraction of rpg_engine will include renaming, the father folder to the_lost_manuscripts and dependanices.

// section

Decision: retain rpg_engine

Current GPSE naming convention

Today — within GPSE
programs/rpg_engine/
systems/command/library/programs/rpg_engine/
Future — standalone repository
the_lost_manuscripts/
- Rename the project root and crate identity as appropriate.
- Update module declarations and imports.
- Resolve mathematical and other GPSE dependencies.
- Preserve the domain logic and reusable components wherever practical.

// section

2. A practical decision table
Question	Best starting point
Does it modify this player's state?	impl Player
Does it validate a change to this player's state?	Usually impl Player
Does it calculate a value from arguments alone?	Standalone function
Does it coordinate several systems or objects?	Standalone service or domain coordinator
Does it read terminal input or print menus?	CLI/presentation module
Does it define how an individual stat behaves?	Potentially impl Stat or the relevant stat type

// section

## D-XXX: Experiments Module and Dependency Rule

**Status:** Approved
**Date:** 2026-11-05

### Decision
`src/experiments/` is the home for new modules before integration.

Dependency rule:
- Domain libraries (`mathematical`, `science`, `simulation`, `foundation`, `archive`, `programs`) MUST NOT depend on `experiments`.
- `experiments` MAY depend on any GPSE module.
- The CLI dispatcher (`systems::command`) MAY expose an experiment, and the command MUST be labeled "experimental" in `help`.

### Graduation
An experiment graduates when it has tests, an XGPSE note, and one canonical home. Graduation is a move plus an import fix.

### Alternatives considered
`test`, `sandbox`, `xp`, `eps`, `furnace`. Rejected for naming collisions or lack of clarity.

### Revisit when
An experiment stays stable without graduating, or a domain library needs something in `experiments`.

// section

note: (D-XXX) is a placeholder for a decision record number. note: worth considering futher implementation of record numbers with this project.

example: 

D-001	Defer the proposed CONTROL system
D-002	Personnel system and no-man-left-behind protocol (proposed)
D-003	Mission authorization and personnel protocols (proposed)
D-004	Retain rpg_engine name
D-005	Experiments module and dependency rule

// section

experiments/ isn’t for code that has bugs, it’s for code that’s unproven in concept

Empty scaffolding (algebra, calculus, chess, crypto, and so on): moving an empty folder just relocates it. Either keep it as a planned placeholder or delete it.

thinking: ghost_gpse: for empty structures