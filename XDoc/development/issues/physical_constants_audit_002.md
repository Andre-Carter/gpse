**Status:** Experimental
**Trust:** Audited
**Last reviewed:** 2026-11-09

# Physical Constants Audit 002

**Subject:** `constants.rs` after the audit 001 fixes (355 entries, `units` field)
**Source of truth:** `allascii.txt`, NIST "Fundamental Physical Constants, Complete Listing" (2022 CODATA). Same file as audit 001.
**Compared against:** audit 001 (`physical_constants_audit.md`, section 1 to 9) and the audit log at its end.
**Focus of this audit:** abbreviations, units in names, identifiers, and every hyphen.
**Rule:** report only. Nothing in `constants.rs` was changed.

**Method.** Both files were parsed by script. Because both now hold 355 entries in the same order, entries were compared by position. Position is confirmed correct, since value and units agree at every position except the one value error below. Numbers are compared as exact decimals. Every name was compared to NIST token by token. Every identifier was checked against the identifier its own name implies, using the rule *uppercase, every run of non-alphanumeric characters becomes one underscore, `/` is written `PER`*. That rule reproduces 345 of 355 identifiers unchanged, which is itself a finding: identifiers are a function of names, so fixing a name and its identifier together is one edit, and a test can enforce it.

## 1. How you did

Short version: **every numeric field is now correct except one digit run, and the structural work (count, order, numbering, wrong labels) is done.** The remaining work is almost entirely the abbreviation policy you named.

| Audit 001 finding | 001 | 002 | Result |
|---|---|---|---|
| Entries vs NIST (355) | 353 | 355 | both missing entries added |
| Order and `//N` numbering | ok | ok, 1 to 355 | no gaps |
| Duplicate identifiers | 0 | 0 | |
| Duplicate `name` strings | 1 | 0 | fixed |
| Value errors | 23 | **1** | 22 fixed |
| Uncertainty errors | 12 | **0** | all fixed |
| Unit errors | 4 | **0** | all fixed |
| Units string vocabulary | n/a | 76 of 76 strings identical to NIST | `units` is a verbatim copy |
| Wrong labels and wrong identifiers | 16 | **1** | 15 fixed, #291 remains |
| Identifier typos (RATION, TRITION, KLITIZING and others) | 8 | 0 | all fixed |
| Name typos listed in 001 | 21 | 0 | names now equal NIST apart from the items in section 3 |
| Regressions (anything that matched NIST before and does not now) | n/a | **0** for values, uncertainties, units | |

Every fix I could verify holds. Two things your log says that the data does not support:
1. **"All value fields should be corrected now" is one short.** `ELECTRON_RELATIVE_ATOMIC_MASS` (#107) was fixed halfway. The leading slip (`5.486...` to `5.485...`) is corrected, but the trailing digits are still wrong. See section 2.
2. **Two `//...` markers disappeared** (#59 and #123), both on entries you edited. See section 6.

### Corrections to audit 001
- **Numbering.** Audit 001 used the old 353-entry numbering. Adding the two missing entries shifted later numbers up by one after #102 and by two after #212. You were right that the 273.13 K entry is #200. Audit 001 said #199.
- **Misses in audit 001's name list.** Two name defects were in the file then and were not listed: #199 is missing the word `of`, and #302 reads `100kPa` without a space. Both are in section 3.
- **#107** was counted as fixed by your log. It is not, fully (section 2).

## 2. Remaining numeric defect (1)

| # | Identifier | NIST value | Current value | Difference |
|---|---|---|---|---|
| 107 | `ELECTRON_RELATIVE_ATOMIC_MASS` | 5.485 799 090 441 e-4 | 5.485_799_090_411e-4 | last digits `441` vs `411` |

Everything else matches: **354 of 355 values, 355 of 355 uncertainties, 355 of 355 units** are exact against NIST, including exact-vs-uncertain status for all 81 exact constants.

## 3. Names compared with NIST

78 names differ from NIST. All differences are accounted for:

| Kind of difference | Entries | Status |
|---|---|---|
| Word shorthand expanded (`mag.`, `mom.`, `gyromag.`) | 63 | Intentional (your decision). Done. |
| Unit symbols spelled out in "in X" names | 17 | Intentional, in progress. See section 4. |
| (the two kinds above overlap in 5 entries) | | |
| **Neither**, an unintended difference | **3** | See below. |

Unintended differences:

| # | NIST | Current | Problem |
|---|---|---|---|
| 133 | hartree**-**inverse meter relationship | hartree inverse meter relationship | **hyphen lost**. The only one of 119. |
| 199 | molar volume **of** ideal gas (273.15 K, 100 kPa) | molar volume ideal gas (273.15 K, 100 kPa) | word `of` missing. Its identifier also lacks `OF`. |
| 302 | Sackur-Tetrode constant (1 K, 100 kPa) | Sackur-Tetrode constant (1 K, 100**kPa**) | space missing. |

**`mom.` check:** NIST uses `mom.` 49 times. All 49 correctly became `moment`. None became `momentum`, and NIST itself spells `momentum` in full in 3 names. The question in your log is closed.

## 4. Abbreviations and units in names

### 4.1 Three kinds of abbreviation (the decision you need to make)
NIST's names contain three different things that all look like abbreviations. They need different rules.

| Family | Examples | Occurrences in NIST names | Current state |
|---|---|---|---|
| **A. Word shorthand** | `mag.`, `mom.`, `gyromag.`, `rms` | 51, 49, 10 and 3 | `mag.`, `mom.`, `gyromag.` fully expanded (110 of 110). `rms` untouched (3). |
| **B. Unit symbols** | `MeV`, `eV`, `GeV`, `K`, `T`, `Hz`, `MHz`, `kPa`, `J`, `u`, `fm` | see 4.2 | partly expanded |
| **C. Symbols that name an object or quantity** | `c`, `h`, `hc`, `h-bar`, `d`, `p`, `t`, `HD`, `HT`, `Cs-133`, `Si (220)`, `W`, `Z`, `x unit`, and `g factor` | 27 | untouched, same as NIST |

Family C must **not** be spelled out mechanically. `g` in `g factor` is the dimensionless g-factor, not grams. `T` is tesla in names but `t` is the triton in `shielding difference of t and p in HT`. Spelling these out would change meaning or add words NIST does not use.

### 4.2 Unit symbols in names: status by unit
The 50 names that carry a unit symbol are in this state: **14 fully spelled out, 3 partly, 27 unchanged, 6 are conditions in parentheses** (`(273.15 K, 101.325 kPa)`).

| Symbol | Spelled out as (suggested) | Names in NIST | Already spelled | Still a symbol | Entries still a symbol |
|---|---|---|---|---|---|
| MeV | mega electron volt | 13 | 4 | 9 | 98, 144, 212, 222, 226, 240, 247, 279, 295 |
| eV | electron volt | 8 | 3 | 5 | 131, 220, 256, 261, 294 |
| GeV | giga electron volt | 1 | 0 | 1 | 264 |
| K | kelvin | 10 | 2 | 8 | 49, 53, 191, 192, 199, 200, 302, 303 |
| T | tesla | 11 | 11 | 0 | none |
| Hz | hertz | 4 | 0 | 4 | 47, 53, 261, 299 |
| MHz | megahertz | 6 | 6 | 0 | none |
| kPa | kilopascal | 6 | 0 | 6 | 191, 192, 199, 200, 302, 303 |
| J | joule | 1 | 0 | 1 | 301 |
| u | unified atomic mass unit | 10 | 0 | 10 | 5, 78, 99, 145, 213, 241, 248, 280, 329, 343 |
| fm | femtometer | 1 | 0 | 1 | 295 |

`T` and `MHz` are done everywhere. Everything else is partial or untouched.

### 4.3 Inconsistencies the partial work created
- **Four spellings of one unit.** `electron volt` (15 names, all verbatim NIST, in the `...-electron volt relationship` names), `electronvolt` (7 names: #4, 13, 46, 52, 77, 300, 342), `mega electronvolt` (#4, #13) and `mega-electronvolt` (#77, #342). The last one also **adds a hyphen NIST does not have**. A search for `electron volt` misses seven entries, and a search for `electronvolt` misses fifteen.
- **Identifiers split the same way:** `ELECTRON_VOLT` in 16 identifiers and `ELECTRONVOLT` in 7.
- **A name and its identifier disagree about the unit** (section 5): #49, #54, #256.
- **One unit partly spelled in one name**: #47 `Bohr magneton in Hz per tesla` and #49 `Bohr magneton in K per tesla`. The `per tesla` half is spelled, the first half is not.

### 4.4 Why the unit belongs in the name (your point, confirmed)
Same quantity, different constants: Bohr magneton appears in `eV/T`, `Hz/T` and `K/T`, and the nuclear magneton in `eV/T`, `K/T` and `MHz/T`. The Boltzmann constant appears in `eV/K`, `Hz/K` and `inverse meter per kelvin`. The `units` field also distinguishes them, but only the name (and so the identifier) is a *key*. Removing units from names would produce duplicate identifiers. Likewise the temperature and pressure in `(273.15 K, 101.325 kPa)` are what separate #191 from #192, #199 from #200 and #302 from #303.

### 4.5 Policy options
**Option 1: spell out every Family A and B abbreviation in both name and identifier, one spelling table (recommended).**
- *For:* identifiers are uppercase-only, so unit symbols lose their case (`MHz` and `mHz` are both `MHZ`, and `T` and `t` are both `T`). Spelled-out words never collide. One rule is the easiest to test and to search. The cost is about 39 more name edits.
- *Against:* names drift from NIST's text, and are longer. Mitigated because `units` still holds the NIST symbol form, and NIST's own text can be regenerated by a parser if ever needed (your note).
- Family C stays as NIST writes it, with `rms` the only judgement call. It is shorthand for words (root mean square), so Family A applies.

**Option 2: keep unit symbols in names (NIST verbatim), expand only Family A.**
- *For:* names match NIST, and the `units` column already uses the same symbols. About 17 edits would be undone.
- *Against:* identifiers still need an uppercase form, so the case problem appears there anyway.

**Option 3: spell out in identifiers only, keep symbols in names.**
- *For:* fixes the case-collision problem at its source with a small change.
- *Against:* name and identifier then differ by design in about 50 entries, which breaks the "identifier is derived from name" property that currently holds for 345 of 355.

**Spelling question inside Option 1: `electron volt` or `electronvolt`?**
NIST writes two words in all 15 of its own `electron volt` names, so **two words** keeps those 15 untouched and gives the dataset one spelling (`mega electron volt`, no hyphen). The SI brochure writes one word, which is also fine, but then those 15 NIST names would have to change too. Pick one and apply it everywhere.

## 5. Identifiers

### 5.1 Name and identifier disagree (8 real, 2 convention)

| # | Identifier | Name | Disagreement |
|---|---|---|---|
| 49 | `BOHR_MAGNETON_IN_KELVIN_PER_TESLA` | Bohr magneton in K per tesla | identifier spells `KELVIN`, name has `K` |
| 54 | `BOLTZMANN_CONSTANT_IN_INVERSE_METER_PER_K` | Boltzmann constant in inverse meter per kelvin | identifier has `K`, name spells `kelvin` |
| 200 | `MOLAR_VOLUME_IDEAL_GAS_273_15_K_101_325_KPA` | molar volume of ideal gas (273.15 K, 101.325 kPa) | identifier lacks `OF`, name has it |
| 256 | `NUCLEAR_MAGNETON_IN_ELECTRON_VOLT_PER_TESLA` | nuclear magneton in eV per tesla | identifier spells `ELECTRON_VOLT`, name has `eV` |
| **291** | `REDUCED_ELECTRON_COMPTON_WAVELENGTH` | reduced **muon** Compton wavelength | **wrong identifier** (open since audit 001, section 6.4) |
| 302 | `SACKUR_TETRODE_1K_100KPA` | Sackur-Tetrode constant (1 K, 100kPa) | identifier drops `CONSTANT`, and fuses `1K` and `100KPA` |
| 303 | `SACKUR_TETRODE_1K_101P325KPA` | Sackur-Tetrode constant (1 K, 101.325 kPa) | drops `CONSTANT`, fuses tokens, and writes the decimal point as `P` |
| 317 | `SHIELDING_DIFFERENCE_TO_D_AND_P_IN_HD` | shielding difference of d and p in HD | identifier says `TO`, name says `of` |
| 53 | `BOLTZMANN_CONSTANT_IN_HZ_PER_K` | Boltzmann constant in Hz/K | convention only: `/` becomes `PER` |
| 261 | `PLANCK_CONSTANT_IN_EV_PER_HZ` | Planck constant in eV/Hz | convention only: `/` becomes `PER` |

Audit 001's other 15 identifier defects are confirmed fixed, including all four wrong labels (#11, #55, #58, #59) and `ALPHA_PARTICLE_PROTON_MASS_RATIO` and `ALPHA_PARTICLE_RELATIVE_ATOMIC_MASS` (#7, #8).

### 5.2 How each character reaches an identifier
NIST names use these non-letter characters. Counts are NIST names containing each one.

| Character | NIST names | What it means | In names now | In identifiers now | Recommendation |
|---|---|---|---|---|---|
| `-` | 119 | see section 7 | kept in 118 of 119, lost in #133, added to #77 and #342 | `_` | keep in names, `_` in identifiers (derived) |
| `.` | 68 | 63 are abbreviation dots (`mag.`, `mom.`, `gyromag.`), 5 are decimal points in conditions (#191, 192, 199, 200, 303) | abbreviation dots: 0 left. Decimals: 5 kept. | `_` in #191, 192, 199, 200, but **`P` in #303** | use `_` everywhere. Fix #303. |
| `/` | 15 | per (`eV/T`) | 12 spelled `per`, 3 left (#53, 226, 261) | `PER`, or `_C` for `MeV/c` | `per`, decided with section 4 |
| `(` `)` `,` | 7 and 6 | conditions and a Miller index | kept | dropped | drop in identifiers, but separate the tokens (see #302, #303) |
| digits | 19 entries | `-90`, `-133`, `-12`, `1st`, `2nd`, `(220)`, `273.15`, `101.325`, `times 2` | kept | kept | keep |
| upper and lower case | 2 clashes | `T` and `t`, `Hartree` and `hartree` | | both uppercased | spell `T` out (done). NIST's own `Hartree`/`hartree` clash is harmless. |

**Your question: how to represent `.` in an identifier.** The only decimals are the conditions of #191, 192, 199, 200 and 303. Three candidates:
1. **`_` (recommended).** Already used in 4 of 5, no new vocabulary. `273.15` becomes `273_15`. It cannot collide here, because no two conditions differ only by where the point is. A uniqueness test guards that permanently.
2. **`P` or `POINT`.** Unambiguous (`101P325`, `101_POINT_325`), but longer, and `P` is already used for pressure in this field.
3. **Remove the condition from the identifier and move it into fields** (temperature and pressure). That is a data-structure decision (section 8), not a naming one.

## 6. The `//...` markers
Your log says one was touched. Two disappeared compared with the previous file: **#59 `CONVENTIONAL_VALUE_OF_AMPERE_90`** and **#123 `FARADAY_CONSTANT`**, both entries you edited. Current state:

| | Count |
|---|---|
| Values NIST prints truncated with `...` (exact, but shown to 9 to 12 digits) | 62 |
| Marked `//...` | 44 correct |
| Marked `//..` (two dots) | 2 (#169, #171) |
| Truncated but **unmarked** | 16: 59, 123, 151, 153, 156, 165, 174, 176, 199, 200, 220, 295, 323, 351, 353, 354 |
| Marked but NIST gives a finite value | 1 (#161) |

The audit 001 note stands: the marker is an unverified human comment, not data. Whether to keep it is a model question (section 8). Nothing about it affects the numeric audit.

## 7. Hyphen checklist (all 119)
Every NIST name contains **at most one hyphen**, so 119 names carry 119 hyphens. If a hyphen were treated as a space, **no two NIST names would collide**, so the current `-` to `_` identifier mapping is safe. It does lose the difference between a pair separator and a word separator, which is why the name must keep the hyphen.

| Role | Count | Example |
|---|---|---|
| Pair separator in a unit-conversion name | 56 | `atomic mass unit-electron volt relationship` |
| Pair separator in a ratio or difference | 46 | `alpha particle-electron mass ratio` |
| Version suffix | 7 | `conventional value of ampere-90` |
| Compound proper name | 5 | `fine-structure constant`, `Stefan-Boltzmann constant`, `Sackur-Tetrode constant` (x2) |
| `h-bar` | 2 | `elementary charge over h-bar` |
| Isotope | 2 | `hyperfine transition frequency of Cs-133`, `molar mass of carbon-12` |
| Compound adjective | 1 | `standard-state pressure` |

Result: **118 of 119 kept in the current names. 1 lost (#133). 2 added that NIST does not have (#77, #342, in `mega-electronvolt`).** Mark each `[x]` once you have checked it against the identifier.

`[x]` = the hyphen is present in the current name. `[ ]` = it is not.

| Done | # | NIST name | Role | Current name hyphen |
|---|---|---|---|---|
| [x] | 1 | alpha particle-electron mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 7 | alpha particle-proton mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 14 | atomic mass unit-electron volt relationship | pair: unit-conversion relationship | kept |
| [x] | 15 | atomic mass unit-hartree relationship | pair: unit-conversion relationship | kept |
| [x] | 16 | atomic mass unit-hertz relationship | pair: unit-conversion relationship | kept |
| [x] | 17 | atomic mass unit-inverse meter relationship | pair: unit-conversion relationship | kept |
| [x] | 18 | atomic mass unit-joule relationship | pair: unit-conversion relationship | kept |
| [x] | 19 | atomic mass unit-kelvin relationship | pair: unit-conversion relationship | kept |
| [x] | 20 | atomic mass unit-kilogram relationship | pair: unit-conversion relationship | kept |
| [x] | 59 | conventional value of ampere-90 | version suffix (-90) | kept |
| [x] | 60 | conventional value of coulomb-90 | version suffix (-90) | kept |
| [x] | 61 | conventional value of farad-90 | version suffix (-90) | kept |
| [x] | 62 | conventional value of henry-90 | version suffix (-90) | kept |
| [x] | 64 | conventional value of ohm-90 | version suffix (-90) | kept |
| [x] | 65 | conventional value of volt-90 | version suffix (-90) | kept |
| [x] | 67 | conventional value of watt-90 | version suffix (-90) | kept |
| [x] | 69 | deuteron-electron mag. mom. ratio | pair: X-Y ratio/difference | kept |
| [x] | 70 | deuteron-electron mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 80 | deuteron-neutron mag. mom. ratio | pair: X-Y ratio/difference | kept |
| [x] | 81 | deuteron-proton mag. mom. ratio | pair: X-Y ratio/difference | kept |
| [x] | 82 | deuteron-proton mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 86 | electron-deuteron mag. mom. ratio | pair: X-Y ratio/difference | kept |
| [x] | 87 | electron-deuteron mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 91 | electron-helion mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 101 | electron-muon mag. mom. ratio | pair: X-Y ratio/difference | kept |
| [x] | 102 | electron-muon mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 103 | electron-neutron mag. mom. ratio | pair: X-Y ratio/difference | kept |
| [x] | 104 | electron-neutron mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 105 | electron-proton mag. mom. ratio | pair: X-Y ratio/difference | kept |
| [x] | 106 | electron-proton mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 108 | electron-tau mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 112 | electron-triton mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 114 | electron volt-atomic mass unit relationship | pair: unit-conversion relationship | kept |
| [x] | 115 | electron volt-hartree relationship | pair: unit-conversion relationship | kept |
| [x] | 116 | electron volt-hertz relationship | pair: unit-conversion relationship | kept |
| [x] | 117 | electron volt-inverse meter relationship | pair: unit-conversion relationship | kept |
| [x] | 118 | electron volt-joule relationship | pair: unit-conversion relationship | kept |
| [x] | 119 | electron volt-kelvin relationship | pair: unit-conversion relationship | kept |
| [x] | 120 | electron volt-kilogram relationship | pair: unit-conversion relationship | kept |
| [x] | 122 | elementary charge over h-bar | symbol (h-bar) | kept |
| [x] | 125 | fine-structure constant | compound proper name | kept |
| [x] | 128 | hartree-atomic mass unit relationship | pair: unit-conversion relationship | kept |
| [x] | 129 | hartree-electron volt relationship | pair: unit-conversion relationship | kept |
| [x] | 132 | hartree-hertz relationship | pair: unit-conversion relationship | kept |
| [ ] | 133 | hartree-inverse meter relationship | pair: unit-conversion relationship | LOST |
| [x] | 134 | hartree-joule relationship | pair: unit-conversion relationship | kept |
| [x] | 135 | hartree-kelvin relationship | pair: unit-conversion relationship | kept |
| [x] | 136 | hartree-kilogram relationship | pair: unit-conversion relationship | kept |
| [x] | 137 | helion-electron mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 147 | helion-proton mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 150 | hertz-atomic mass unit relationship | pair: unit-conversion relationship | kept |
| [x] | 151 | hertz-electron volt relationship | pair: unit-conversion relationship | kept |
| [x] | 152 | hertz-hartree relationship | pair: unit-conversion relationship | kept |
| [x] | 153 | hertz-inverse meter relationship | pair: unit-conversion relationship | kept |
| [x] | 154 | hertz-joule relationship | pair: unit-conversion relationship | kept |
| [x] | 155 | hertz-kelvin relationship | pair: unit-conversion relationship | kept |
| [x] | 156 | hertz-kilogram relationship | pair: unit-conversion relationship | kept |
| [x] | 157 | hyperfine transition frequency of Cs-133 | isotope (element-mass number) | kept |
| [x] | 158 | inverse fine-structure constant | compound proper name | kept |
| [x] | 159 | inverse meter-atomic mass unit relationship | pair: unit-conversion relationship | kept |
| [x] | 160 | inverse meter-electron volt relationship | pair: unit-conversion relationship | kept |
| [x] | 161 | inverse meter-hartree relationship | pair: unit-conversion relationship | kept |
| [x] | 162 | inverse meter-hertz relationship | pair: unit-conversion relationship | kept |
| [x] | 163 | inverse meter-joule relationship | pair: unit-conversion relationship | kept |
| [x] | 164 | inverse meter-kelvin relationship | pair: unit-conversion relationship | kept |
| [x] | 165 | inverse meter-kilogram relationship | pair: unit-conversion relationship | kept |
| [x] | 168 | joule-atomic mass unit relationship | pair: unit-conversion relationship | kept |
| [x] | 169 | joule-electron volt relationship | pair: unit-conversion relationship | kept |
| [x] | 170 | joule-hartree relationship | pair: unit-conversion relationship | kept |
| [x] | 171 | joule-hertz relationship | pair: unit-conversion relationship | kept |
| [x] | 172 | joule-inverse meter relationship | pair: unit-conversion relationship | kept |
| [x] | 173 | joule-kelvin relationship | pair: unit-conversion relationship | kept |
| [x] | 174 | joule-kilogram relationship | pair: unit-conversion relationship | kept |
| [x] | 175 | kelvin-atomic mass unit relationship | pair: unit-conversion relationship | kept |
| [x] | 176 | kelvin-electron volt relationship | pair: unit-conversion relationship | kept |
| [x] | 177 | kelvin-hartree relationship | pair: unit-conversion relationship | kept |
| [x] | 178 | kelvin-hertz relationship | pair: unit-conversion relationship | kept |
| [x] | 179 | kelvin-inverse meter relationship | pair: unit-conversion relationship | kept |
| [x] | 180 | kelvin-joule relationship | pair: unit-conversion relationship | kept |
| [x] | 181 | kelvin-kilogram relationship | pair: unit-conversion relationship | kept |
| [x] | 182 | kilogram-atomic mass unit relationship | pair: unit-conversion relationship | kept |
| [x] | 183 | kilogram-electron volt relationship | pair: unit-conversion relationship | kept |
| [x] | 184 | kilogram-hartree relationship | pair: unit-conversion relationship | kept |
| [x] | 185 | kilogram-hertz relationship | pair: unit-conversion relationship | kept |
| [x] | 186 | kilogram-inverse meter relationship | pair: unit-conversion relationship | kept |
| [x] | 187 | kilogram-joule relationship | pair: unit-conversion relationship | kept |
| [x] | 188 | kilogram-kelvin relationship | pair: unit-conversion relationship | kept |
| [x] | 197 | molar mass of carbon-12 | isotope (element-mass number) | kept |
| [x] | 204 | muon-electron mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 215 | muon-neutron mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 216 | muon-proton mag. mom. ratio | pair: X-Y ratio/difference | kept |
| [x] | 217 | muon-proton mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 218 | muon-tau mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 230 | neutron-electron mag. mom. ratio | pair: X-Y ratio/difference | kept |
| [x] | 231 | neutron-electron mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 243 | neutron-muon mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 244 | neutron-proton mag. mom. ratio | pair: X-Y ratio/difference | kept |
| [x] | 245 | neutron-proton mass difference | pair: X-Y ratio/difference | kept |
| [x] | 246 | neutron-proton mass difference energy equivalent | pair: X-Y ratio/difference | kept |
| [x] | 247 | neutron-proton mass difference energy equivalent in MeV | pair: X-Y ratio/difference | kept |
| [x] | 248 | neutron-proton mass difference in u | pair: X-Y ratio/difference | kept |
| [x] | 249 | neutron-proton mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 251 | neutron-tau mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 254 | Newtonian constant of gravitation over h-bar c | symbol (h-bar) | kept |
| [x] | 269 | proton-electron mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 282 | proton-muon mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 283 | proton-neutron mag. mom. ratio | pair: X-Y ratio/difference | kept |
| [x] | 284 | proton-neutron mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 287 | proton-tau mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 302 | Sackur-Tetrode constant (1 K, 100 kPa) | compound proper name | kept |
| [x] | 303 | Sackur-Tetrode constant (1 K, 101.325 kPa) | compound proper name | kept |
| [x] | 322 | standard-state pressure | OTHER | kept |
| [x] | 323 | Stefan-Boltzmann constant | compound proper name | kept |
| [x] | 325 | tau-electron mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 331 | tau-muon mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 332 | tau-neutron mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 333 | tau-proton mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 335 | triton-electron mass ratio | pair: X-Y ratio/difference | kept |
| [x] | 345 | triton-proton mass ratio | pair: X-Y ratio/difference | kept |

Hyphens added by the current names, not in NIST: **#77** and **#342** (`mega-electronvolt`).

## 8. What this audit implies for the data structure
These are observations, not a design. They come directly from the findings.

1. **A name carries several kinds of information at once.** For example `Bohr magneton in electron volt per tesla` is the *quantity* (Bohr magneton) plus the *unit it is expressed in*. `Loschmidt constant (273.15 K, 100 kPa)` is the quantity plus *conditions*. `X-Y mass ratio` is a *pair*. Today these are only recoverable by parsing text. If any of them become fields, an identifier can be derived from fields and not the reverse.
2. **The unit in a name and the `units` field say the same thing twice,** which is why they drift (#47, #49, #256). Deriving one from the other removes a class of defect.
3. **Identifier is a function of name** for 345 of 355 entries, so identifier can be generated.
4. **The conditions of 6 entries are real data** (temperature, pressure) and are currently only text.
5. **`None` in `uncertainty` means exact** and `None` in `units` means dimensionless. Your log already flagged that the two `None`s mean different things.
6. **Exactness and truncation** (section 6) are properties of the value, not of a comment.
7. **Stale label.** `Display` still prints `SI base units:` although the field is now `units`. A `symbol` field is also commented out.

## 9. Checklist to reach 100%
Numeric (blocks the refactor):
- [ ] #107 `ELECTRON_RELATIVE_ATOMIC_MASS`: `5.485_799_090_411e-4` to `5.485_799_090_441e-4`

Names and identifiers (small, independent):
- [ ] #133 restore the hyphen: `hartree-inverse meter relationship`
- [ ] #199 restore `of`: `molar volume of ideal gas (273.15 K, 100 kPa)`, and the identifier
- [ ] #200 identifier: add `OF`
- [ ] #302 name: `100 kPa` with a space
- [ ] #291 identifier: `REDUCED_MUON_COMPTON_WAVELENGTH`
- [ ] #317 identifier: `TO` to `OF`
- [ ] #302, #303 identifiers: restore `CONSTANT`, separate the tokens, and use `_` for the decimal point
- [ ] #77, #342: drop the added hyphen

Decide first, then apply (section 4.5):
- [ ] the abbreviation policy (Option 1, 2 or 3)
- [ ] `electron volt` or `electronvolt` (one spelling, everywhere)
- [ ] `rms` and `u`
- [ ] decimal point in identifiers

Tidy:
- [ ] the 16 unmarked, 2 malformed and 1 spurious `//...` markers, or decide to remove them
- [ ] `Display`: change `SI base units:` to `units`
- [ ] `ELECTRON_MUON_MASS_RATIO` is missing a trailing comma (valid Rust; `cargo fmt` will add it)

## 10. Reproducing this audit
Deterministic: parse both files, compare exact decimals by position, then compare names token by token. The checks that should become a Rust test once the data structure is chosen: value, uncertainty and units equal NIST for all 355 entries, the identifier equals the one derived from the name, every identifier is unique, and the character set of every name is the allowed set.