Roadmap: what we intend to build next. Sebas

# 1611 KJV CORPUS
# FORMAT / DISPLAY
# METADATA / CLI

# MATH CONSTANTS
# AUDIT PHYSICAL CONSTANTS
# PERIODIC TABLE - ELEMENT

# CALC
# FOUNDATION::INPUT::TEXT
# MATH LIBRARY FUNCTIONS
# MATH CLI

# NUMERIC
# DIMENSIONS
# CONVERSION

# MIDDLE GROUND EXPERIMENTS, DATA VISUALIZATION

# RUST-RUST
# AUDIT
# RESEARCH

# THE GREAT GPSE DOCUMENTATION REFACTOR. 😂
# WAR PANEL
# WAR ROOM

# SANDBOX
# SIMULATION
# MAP

# PIGEON - MESSAGER
# CAMEL  - DATA STORAGE
# SPIDER - API, SEARCH, WEB-SCRAPPING, 
# ANT    - CONVERSION, MACHINE-MACHINE, AUTOMATION
# AI

- kjv, amplified, nlt...

VERSE       → Genesis 1:1
VERSE RANGE → Genesis 1:1–5
CHAPTER     → Genesis 1
BOOK        → Genesis
BIBLE       → KJV 1611

The GPSE top-three roadmap
1. Mathematics
Foundation

Goal: Build reliable, reusable mathematical primitives.

Focus on:
Numeric data types and type limits.
Arithmetic operations and edge-case tests.
Dimensions, units, and dimensionless values.
Sequences, series, factorials, and summation.
Carter mathematics after the core contracts are stable.
Milestone: GPSE can represent numeric values, calculate with them safely according to defined policies, and verify results with tests.

2. Bible / READ

Goal: Build a dependable, searchable textual archive.
1. Preserve the working book, chapter, and verse lookup.
2. Strengthen parsing and input validation.
3. Support verse ranges and chapter/book navigation.
4. Add searching and structured textual metadata.
Milestone: READ reliably retrieves the requested passage and handles invalid references gracefully.

Keep the typed corpus and retrieval logic separate from CLI presentation.

// section

No new parser architecture until we've audited the parsing GPSE already performs.

We're going to refactor existing behavior, identify duplication, establish consistent interfaces, and only then decide what belongs in GPSE Core.
1. Initial parsing inventory
Based on our previous work, I see several existing parsing operations worth investigating:
Area	Existing operation	What we should examine
CLI	Command input	Trimming, normalization, command recognition
Calculator	Numeric input	f64 parsing, separators, validation
Calculator	Operator input	Symbols and word aliases
Data types	Numeric conversion	&str into primitive numeric types
Bible	Scripture references	Book, chapter, verse parsing
Carter functions	Numeric input	Input normalization and validation

We also have the RSVP program as inspiration, but let's inspect its existing implementation before drawing architectural conclusions.
2. Refactoring objectives
I propose this order:
1. Inventory — identify every existing parsing function.
2. Characterize — document exactly what each currently accepts and returns.
3. Consolidate — identify duplicated operations.
4. Standardize — establish consistent parsing and error-handling conventions.
5. Test — preserve existing behavior and catch regressions.
6. Integrate — determine what belongs in core/parsing.
One important principle: parsing should interpret input and produce structured data. It shouldn't become responsible for executing commands or performing mathematical operations.