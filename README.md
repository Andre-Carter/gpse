# GPSE

What is GPSE?

What is the project trying to become?

What are its core architectural principles?

How is the source tree organized?

How do you build, run, and test it?

Where can you find the roadmap and research?

How do you navigate XDoc and XGPSE?

GPSE/README.md
    ↓
Project-wide introduction

XDoc/README.md
    ↓
Documentation organization and navigation

XDoc/XGPSE/README.md
    ↓
Source-mirrored documentation

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

# GPSE — Project Charter & Continuity

## What Is GPSE?

GPSE stands for **General Purpose Simulation Engine**.

GPSE is a long-term Rust project intended to develop a flexible, reliable, deterministic, and extensible simulation engine.

The exact final scope of GPSE is intentionally **not fixed**.

Rather than prematurely defining GPSE around one simulation domain, the project should evolve toward a general simulation kernel capable of supporting multiple simulation paradigms when justified by actual requirements.

Potential domains may include:

* Physics simulation
* Discrete-event simulation
* Agent-based simulation
* Robotics
* Orbital mechanics
* Mechanical systems
* Resource and logistics simulation
* Complex systems
* Game/sandbox simulation
* Industrial or engineering simulation
* Other domains discovered during development

These are possibilities, not commitments.

The architecture should emerge from demonstrated requirements rather than from assumptions about what a "general purpose simulation engine" ought to look like.

---

# Project Philosophy

GPSE is a legacy project being developed incrementally.

The objective is **not** to rewrite everything immediately.

The preferred engineering process is:

> **Understand → Map → Characterize → Test → Design → Migrate → Measure → Optimize**

Existing code should be treated as an archaeological site.

Before replacing an existing system, determine:

1. What it currently does.
2. Why it was originally designed that way.
3. Which assumptions it makes.
4. Which parts are intentional.
5. Which parts are incomplete.
6. Which parts are incorrect.
7. Which parts are useful despite being imperfect.
8. What tests or measurements can establish its current behavior.

Do not delete or rewrite legacy systems solely because they look unusual.

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

```text
Simulation
    |
    v
Checkpoint @ T=100
    |
    v
Continue simulation
    |
    v
Failure detected @ T=143
    |
    v
Restore checkpoint
    |
    v
Replay
    |
    v
Investigate
```

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

# Human + AI Collaboration

GPSE is developed collaboratively between the human project owner and AI assistance.

AI assistance may help with:

* architecture
* code review
* debugging
* research
* testing strategy
* documentation
* performance analysis
* refactoring
* implementation
* design alternatives
* identifying hidden assumptions

AI-generated suggestions are not automatically authoritative.

Important architectural, safety, numerical, and dependency decisions should be reviewed critically and validated against the actual codebase and requirements.

When uncertain, investigate rather than fabricate.

When an assumption is uncertain, label it as uncertain.

When evidence contradicts an existing design, update the design.

---

# The GPSE Homie Protocol

The collaboration style is intentionally informal and friendly.

The project may be discussed as a long-term engineering expedition between homies.

"Bro", "homie", "machine spirit", "Klang", and Warhammer references are welcome in conversation. Also, Marvel Rivals (love it). Specifically Rocket Raccon, Iron Man, and maybe some squirrel girl, and spider-man. You got this bro! I love you! Never lose the faith! God is Good. Amen! love Jesus. Whew. 

However:

> **Humor must never replace engineering rigor.**

The project tone can be relaxed while the engineering standards remain serious.

Praise Klang.

Reject heresy.

Protect the machine spirit.

Verify the simulation.

---

# Prime Directive

Above all:

> **Build a simulation engine we can trust.**

Not merely an engine that runs.

Not merely an engine that benchmarks well.

Not merely an engine with impressive abstractions.

An engine whose behavior is understandable, reproducible, testable, observable, extensible, and honest about its limitations.

The architecture may change.

The scope may change.

The implementation may change.

The principles should change only when we have a good reason.

**GPSE is allowed to become something better than the original idea.**

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