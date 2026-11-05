# GPSE Development Procedures
Development procedures: how we design and implement a component.

**Document Type:** Engineering Workflow
**Status:** Proposed Standard
**Scope:** GPSE libraries, modules, functions, systems, and interfaces

## Purpose

Establish a consistent development procedure for designing, implementing, testing, and maintaining GPSE components.

The objective is to ensure that GPSE grows through deliberate engineering rather than accumulating disconnected functions, inconsistent conventions, and unnecessary complexity.

**Core Principle:** Evaluate before implementation. Define before expansion. Test before integration. Document as we go.

---

# 1. Evaluate Dimensions

Before implementing a component, identify its dimensions: the independent considerations that define what the component does, what it accepts, and how it behaves.

### 1.1. Domain and Location

* Which library, field, or system owns the component?
* What mathematical, scientific, or engineering domain does it belong to?
* Does an appropriate module already exist?
* Does the component belong in an existing library or justify a new one?

Example:

* Library: `mathematical::arithmetic`
* Operations: addition, subtraction, multiplication, division
* Related field: numerical computation

### 1.2. Data Types

* What types of data does the component accept?
* What types does it produce?
* Which numeric types are supported?
* Are signed, unsigned, integer, floating-point, or custom types required?
* Are conversions necessary, and are they safe?

Example:

* Numeric type: `i32`
* Input: two `i32` values
* Output: `i32`, or an error if the operation cannot be completed safely

### 1.3. Operations and Behavior

* Which operations, methods, and transformations are required?
* What are the inputs, outputs, and side effects?
* Does the component generate, store, transform, inspect, or present data?
* What are the expected results and edge cases?

### 1.4. Additional Dimensions

Evaluate any further dimensions relevant to the component, including:

* Units and physical dimensions
* Ownership, borrowing, and lifetimes
* Mutability and state
* Determinism and reproducibility
* Performance and memory usage
* Error behavior and resource limits
* Interface and integration requirements

Not every component requires every dimension. Evaluate what is relevant and record important decisions.

---

# 2. Naming Conventions

Names must communicate purpose, behavior, and domain meaning.

### 2.1. Readability

* Prefer understandable names over unnecessarily abbreviated names.
* Avoid naming choices that make ordinary development painful.
* Optimize for future maintainers, not merely the current implementation.
* Use consistent terminology across related modules.

### 2.2. Rust Conventions

Follow standard Rust naming conventions:

* `snake_case` for functions, methods, variables, and modules.
* `UpperCamelCase` for types, traits, structs, and enums.
* `SCREAMING_SNAKE_CASE` for constants and statics.

Examples:

* `sequence_linear_i32`
* `checked_add_i32`
* `SequenceError`
* `ArithmeticError`
* `MAX_SEQUENCE_LENGTH`

### 2.3. Domain Accuracy

* Use terminology appropriate to the relevant mathematical, scientific, or engineering field.
* Distinguish variables, coefficients, parameters, constants, arguments, and results correctly.
* Do not invent terminology where established terminology already serves the purpose.
* When GPSE introduces its own terminology, define it explicitly and distinguish it from established conventions.

### 2.4. Naming Review

Before finalizing a name, ask:

1. Does it describe what the component actually does?
2. Is the terminology technically accurate?
3. Is it consistent with related GPSE components?
4. Will it remain understandable as the system grows?

---

# 3. Safety and Interface Review

Before implementation is considered complete, identify how the component can fail and how its behavior should be communicated.

### 3.1. Error Handling

* Identify invalid inputs and unsupported operations.
* Consider arithmetic overflow, division by zero, invalid ranges, and conversion failures where relevant.
* Prefer explicit, documented error behavior over unexpected panics.
* Use appropriate error types and preserve useful error information.
* Define whether invalid inputs produce errors, empty results, or other documented outcomes.

### 3.2. Ownership and Resource Safety

* Determine whether values should be owned, borrowed, cloned, or consumed.
* Avoid unnecessary allocations and cloning.
* Consider memory usage and computational limits.
* Avoid exposing internal state unnecessarily.
* Ensure that errors do not leave callers with misleading or partially valid results.

### 3.3. CLI and Presentation

When a component interacts with the CLI, determine:

* How input is collected and validated.
* How commands are routed to the correct handler.
* How results are formatted and displayed.
* How errors and diagnostic information are presented.
* Whether output should be human-readable, machine-readable, or both.

**Architecture rule:** Domain functions should generally return data or errors. CLI handlers should handle user input and presentation.

### 3.4. Side Effects

Identify whether the component:

* Prints to the terminal.
* Reads input.
* Modifies shared state.
* Writes files or communicates with external systems.
* Depends on randomness, time, or other environmental conditions.

Keep side effects explicit and separate from pure computation where practical.

---

# 4. Think: Dependencies, Consumers, and Growth

Before finalizing a component, consider its place in the wider GPSE architecture.

### 4.1. Consumers

* Which current components will use it?
* Which future systems might reasonably need it?
* Can it be reused without copying its implementation?
* Does it expose an appropriate public interface?

### 4.2. Cross-References

* Does related functionality already exist elsewhere?
* Can this component reuse existing GPSE infrastructure?
* Should related documentation or modules link to it?
* Is another component duplicating the same responsibility?

Maintain one canonical implementation for each responsibility wherever practical.

### 4.3. Dependencies

* What does this component depend on?
* Are those dependencies necessary?
* Does the dependency direction preserve architectural boundaries?
* Can lower-level libraries remain independent of the CLI?
* Does a dependency introduce unnecessary complexity or restrictions?

Prefer clear dependency direction and minimal coupling.

### 4.4. Growth and Future Compatibility

* Can the component be extended without rewriting unrelated systems?
* Are numeric types, operations, or domain rules likely to expand?
* Are abstractions justified by current needs or demonstrated repetition?
* Would a generic interface help, or would it obscure simple behavior?
* Is the design flexible without attempting to anticipate every possible future requirement?

**GPSE principle:** Build for foreseeable growth, not hypothetical perfection.

---

# 5. Implement and Verify

A component is not complete merely because it compiles.

### 5.1. Implementation

* Follow the evaluated dimensions and naming decisions.
* Keep responsibilities focused.
* Prefer simple, explicit implementations before introducing complex abstractions.
* Reuse established interfaces and conventions.
* Document non-obvious decisions.

### 5.2. Testing

* Test expected behavior.
* Test boundary conditions and invalid inputs.
* Test error handling and overflow where applicable.
* Test important type-specific behavior.
* Add regression tests for discovered defects.
* Verify public behavior independently of the CLI whenever practical.

### 5.3. Verification

Run the applicable project checks, including:

* Formatting checks.
* Compilation checks.
* Unit and integration tests.
* Documentation checks where available.
* Manual CLI checks for interactive behavior.

Investigate failures rather than assuming they are unrelated.

### 5.4. Integration

* Confirm that callers can use the public interface correctly.
* Check for unintended changes to existing behavior.
* Verify module exports and dependency boundaries.
* Update relevant documentation and cross-references.

---

# 6. Document, Checkpoint, and Maintain

### 6.1. Documentation

Record the component's purpose, interface, supported behavior, important limitations, and relevant design decisions.

Update XDoc/XGPSE as the implementation evolves. Documentation may describe planned work, but it must distinguish planned, experimental, implemented, and retired functionality.

### 6.2. Version Control

* Review the changes before staging.
* Inspect the repository status and diff.
* Run appropriate verification checks.
* Commit a coherent unit of work with a meaningful message.
* Push the checkpoint when appropriate.

Prefer small, recoverable checkpoints over large, difficult-to-review changes.

### 6.3. Maintenance

When a component changes:

* Re-evaluate affected dimensions.
* Review its consumers and dependencies.
* Update tests and documentation.
* Check whether previous conventions still apply.
* Remove obsolete implementations and duplicated responsibilities safely.

---

# 7. Feedback Loop

Development is iterative, not strictly linear.

If implementation or testing reveals a missing requirement, return to the relevant stage and revise the design.

For example:

* A test exposes an overflow case → revisit safety and error handling.
* A second system needs the same operation → revisit reuse and dependencies.
* A name proves ambiguous → revisit naming conventions.
* A new numeric type changes behavior → revisit data types and testing.

Record important discoveries so future work benefits from them.

---

# Completion Checklist

Before considering a component ready for integration:

* [ ] Its domain and module ownership are clear.
* [ ] Its inputs, outputs, and data types are defined.
* [ ] Its naming follows Rust conventions and domain terminology.
* [ ] Its errors and edge cases have been considered.
* [ ] Its ownership, side effects, and resource behavior are understood.
* [ ] Its dependencies and consumers have been reviewed.
* [ ] Its expected behavior and important failure cases are tested.
* [ ] Formatting and applicable build checks pass.
* [ ] Documentation and cross-references are updated.
* [ ] Changes are reviewed and checkpointed.

## Engineering Principle

**GPSE must remain understandable as it grows.**

The purpose of this procedure is not to eliminate complexity or require unnecessary ceremony. It is to make complexity deliberate, visible, testable, and maintainable.

Evaluate the dimensions. Name the components accurately. Consider safety. Think about the wider system. Implement, verify, document, and improve.

One cog at a time.
