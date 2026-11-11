// SECTION 1

Dimensional Analysis: A Lesson on M, L, T
1. The core idea

Every physical quantity is built from a few basic kinds of measurement. In mechanics there are three:

Symbol	Base dimension	SI unit
M	Mass	kilogram (kg)
L	Length	metre (m)
T	Time	second (s)

Every mechanical quantity can be written as a product of powers of these:

[Q] = M^a · L^b · T^c

The square brackets mean “the dimensions of”. The exponents a, b, c are usually integers or simple fractions. They tell you how the quantity depends on mass, length, and time.

Dimension is not the same as unit. Metres, feet, and miles are different units, but all have the dimension L. Dimension is the kind of quantity, and unit is the scale used to measure it.

2. What you need to know first

You only need a little math:

Exponent rules
x^a · x^b = x^(a+b)
x^a / x^b = x^(a−b)
(x^a)^b = x^(ab)
x^0 = 1
x^(−1) = 1/x
x^(1/2) = √x
Solving simple linear equations, for example 2c = −1 gives c = −1/2.
Basic physics definitions such as velocity = distance/time and force = mass × acceleration.
3. The rules of dimensional math

Rule 1: Multiplying adds exponents.

[A·B] = [A][B]

Example: area = length × length, so [A] = L·L = L².

Rule 2: Dividing subtracts exponents.

[A/B] = [A]/[B]

Example: velocity = length/time, so [v] = L/T = L T⁻¹.

Rule 3: Powers multiply exponents.

[A^n] = [A]^n

Example: v² has dimensions (L T⁻¹)² = L² T⁻².

Rule 4: You can only add or subtract quantities with the same dimensions.
You can’t add 5 metres to 3 seconds. If an equation has A + B = C, then [A] = [B] = [C]. This is the main principle behind dimension checking.

Rule 5: Pure numbers and some functions are dimensionless.

Numbers (2, π, ½, e) have dimension 1, written [1] or M⁰L⁰T⁰.
Angles (radians) are ratios of lengths, so they are dimensionless.
The argument of sin, cos, log, exp, etc. must be dimensionless. The result is dimensionless too.

Rule 6: Calculus operations.

[dy/dx] = [y]/[x]
[∫ y dx] = [y][x]

Example: acceleration a = dv/dt, so [a] = L T⁻¹ / T = L T⁻².

4. How to find the dimensions of any quantity

Method: Start from a definition or law you know, then substitute dimensions step by step.

Example: Force. Newton’s second law is F = ma.

[m] = M
[a] = L T⁻²
[F] = M · L T⁻² = M L T⁻²

Example: Energy. Kinetic energy is KE = ½mv². The ½ is dimensionless.

[KE] = M · (L T⁻¹)² = M L² T⁻²

Example: Pressure. Pressure = force/area.

[P] = M L T⁻² / L² = M L⁻¹ T⁻²
5. Reference table
Quantity	Formula	Dimensions	SI unit
Area	length²	L²	m²
Volume	length³	L³	m³
Velocity	distance/time	L T⁻¹	m/s
Acceleration	velocity/time	L T⁻²	m/s²
Density	mass/volume	M L⁻³	kg/m³
Momentum	mass × velocity	M L T⁻¹	kg·m/s
Force	mass × acceleration	M L T⁻²	N
Impulse	force × time	M L T⁻¹	N·s
Work / Energy	force × distance	M L² T⁻²	J
Power	energy/time	M L² T⁻³	W
Pressure	force/area	M L⁻¹ T⁻²	Pa
Torque	force × distance	M L² T⁻²	N·m
Frequency	1/time	T⁻¹	Hz
Angular velocity	angle/time	T⁻¹	rad/s
Spring constant	force/length	M T⁻²	N/m
Surface tension	force/length	M T⁻²	N/m
Dynamic viscosity	force·time/area	M L⁻¹ T⁻¹	Pa·s
Gravitational constant G	F r²/(m₁m₂)	M⁻¹ L³ T⁻²	N·m²/kg²

Note that torque and energy have the same dimensions even though they are different physical things. Dimensions can’t tell you everything.

6. Use 1: Checking an equation

If an equation is physically correct, both sides must have the same dimensions.

Example A: Is s = ut + ½at² dimensionally consistent?

[s] = L
[ut] = (L T⁻¹)(T) = L ✓
[½at²] = (L T⁻²)(T²) = L ✓

All terms are L, so the equation is consistent.

Example B: Is v = u + at² correct?

[v] = L T⁻¹
[u] = L T⁻¹ ✓
[at²] = (L T⁻²)(T²) = L ✗

The terms don’t match, so the equation is wrong. (The correct form is v = u + at.)

Caution: Passing the check doesn’t prove an equation is right. It can’t detect a wrong dimensionless factor (½ vs 2), and it can’t catch a wrong sign.

7. Use 2: Deriving a formula (the method of exponents)

Problem: A simple pendulum has length l, bob mass m, and gravity g. Find how its period τ depends on them.

Step 1: Guess that the answer is a product of powers.

τ = k · m^a · l^b · g^c

where k is a dimensionless constant.

Step 2: Write the dimensions of each part.

[τ] = T
[m] = M
[l] = L
[g] = L T⁻²

Step 3: Substitute.

T = M^a · L^b · (L T⁻²)^c
T = M^a · L^(b+c) · T^(−2c)

Step 4: Match exponents on both sides.

M: a = 0
L: b + c = 0
T: −2c = 1, so c = −1/2

Then b = +1/2.

Step 5: Write the result.

τ = k · √(l/g)

Dimensional analysis gave us the form of the period formula without any physics. The constant k (which turns out to be 2π) has to come from theory or experiment. Mass dropped out because a = 0.

8. Use 3: Unit conversions

If you know the dimensions, you know how a conversion factor behaves.

Example: Convert 1 N to g·cm/s² (dynes).

N = kg·m/s² has dimensions M L T⁻²
1 kg = 1000 g and 1 m = 100 cm
1 N = 1000 g × 100 cm / s² = 10⁵ g·cm/s²

Each base unit converts separately, and the exponents tell you how many times each factor applies.

9. Limits of the method
It cannot find dimensionless constants (½, 2π, etc.).
It fails if you leave out a relevant variable or include an irrelevant one.
It can’t handle sums like sin θ + cos θ; it only works for products of powers.
If more than three unknown exponents appear with only three base dimensions (M, L, T), the system is underdetermined. You then need extra information or the Buckingham Pi theorem.
Other dimensions exist for non-mechanical physics: temperature Θ, electric current I, amount of substance N, luminous intensity J.
10. Quick procedure to follow
List the quantities involved.
Write each one’s dimension as M^a L^b T^c.
Apply the rules: add exponents when multiplying, subtract when dividing, multiply when raising to a power.
Compare both sides of an equation, or solve for exponents, matching M, L, and T separately.
11. Practice problems
Find the dimensions of power from P = W/t.
Show that work = force × distance and kinetic energy = ½mv² have the same dimensions.
Check whether v² = u² + 2as is dimensionally consistent.
The speed v of a wave on a string depends on tension F (a force) and mass per unit length μ. Find v = k F^a μ^b.
Find the dimensions of the constant G in F = G m₁m₂/r².

Answers

P = M L² T⁻² / T = M L² T⁻³
Work: (M L T⁻²)(L) = M L² T⁻². KE: M (L T⁻¹)² = M L² T⁻². They match ✓
[v²] = L² T⁻². [u²] = L² T⁻². [as] = (L T⁻²)(L) = L² T⁻². All match ✓
[μ] = M L⁻¹. L T⁻¹ = (M L T⁻²)^a (M L⁻¹)^b. M: a + b = 0. L: a − b = 1. T: −2a = −1. This gives a = 1/2 and b = −1/2, so v = k √(F/μ).
G = F r² / (m₁m₂) = M L T⁻² · L² / M² = M⁻¹ L³ T⁻²

// SECTION 2

Dimensional Analysis: A Lesson on M, L, T
A study guide for using mass, length, and time dimensions in physics math.
1. The core idea
Every physical quantity is built from a few basic kinds of measurement. In mechanics there are three:
Symbol
Base dimension
SI unit
M
Mass
kilogram (kg)
L
Length
metre (m)
T
Time
second (s)
Every mechanical quantity can be written as a product of powers of these:
[Q] = M^a · L^b · T^c
The square brackets mean "the dimensions of". The exponents a, b, c are usually integers or simple fractions. They tell you how the quantity depends on mass, length, and time.
Dimension is not the same as unit. Metres, feet, and miles are different units, but all have the dimension L. Dimension is the kind of quantity; a unit is the scale used to measure it.
2. What you need to know first
1. Exponent rules
    ◦ x^a · x^b = x^(a+b)
    ◦ x^a / x^b = x^(a−b)
    ◦ (x^a)^b = x^(ab)
    ◦ x^0 = 1
    ◦ x^(−1) = 1/x
    ◦ x^(1/2) = √x
2. Solving simple linear equations, for example 2c = −1 gives c = −1/2.
3. Basic physics definitions such as velocity = distance/time and force = mass × acceleration.
3. The rules of dimensional math
Rule 1: Multiplying adds exponents. [A·B] = [A][B]. Example: area = length × length, so [A] = L·L = L².
Rule 2: Dividing subtracts exponents. [A/B] = [A]/[B]. Example: velocity = length/time, so [v] = L/T = L T⁻¹.
Rule 3: Powers multiply exponents. [Aⁿ] = [A]ⁿ. Example: v² has dimensions (L T⁻¹)² = L² T⁻².
Rule 4: You can only add or subtract quantities with the same dimensions. You can't add 5 metres to 3 seconds. If an equation has A + B = C, then [A] = [B] = [C]. This is the main principle behind dimension checking.
Rule 5: Pure numbers and some functions are dimensionless.
• Numbers (2, π, ½, e) have dimension 1, written M⁰L⁰T⁰.
• Angles in radians are ratios of lengths, so they are dimensionless.
• The argument of sin, cos, log, exp, etc. must be dimensionless, and the result is dimensionless too.
Rule 6: Calculus operations. [dy/dx] = [y]/[x] and [∫ y dx] = [y][x]. Example: acceleration a = dv/dt, so [a] = L T⁻¹ / T = L T⁻².
4. How to find the dimensions of any quantity
Start from a definition or law you know, then substitute dimensions step by step.
Force. Newton's second law is F = ma.
• [m] = M
• [a] = L T⁻²
• [F] = M · L T⁻² = M L T⁻²
Energy. Kinetic energy is KE = ½mv². The ½ is dimensionless.
• [KE] = M · (L T⁻¹)² = M L² T⁻²
Pressure. Pressure = force / area.
• [P] = M L T⁻² / L² = M L⁻¹ T⁻²
5. Reference table
Quantity
Formula
Dimensions
SI unit
Area
length²
L²
m²
Volume
length³
L³
m³
Velocity
distance/time
L T⁻¹
m/s
Acceleration
velocity/time
L T⁻²
m/s²
Density
mass/volume
M L⁻³
kg/m³
Momentum
mass × velocity
M L T⁻¹
kg·m/s
Force
mass × acceleration
M L T⁻²
N
Impulse
force × time
M L T⁻¹
N·s
Work / Energy
force × distance
M L² T⁻²
J
Power
energy/time
M L² T⁻³
W
Pressure
force/area
M L⁻¹ T⁻²
Pa
Torque
force × distance
M L² T⁻²
N·m
Frequency
1/time
T⁻¹
Hz
Angular velocity
angle/time
T⁻¹
rad/s
Spring constant
force/length
M T⁻²
N/m
Surface tension
force/length
M T⁻²
N/m
Dynamic viscosity
force·time/area
M L⁻¹ T⁻¹
Pa·s
Gravitational constant G
F r²/(m₁m₂)
M⁻¹ L³ T⁻²
N·m²/kg²
Note that torque and energy have the same dimensions even though they are different physical things. Dimensions can't tell you everything.
6. Use 1: Checking an equation
If an equation is physically correct, both sides must have the same dimensions.
Example A: Is s = ut + ½at² dimensionally consistent?
• [s] = L
• [ut] = (L T⁻¹)(T) = L ✓
• [½at²] = (L T⁻²)(T²) = L ✓
All terms are L, so the equation is consistent.
Example B: Is v = u + at² correct?
• [v] = L T⁻¹
• [u] = L T⁻¹ ✓
• [at²] = (L T⁻²)(T²) = L ✗
The terms don't match, so the equation is wrong. (The correct form is v = u + at.)
Caution: Passing the check doesn't prove an equation is right. It can't detect a wrong dimensionless factor (½ vs 2), and it can't catch a wrong sign.
7. Use 2: Deriving a formula (the method of exponents)
Problem: A simple pendulum has length l, bob mass m, and gravity g. Find how its period τ depends on them.
Step 1: Assume the answer is a product of powers.
τ = k · m^a · l^b · g^c, where k is a dimensionless constant.
Step 2: Write the dimensions of each part.
• [τ] = T
• [m] = M
• [l] = L
• [g] = L T⁻²
Step 3: Substitute.
T = M^a · L^b · (L T⁻²)^c = M^a · L^(b+c) · T^(−2c)
Step 4: Match exponents on both sides.
• M: a = 0
• L: b + c = 0
• T: −2c = 1, so c = −1/2, and then b = +1/2
Step 5: Write the result.
τ = k · √(l/g)
Dimensional analysis gave us the form of the period formula without any detailed physics. The constant k (which turns out to be 2π) has to come from theory or experiment. Mass dropped out because a = 0.
8. Use 3: Unit conversions
If you know the dimensions, you know how a conversion factor behaves.
Example: Convert 1 N to g·cm/s² (dynes).
• N = kg·m/s² has dimensions M L T⁻²
• 1 kg = 1000 g and 1 m = 100 cm
• 1 N = 1000 g × 100 cm / s² = 10⁵ g·cm/s²
Each base unit converts separately, and the exponents tell you how many times each factor applies.
9. Limits of the method
• It cannot find dimensionless constants (½, 2π, etc.).
• It fails if you leave out a relevant variable or include an irrelevant one.
• It can't handle sums like sin θ + cos θ; it only works for products of powers.
• If more than three unknown exponents appear with only three base dimensions (M, L, T), the system is underdetermined. You then need extra information or the Buckingham Pi theorem.
• Other dimensions exist for non-mechanical physics: temperature Θ, electric current I, amount of substance N, luminous intensity J.
10. Quick procedure
1. List the quantities involved.
2. Write each one's dimension as M^a L^b T^c.
3. Apply the rules: add exponents when multiplying, subtract when dividing, multiply when raising to a power.
4. Compare both sides of an equation, or solve for exponents, matching M, L, and T separately.
11. Practice problems
1. Find the dimensions of power from P = W/t.
2. Show that work = force × distance and kinetic energy = ½mv² have the same dimensions.
3. Check whether v² = u² + 2as is dimensionally consistent.
4. The speed v of a wave on a string depends on tension F (a force) and mass per unit length μ. Find v = k F^a μ^b.
5. Find the dimensions of the constant G in F = G m₁m₂/r².
Answers
1. P = M L² T⁻² / T = M L² T⁻³
2. Work: (M L T⁻²)(L) = M L² T⁻². KE: M (L T⁻¹)² = M L² T⁻². They match ✓
3. [v²] = L² T⁻². [u²] = L² T⁻². [as] = (L T⁻²)(L) = L² T⁻². All match ✓
4. [μ] = M L⁻¹. L T⁻¹ = (M L T⁻²)^a (M L⁻¹)^b. M: a + b = 0. L: a − b = 1. T: −2a = −1. So a = 1/2 and b = −1/2, giving v = k √(F/μ).
5. G = F r² / (m₁m₂) = M L T⁻² · L² / M² = M⁻¹ L³ T⁻²
12. Research prompts to save for later
Copy any of these into a new conversation when you're ready to go deeper.

Prompt 1: 

The Buckingham Pi theorem

Teach me the Buckingham Pi theorem from scratch. Start with why dimensional analysis alone gets stuck when there are more variables than base dimensions. Then show the formal statement of the theorem, how to count the number of dimensionless groups, and the step-by-step procedure for choosing repeating variables. Work through at least three examples: drag on a sphere (Reynolds number), pipe flow pressure drop, and a pendulum with large amplitude. For each, explain what the dimensionless groups mean physically and how engineers use them for scale-model testing. End with five practice problems with full solutions.

Prompt 2: 

Dimensionless numbers and scaling laws in physics and engineering

Give me a structured tour of the most important dimensionless numbers (Reynolds, Mach, Froude, Strouhal, Weber, Prandtl, Rayleigh, and others). For each, give the formula, the M-L-T derivation showing it is dimensionless, the physical ratio it represents, and a real-world example of where it matters. Then explain how scaling laws (such as why ants can carry many times their weight and elephants cannot, or why small models can predict large aircraft behavior) follow from dimensional reasoning. Include worked calculations and note where the arguments break down.

Prompt 3: 

Dimensional analysis beyond mechanics, plus how it is used in real research

Extend dimensional analysis beyond M, L, T. Cover temperature (Θ), electric current (I), amount of substance (N), and luminous intensity (J), and show how electromagnetism, thermodynamics, and chemistry quantities are expressed in the full SI dimensional system. Then explain natural units and Planck units, how physicists set c = ħ = G = 1, and how Fermi estimation and order-of-magnitude reasoning use dimensions. Include historical examples where dimensional reasoning produced real discoveries (such as G. I. Taylor's estimate of the Trinity explosion energy from photographs), and explain how to recognize when dimensional analysis will and will not work. Please flag any citations or historical details I should double check, since I need to verify them independently.