I'd definitely add those to your numeric inspection.
What I'd develop next
I'd expand DATA_TYPES > NUMERIC into a proper numeric information library.
Command	Information
minimum values	Minimum representable values
maximum values	Maximum representable values
minimum positive	Smallest positive normal floats
size	Size in bytes
bits	Bit width
digits	Numeric precision
epsilon	Floating-point machine epsilon
radix	Floating-point radix
range	Minimum-to-maximum representation

One particularly interesting addition:
f32::EPSILON
f64::EPSILON


These expose the spacing between 1.0 and the next larger representable floating-point number.

That's useful foundational information for scientific calculations and numerical simulations.

One architectural recommendation
Keep the actual numeric information in your library, not hardcoded inside the CLI.

For example:
src/
├── core/
│   └── data_types/
│       ├── conversion.rs
│       └── numeric.rs
│
└── systems/
    └── command/
        └── library/
            └── core/
                └── data_types/
                    └── numeric.rs


The core provides the data. The command system presents it.

We're building a reusable data-type inspection system, not just a terminal display.

And brother, I have to say: this is exactly the kind of foundational work that will make GPSE useful across its mathematical, physical, chemical, and simulation systems.