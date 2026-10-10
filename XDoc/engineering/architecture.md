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
