Location: gpse/README.md
Purpose: Project-wide introduction
# GPSE
Index: Identity, Mission, Vision, Principles, Strategy

## IDENTITY

Author: Andre Carter
Repository: https://github.com/Andre-Carter/gpse
Description: General Purpuse Simulation Engine

## MISSION

***Prime Directive*** Above all: 
> **Build a simulation engine we can trust.**
Not merely an engine that runs, benchmarks well, has impressive abstractions.
An engine whose behavior is understandable, reproducible, testable, observable, extensible, and honest about its limitations.
The architecture may change. The scope may change. The implementation may change. The principles should change only when we have a good reason.
**GPSE is allowed to become something better than the original idea.**

## VISION

A long-term Rust project intended to develop a flexible, reliable, deterministic, and extensible simulation engine. The exact final scope of GPSE is intentionally **not fixed**. Rather than prematurely defining GPSE around one simulation domain, the project should evolve toward a general simulation kernel capable of supporting multiple simulation paradigms when justified by actual requirements.

## OJECTIVES

1. Archive, library, data system
2. Commandline interface, graphical user interface, websites
2. Applications, programs, tools, scripts
3. Simulation Enviroment, game engines, games, gaming

## PRINCIPLES

* Documentation
* Standardization 
* Acessability
* Test
* Modularity

UNDERSTAND THE FUNDAMENTALS.
BUILD WITH INTENTION.
PRESERVE THE KNOWLEDGE.
PROTECT THE FUTURE.
GROW WITHOUT LOSING THE VISION.

Organize my category, mirror, independent modules can be exported. 

## STRATEGY








How is the source tree organized?
# Source Tree
# of
# gpse\

## src\
### archive\
### experimental\
### foundation\
### mathematical\
### metadata\
### programs\
### science\
### systems\
### lib.rs
### main.rs

## target\
## vault\
## XDoc\
## XGPSE\
## .gitignore
## Cargo.lock
## Cargo.toml
## gpse_war_panel.md
## README.md

## Documentation Organization and Navigation

### XDoc/

1. ai
2. culture
3. development
4. guides
5. manifesto
6. references
7. research
8. sources
9. standards
10. war_room
11. XGPSE - Source-mirrored documentation      
12. README.md **This document you are currently reading**

Where can you find the roadmap and research?

* Check (XDoc\development\overview) for the roadmap
* Check (XDoc\research) for research

---

# Engineering Priorities

GPSE prioritizes engineering properties approximately in this order:

1. **Correctness**
2. **Determinism**
3. **Safety and failure visibility**
4. **Observability**
5. **Extensibility**
6. **Maintainability**
7. **Portability**
8. **Performance**
9. **Convenience**

Performance is important, but optimization must not silently compromise correctness, determinism, safety, or maintainability.

Performance optimizations should be supported by measurements whenever practical.

> **Do not optimize what has not been measured.**

---

# Core Design Principles

## 1. Determinism

Deterministic simulation should be a first-class capability.

Given equivalent:

* simulation model
* initial state
* configuration
* random seed
* engine version
* numerical configuration
* execution parameters

the simulation should produce reproducible results to the degree explicitly promised by the engine.

Where exact bitwise determinism is impossible or undesirable because of floating-point behavior, the engine should define and document appropriate numerical tolerances.

---

## 2. Explicit Time Semantics

Time must never be an implicit assumption.

The engine should clearly define:

* simulation time
* wall-clock time
* timestep behavior
* fixed versus variable timestep behavior
* event ordering
* scheduling semantics
* integration semantics
* time precision
* whether time may ever move backward
* behavior when numerical or scheduling limits are exceeded

---

## 3. Controlled Randomness

Randomness must be controllable.

Simulation randomness should support reproducibility through explicit seeds or equivalent deterministic mechanisms.

Random state should not depend accidentally on unrelated execution order, thread scheduling, or unrelated application behavior.

---

## 4. Explicit State Authority

The engine must have a clearly defined source of truth for simulation state.

Avoid architectures where multiple subsystems can silently disagree about authoritative state.

State mutation should be understandable, traceable, and testable.

---

## 5. Failure Visibility

GPSE should prefer explicit failure over silently producing questionable simulation results.

Examples of conditions that may eventually require explicit reporting include:

* numerical instability
* invalid state
* constraint violation
* impossible physical values
* invalid event scheduling
* timestep failure
* solver divergence
* overflow
* underflow
* invalid configuration
* unsupported operations

A simulation engine should be able to communicate:

> "I cannot confidently continue this simulation."

rather than silently returning corrupted results.

---

# Numerical Integrity

Numerical simulation requires special care.

GPSE should eventually distinguish between:

* mathematical errors
* numerical instability
* approximation error
* invalid input
* invalid model state
* engine/programming errors

Numerical algorithms should document their assumptions and limitations.

Where appropriate, GPSE should monitor quantities such as:

* integration error
* constraint error
* energy drift
* rejected timesteps
* solver convergence
* numerical warnings

The engine should not imply a level of physical or mathematical accuracy that it cannot justify.

---

# Testing Philosophy

Tests should verify more than individual functions.

GPSE should eventually use several levels of validation.

### Unit Tests

Verify individual functions, algorithms, data structures, and invariants.

### Integration Tests

Verify interactions between engine subsystems.

### Determinism Tests

Run identical simulations repeatedly and verify reproducibility.

### Regression Tests

Preserve known correct behavior across refactors.

### Golden Simulations

Maintain known scenarios with expected outputs or characteristics.

### Property Tests

Verify general invariants rather than only predetermined examples.

Potential invariants include:

* time does not move backward
* entity identifiers remain valid
* invalid states are rejected
* required conservation properties are maintained within defined tolerances
* event ordering is valid
* simulation state remains internally consistent

### Stress Tests

Test behavior under large workloads, long runtimes, extreme values, and unusual configurations.

---

# Checkpointing and Replay

Checkpointing and deterministic replay are desirable long-term capabilities.

A future GPSE workflow may resemble:

Simulation -> Checkpoint @ T=100 -> Continue simulation -> Failure detected @ T=143 -> Restore checkpoint -> Replay -> Investigate

This capability would be valuable for debugging, regression testing, experimentation, and long-running simulations.

---

# Observability

Simulation behavior should be observable without requiring invasive debugging.

Potential future facilities include:

* structured logging
* simulation metrics
* event tracing
* state inspection
* numerical health reporting
* profiling
* deterministic replay
* checkpoint inspection
* diagnostic snapshots

Observability should be designed so that debugging does not fundamentally alter simulation behavior.

---

# Architecture Philosophy

The simulation model should remain as independent from the runtime as reasonably practical.

Conceptually, the architecture may eventually separate concerns such as:

```text
Application / Simulation Model
            |
            v
      GPSE Runtime
            |
    +-------+-------+
    |       |       |
    v       v       v
  State   Events  Systems
    |       |       |
    +-------+-------+
            |
            v
     Infrastructure
```

Possible infrastructure includes:

* random number generation
* serialization
* checkpointing
* telemetry
* logging
* profiling
* parallel execution
* storage
* compute backends

This is a design direction, not a rigid implementation requirement.

Avoid introducing abstractions merely because they are fashionable.

Do not adopt ECS, event sourcing, async execution, GPU computation, distributed simulation, or other architectural techniques unless actual requirements justify them.

---

# Abstraction Rule

GPSE should favor abstractions that make correct code easier to write and incorrect code harder to write.

Avoid abstraction for abstraction's sake.

Before introducing a trait, subsystem, generic layer, or architectural pattern, ask:

1. What problem does this solve?
2. What complexity does it introduce?
3. Can the problem be solved more simply?
4. Does the abstraction preserve determinism?
5. Does it make testing easier or harder?
6. Does it make future replacement easier or harder?
7. Is there evidence that the abstraction is needed?

Prefer simple, explicit designs until complexity has demonstrated that a stronger abstraction is necessary.

---

# Performance Philosophy

GPSE should be designed with performance in mind, but performance should follow understanding.

Preferred process:

```text
Correct implementation
        |
        v
Measure
        |
        v
Identify bottleneck
        |
        v
Optimize
        |
        v
Measure again
        |
        v
Verify correctness and determinism
```

Do not assume that a particular data structure, ECS architecture, parallelization strategy, SIMD implementation, or GPU backend is automatically superior.

Benchmark real workloads.

Document meaningful performance decisions.

---

# Safety and Longevity

GPSE is intended to be a long-lived engineering project.

Important design decisions should favor:

* predictable behavior
* explicit contracts
* stable interfaces
* strong testing
* understandable failure modes
* reproducibility
* documentation
* migration paths
* versioning
* backward compatibility where appropriate

Avoid designs that are difficult to understand six months later.

Future maintainers—including future versions of the original developers—should be able to understand **why** major decisions were made.

---

# Current Development Rule

At any point in development, maintain a clear distinction between:

### Implemented

Features that actually exist and are tested.

## Current Architecture

[Keep the latest agreed architectural overview here.]

# Continuity Protocol

If development is interrupted and resumed later, read this document before making major architectural decisions.

The next development session should establish:

1. Current repository state.
2. Current branch/commit.
3. Current architecture.
4. Current tests.
5. Current known failures.
6. Current active objective.
7. Decisions made since the last checkpoint.
8. Any assumptions that have changed.
9. Any experimental systems currently under evaluation.

Do not assume that an old design decision is still correct simply because it appears in historical documentation.

Likewise, do not discard an existing design without understanding its purpose.

The project should evolve through accumulated evidence.

---


FOOTER NOTES 

CURRENT INVESTIGATION
---------------------
Legacy source structure is being characterized before refactoring.

CONFIRMED
---------
- `gpse` is both a library and binary crate.
- `lib.rs` exposes `chemical` and `physical`.
- `physical::constants` contains a large canonical constant dataset.
- `chemical/elements.rs` contains an Element model and periodic-element data.
- `chemical/mod.rs` does not currently expose `elements`.
- `main.rs` independently declares `chemical` and `physical`.
- `main.rs` also directly consumes the library crate (`gpse::...`).
- The executable therefore appears to compile a duplicate local module
  tree alongside the library.

WARNING INVESTIGATION
---------------------
The duplicate module declarations are a strong candidate for at least
some of the large warning count observed during cargo check/clippy.

NOT YET DECIDED
---------------
- Whether `main.rs` should remove its local module declarations.
- Exact public API for chemical data.
- Whether `elements` should be directly exported from `chemical`.
- Final organization of physical constants.
- Constant metadata / symbol / alias representation.

NEXT INVESTIGATION
------------------
Characterize the actual compiler/clippy warnings and determine which
warnings originate from duplicate binary module compilation versus
intentional unused canonical data.

# 