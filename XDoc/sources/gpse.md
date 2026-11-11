## Source Indexing
Sources should be indexed - S001

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