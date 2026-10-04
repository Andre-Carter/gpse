- THE GREAT GPSE DOCUMENTATION REFACTOR. 😂

VERSE       → Genesis 1:1
VERSE RANGE → Genesis 1:1–5
CHAPTER     → Genesis 1
BOOK        → Genesis
BIBLE       → KJV 1611

- sebas 

- kjv, amplified, nlt...

- insert (verse #) for verse ranges

CARTER
│
├── CONSTANTS
│   ├── CC
│   ├── CC_COMPL
│   ├── CC_RADIAN
│   ├── CC_FOUR
│   └── ...
│
├── GEOMETRY
│   ├── 2D
│   │   ├── square
│   │   ├── circle
│   │   ├── area gap
│   │   └── perimeter gap
│   │
│   └── 3D
│       ├── cube
│       ├── sphere
│       ├── volume ratio
│       └── surface ratio
│
├── RATIOS
│   ├── Carter Ratio
│   ├── Carter Constant
│   └── derived ratios
│
└── SEQUENCES
    ├── C(n) = n²/(2π)
    └── R(n) = n/(n+π²)

I'd make the report explicitly distinguish measurements from derived ratios:

CARTER GEOMETRY
================

INPUT
h: 10
diameter/side: 20

2-D
---
Corner Length:       14.142135623730951
Circle Circumference: 62.83185307179586
Square Perimeter:     80

Circle Area:          314.1592653589793
Square Area:          400

Gaps
----
Area Gap:             85.84073464102067
Perimeter Gap:        17.168146928204138

RATIOS
------
Circle/Square Area:       π/4
Circle/Square Perimeter:  π/4
Square/Circle Area:       4/π
Square/Circle Perimeter:  4/π

CARTER CONSTANT
---------------
CC:                     0.2146018366025517
1 - π/4:                0.2146018366025517
CC Error:              ~2.8e-17

3-D
---
Sphere Surface Area:    1256.6370614359173
Cube Surface Area:      2400

Sphere Volume:          4188.790204786391
Cube Volume:            8000

RATIOS
------
Sphere/Cube Volume:         π/6
Sphere/Cube Surface Area:   π/6
Cube/Sphere Volume:         6/π
Cube/Sphere Surface Area:   6/π