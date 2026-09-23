# GPSE Code Standardization

## Working Architectural Outline

### 1. Purpose

Establish consistent conventions for GPSE source code, canonical data,
simulation models, APIs, naming, units, testing, and module organization.

The purpose is not to maximize abstraction or impose conventions before
their usefulness is demonstrated.

GPSE follows the principle:

> Understand → Map → Characterize → Test → Design → Migrate → Measure → Optimize

Standards should therefore be treated as strong defaults that may be
revised when concrete requirements demonstrate a better approach.

---

# 2. Rust Naming Conventions

GPSE should follow idiomatic Rust naming unless there is a documented
reason not to.

| Construct    | Convention             | Example            |
| ------------ | ---------------------- | ------------------ |
| Struct       | `PascalCase`           | `CelestialBody`    |
| Enum         | `PascalCase`           | `BodyType`         |
| Trait        | `PascalCase`           | `PhysicalEntity`   |
| Function     | `snake_case`           | `calculate_orbit`  |
| Method       | `snake_case`           | `mass_kg()`        |
| Variable     | `snake_case`           | `earth_state`      |
| Parameter    | `snake_case`           | `initial_velocity` |
| Struct field | `snake_case`           | `mass_kg`          |
| Module       | `snake_case`           | `physical`         |
| `const`      | `SCREAMING_SNAKE_CASE` | `SPEED_OF_LIGHT`   |
| `static`     | `SCREAMING_SNAKE_CASE` | `SUN`              |
| Type alias   | `PascalCase`           | `Mass`             |

The distinction between type names, canonical named data, and ordinary
instances should remain intentional.

Example:

```rust
pub struct CelestialBody {
    pub name: &'static str,
    pub mass_kg: f64,
    pub radius_m: f64,
}

pub static SUN: CelestialBody = CelestialBody {
    name: "Sun",
    mass_kg: 1.989e30,
    radius_m: 696_340_000.0,
};
```

Interpretation:

* `CelestialBody` = type
* `SUN` = canonical GPSE data
* `"Sun"` = display/name metadata
* `sun` = ordinary variable if an instance is created or referenced locally

---

# 3. Canonical Data Naming

Canonical scientific objects should use uppercase `SCREAMING_SNAKE_CASE`
when exposed as named constants/statics.

Examples:

```rust
pub static SUN: CelestialBody = ...;
pub static EARTH: CelestialBody = ...;
pub static MOON: CelestialBody = ...;

pub static HYDROGEN: Element = ...;
pub static URANIUM: Element = ...;

pub static SPEED_OF_LIGHT_IN_VACUUM: Constant = ...;
```

This distinguishes canonical definitions from runtime state.

The human-readable `name` field should retain the conventional scientific
or natural-language name:

```rust
name: "Sun"
```

not:

```rust
name: "SUN"
```

unless the source domain itself defines the name that way.

---

# 4. Definition vs State

GPSE should distinguish canonical entity definitions from mutable
simulation state whenever requirements demonstrate that the distinction
is useful.

Potential future model:

```text
CelestialBodyDefinition
    canonical properties
    mass
    radius
    composition
    identifiers
    metadata

CelestialBodyState
    position
    velocity
    rotation
    simulation-specific state
```

This distinction should NOT be implemented prematurely.

The initial Sun/Earth/Moon experiment should be used to determine whether
this separation produces a useful architectural boundary.

---

# 5. Units

Units should be explicit wherever ambiguity is reasonably possible.

Preferred field naming:

```rust
mass_kg
radius_m
distance_m
velocity_m_per_s
time_s
```

Avoid ambiguous fields such as:

```rust
mass
radius
distance
speed
```

unless the type itself guarantees the unit.

Future typed-unit abstractions should be considered only after concrete
requirements justify them.

---

# 6. Numeric Literals

Scientific numeric literals should prioritize readability and correctness.

Use:

```rust
1.989e30
```

for scientific notation.

Use digit separators for large exact values:

```rust
299_792_458.0
696_340_000.0
```

Do NOT use `^` for exponentiation in Rust.

```rust
1.989 * 10^30 // WRONG: ^ is bitwise XOR
```

---

# 7. Module Organization

The library should contain canonical GPSE functionality.

Preferred relationship:

```text
src/
├── lib.rs
├── main.rs
│
├── physical/
│   ├── mod.rs
│   ├── constants.rs
│   └── ...
│
├── chemical/
│   ├── mod.rs
│   ├── elements.rs
│   └── ...
│
├── entities/
│   ├── mod.rs
│   └── celestial.rs
│
└── simulation/
    ├── mod.rs
    └── ...
```

`main.rs` should generally consume the library rather than independently
declaring duplicate copies of library modules.

---

# 8. Public API

`pub` should mean something intentional.

Before exposing a type, constant, field, function, or module publicly,
ask:

1. Is this part of the intended GPSE API?
2. Does another module genuinely need it?
3. Is exposing the implementation detail useful?
4. Can the API remain smaller without losing capability?

Prefer a small, understandable public surface over premature exposure.

---

# 9. Canonical Scientific Data

Canonical data should be:

* deterministic
* immutable where appropriate
* independently testable
* clearly named
* documented with provenance where practical
* represented using explicit units
* separated from simulation state

Canonical data should not be duplicated across modules.

A single authoritative definition should be preferred.

---

# 10. Testing Standard

Every architectural component should have tests appropriate to its role.

Potential categories:

```text
Unit tests
Integration tests
Determinism tests
Regression tests
Property tests
Golden simulations
Stress tests
```

Small canonical data modules should have focused validation.

Example:

```rust
#[test]
fn speed_of_light_is_deterministic() {
    assert_eq!(
        SPEED_OF_LIGHT_IN_VACUUM.value,
        299_792_458.0
    );
}
```

Tests should verify meaningful guarantees rather than merely increase
test counts.

---

# 11. Determinism

Determinism is a first-class GPSE requirement.

Equivalent:

* model
* initial state
* configuration
* random seed
* engine version
* numerical configuration
* execution parameters

should produce reproducible results to the degree promised by the
relevant subsystem.

Where bitwise determinism is inappropriate, acceptable tolerances and
sources of variation should be documented.

---

# 12. Comments and Documentation

Comments should explain:

* why something exists
* why a non-obvious implementation was selected
* assumptions
* units
* provenance
* limitations
* known deviations from ideal behavior

Avoid comments that merely restate obvious syntax.

Prefer:

```rust
// Radius stored in meters to maintain SI consistency with the simulation
// layer.
```

over:

```rust
// Set radius_m to a number.
```

---

# 13. Error and Failure Philosophy

GPSE should favor explicit failure over silently questionable results.

The architecture should distinguish, where applicable:

* invalid input
* invalid simulation state
* mathematical error
* numerical instability
* approximation
* solver failure
* programming error

Failure visibility is preferred over silently producing potentially
misleading simulation results.

---

# 14. Architecture Decision Rule

No abstraction should be introduced solely because it is common in other
simulation engines.

Examples include:

* ECS
* event buses
* elaborate unit systems
* registries
* plugin systems
* serialization frameworks
* parallel execution layers
* inheritance-like hierarchies
* large generic trait systems

Each should earn its place through demonstrated requirements.

---

# 15. Experimental Architecture

Small experiments should be preferred before large migrations.

Example:

```text
Sun
Earth
Moon
```

can be used to test the entity architecture before introducing hundreds
or thousands of entities.

Experiments should answer specific questions and remain reversible.

An experimental design is not automatically a permanent GPSE architecture.

---

# 16. Formatting and Tooling

Rust tooling should be treated as part of the standard workflow.

Baseline tools:

```text
cargo fmt
cargo check
cargo test
cargo clippy
```

Formatting should remain rustfmt-compatible.

Warnings should be investigated rather than globally suppressed.

Automatic source modification tools such as `cargo fix` should be used
deliberately rather than blindly across legacy code.

---

# 17. Repository Hygiene

Generated build artifacts should not be committed.

Examples:

```text
target/
```

Git history should contain coherent checkpoints.

Preferred workflow:

```text
edit
→ inspect
→ format
→ check
→ test
→ review diff
→ commit
→ push
```

Commits should represent meaningful, recoverable states.

---

# 18. Current Status of This Standard

This document is a working standard, not a final specification.

Conventions should be revisited when actual GPSE development reveals:

* better abstractions
* new requirements
* scalability problems
* usability problems
* determinism concerns
* performance requirements
* API design issues

The project should prefer demonstrated evidence over architectural
assumption.

> Build only what we understand well enough to trust.
