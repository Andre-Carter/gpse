Policy
Conventions

(verified)
Word shorthand: 
mag. -> magnetic
mom. -> moment (Every mom. became moment, never momentum.)
gyromag. -> gyromagnetic

(unverified)
Unit symbol abbreviations:
Mev -> mega electron volt
eV -> electronvolt
K -> kelvin
Hz -> hertz
kPa -> kilopascal
U -> u (unified atomic mass unit)
T -> tesla
MHz -> megahertz

Symbols that name objects or quantities (c, h, d, p, t, g factor): spelling these out would change meaning. g is the g-factor, not grams, and T is tesla while t is the triton.

My recommendation is to spell out the first two families in both name and identifier, using one spelling table. Identifiers are uppercase-only, so MHz and mHz would collide. Keep symbols in units. Your point that units belong in names holds up. Bohr magneton appears in eV/T, Hz/T and K/T, and dropping the unit would create duplicate identifiers. The doc lays out three options with trade-offs.

My recommendation is to spell out the first two families in both name and identifier, using one spelling table. Identifiers are uppercase-only, so MHz and mHz would collide. Keep symbols in units. Your point that units belong in names holds up. Bohr magneton appears in eV/T, Hz/T and K/T, and dropping the unit would create duplicate identifiers. The doc lays out three options with trade-offs.

Hyphens. All 119 are in the checklist: each NIST name has at most one. 102 are pair separators, and the rest are version suffixes, isotopes, compound names and h-bar. 118 of 119 are kept in your names. Treating a hyphen as a space would collide no two names, so mapping - to _ in identifiers is safe. The name should keep the hyphen, because identifiers can’t tell a pair separator from a word break.

Decisions I need from you

Which abbreviation policy: option 1, 2 or 3 in section 4.5?
If you spell out units, should it be electron volt (two words, as NIST writes it in 15 of its own names) or electronvolt? Pick one and apply it everywhere.
rms → root mean square, and u → unified atomic mass unit?
Decimal points in identifiers, i.e. your open question: I’d use _ everywhere (273_15). That fixes #303, the only place using P. The alternative is moving temperature and pressure into fields.

Section 8 lists what the audit implies for the data structure: a name mixes quantity, unit and conditions, and the unit is stated twice, which is why #47, #49 and #256 drifted. Once you settle the policy, I’m glad to go through candidate structures with you.