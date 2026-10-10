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

