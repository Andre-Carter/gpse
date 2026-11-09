**Status:** Experimental
**Trust:** Audited
**Last reviewed:** 2026-11-08

# Physical Constants Audit

**Subject:** `src/science/physical/constants.rs`
**Source of truth:** `allascii.txt`, "Fundamental Physical Constants, Complete Listing, 2022 CODATA adjustment" (NIST). Cited in `sources.md`.
**Method:** Both files parsed by script. Entries aligned in order (name and value together). Every field of every aligned pair compared exactly using decimal arithmetic, not floating point.
**Rule for this audit:** report only. Nothing in `constants.rs` was changed. Fixes belong in separate commits after the migration, so the migration can be verified as a pure copy.

## 1. Summary

| Check | Result |
|---|---|
| Entries in NIST listing | **355** |
| `pub static` items in `constants.rs` | **353** (not 351) |
| Entries aligned one-to-one | 353 |
| Missing from `constants.rs` | **2** |
| Extra entries not in NIST | 0 |
| Order matches NIST | Yes, for all 353 |
| `//N` numbering comments match position | Yes, 1 to 353, no gaps or duplicates |
| Duplicate identifiers | 0 |
| Duplicate `name` strings | 1 (`atomic mass constant energy equivalent`, #12 and #13) |
| **Value errors** | **23** |
| **Uncertainty errors** | **12** |
| **Unit errors** | **4** |
| Name differences from NIST | 84 (59 are policy, see 5.1) |
| Identifier problems | 16 real (4 labels on wrong data, 4 wrong identifiers, 8 typos or incomplete), plus an inconsistent `/T` naming convention |
| Exact constants (uncertainty `None`) | 81, matches NIST exactly |

Fields with no defect: for the 353 aligned entries, 330 values, 341 uncertainties and 349 units match NIST exactly.

## 2. Missing entries (2)

Both were probably lost in the renumbering noted by the `//204 oops we had a dup` comment (now removed).

| NIST # | Name | Value | Uncertainty | Unit | Insert between |
|---|---|---|---|---|---|
| 102 | electron-muon mass ratio | 4.836 331 70 e-3 | 0.000 000 11 e-3 | (none) | #101 `ELECTRON_MUON_MAGNETIC_MOMENT_RATIO` and #102 `ELECTRON_NEUTRON_MAGNETIC_MOMENT_RATIO` |
| 212 | muon mass energy equivalent in MeV | 105.658 3755 | 0.000 0023 | MeV | #210 `MUON_MASS_ENERGY_EQUIVALENT` and #211 `MUON_MASS_IN_U` |

Adding them makes the library 355 entries, matching NIST. Inserting shifts every later number by one or two, so the numbering comments will need regenerating (the migration script does this).

## 3. Value errors (23)

Columns: NIST # is the position in the NIST listing. `constants.rs` # is the `//N` comment.

| NIST # | rs # | Identifier | NIST value | Current value | Class |
|---|---|---|---|---|---|
| 35 | 35 | `ATOMIC_UNIT_OF_LENGTH` | 5.291 772 105 44 e-11 | 5.291_722_105_44e-11 | digit slip |
| 38 | 38 | `ATOMIC_UNIT_OF_MAGNETIZABILITY` | 7.891 036 5794 e-29 | 2.350_517_570_77_e5 | wrong source (value copied from #37) |
| 83 | 83 | `DEUTERON_RELATIVE_ATOMIC_MASS` | 2.013 553 212 544 | 2.015_533_212_544 | digit slip |
| 94 | 94 | `ELECTRON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO` | -1.001 159 652 180 46 | -1.001_159_662_180_46 | digit slip |
| 98 | 98 | `ELECTRON_MASS_ENERGY_EQUIVALENT_IN_MEV` | 0.510 998 950 69 | 0.510_988_950_69 | digit slip |
| 99 | 99 | `ELECTRON_MASS_IN_U` | 5.485 799 090 441 e-4 | 5.484_799_090_441e-4 | digit slip |
| 107 | 106 | `ELECTRON_RELATIVE_ATOMIC_MASS` | 5.485 799 090 441 e-4 | 5.486_799_090_411e-4 | digit slip |
| 123 | 122 | `FARADAY_CONSTANT` | 96 485.332 12... | 96.485_332_12 | scale error (x1000) |
| 147 | 146 | `HELION_PROTON_MASS_RATIO` | 2.993 152 671 552 | 2.993_152_617_552 | digit slip |
| 156 | 155 | `HERTZ_KILOGRAM_RELATIONSHIP` | 7.372 497 323... e-51 | 7.372_497_323e51 | exponent sign |
| 182 | 181 | `KILOGRAM_ATOMIC_MASS_UNIT_RELATIONSHIP` | 6.022 140 7537 e26 | 6.002_140_757_7e26 | digit slip |
| 184 | 183 | `KILOGRAM_HARTREE_RELATIONSHIP` | 2.061 485 788 7415 e34 | 2.061_458_788_741_5e34 | digit slip |
| 189 | 188 | `LATTICE_PARAMETER_OF_SILICON` | 5.431 020 511 e-10 | 6.431_020_511e-10 | digit slip |
| 190 | 189 | `LATTICE_SPACING_OF_IDEAL_SI_220` | 1.920 155 716 e-10 | 1.920_155_716_716e-10 | invented digits |
| 202 | 201 | `MOLYBDENUM_X_UNIT` | 1.002 099 52 e-13 | 1.002_009_52e-13 | digit slip |
| 204 | 203 | `MUON_ELECTRON_MASS_RATIO` | 206.768 2827 | 206.768_287_7 | digit slip |
| 222 | 220 | `NATURAL_UNIT_OF_ENERGY_IN_MEV` | 0.510 998 950 69 | 0.510_988_950_69 | digit slip |
| 224 | 222 | `NATURAL_UNIT_OF_MASS` | 9.109 383 7139 e-31 | 9.109_383_713_9e-13 | exponent error |
| 251 | 249 | `NEUTRON_TAU_MASS_RATIO` | 0.528 779 | 0.528_799 | digit slip |
| 307 | 305 | `SHIELDED_HELION_MAGNETIC_MOMENT` | -1.074 553 110 35 e-26 | -1.074_533_110_35e-26 | digit slip |
| 309 | 307 | `SHIELDED_HELION_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO` | -2.127 497 7624 | -2.117_497_762_4 | digit slip |
| 316 | 314 | `SHIELDED_PROTON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO` | 2.792 775 648 | 2.792_755_648 | digit slip |
| 335 | 333 | `TRITON_ELECTRON_MASS_RATIO` | 5496.921 535 51 | 5_469.921_535_51 | digit slip |

Classes: **digit slip** 18, **exponent or scale error** 3 (Faraday constant x1000, hertz-kilogram relationship sign, natural unit of mass e-13 vs e-31), **wrong source** 1, **invented digits** 1 (`LATTICE_SPACING_OF_IDEAL_SI_220` has extra trailing digits that NIST does not give).

`ELECTRON_MASS_ENERGY_EQUIVALENT_IN_MEV` and `NATURAL_UNIT_OF_ENERGY_IN_MEV` share the same digit slip (0.510 998 vs 0.510 988), which suggests one was copied from the other.

## 4. Uncertainty errors (12)

| NIST # | rs # | Identifier | NIST uncertainty | Current | Class |
|---|---|---|---|---|---|
| 38 | 38 | `ATOMIC_UNIT_OF_MAGNETIZABILITY` | 0.000 000 0049 e-29 | 0.000_000_000_73e5 | wrong source |
| 41 | 41 | `ATOMIC_UNIT_OF_PERMITTIVITY` | 0.000 000 000 17 e-10 | 0.000_000_000_17 | exponent missing |
| 76 | 76 | `DEUTERON_MASS_ENERGY_EQUIVALENT` | 0.000 000 000 94 e-10 | 0.000_000_000_94 | exponent missing |
| 135 | 134 | `HARTREE_KELVIN_RELATIONSHIP` | 0.000 000 000 0034 e5 | 0.000_000_000_003_4 | exponent missing |
| 136 | 135 | `HARTREE_KILOGRAM_RELATIONSHIP` | 0.000 000 000 0053 e-35 | 0.000_000_000_005_3 | exponent missing |
| 143 | 142 | `HELION_MASS_ENERGY_EQUIVALENT` | 0.000 000 0014 e-10 | 0.000_000_001_4e10 | exponent sign |
| 148 | 147 | `HELION_RELATIVE_ATOMIC_MASS` | 0.000 000 000 074 | 0.000_000_000_74 | digit place (x10) |
| 149 | 148 | `HELION_SHIELDING_SHIFT` | 0.000 0023 e-5 | 0.000_002_3 | exponent missing |
| 177 | 176 | `KELVIN_HARTREE_RELATIONSHIP` | 0.000 000 000 0035 e-6 | 0.000_000_000_003_5 | exponent missing |
| 203 | 202 | `MUON_COMPTON_WAVELENGTH` | 0.000 000 026 e-14 | 0.000_000_026 | exponent missing |
| 210 | 209 | `MUON_MASS` | 0.000 000 042 e-28 | 0.000_000_42e-28 | digit place (x10) |
| 278 | 276 | `PROTON_MASS_ENERGY_EQUIVALENT` | 0.000 000 000 47 e-10 | 0.000_000_000_47 | exponent missing |

Nine of the twelve are a missing or wrong exponent, so the stored uncertainty is off by a factor such as 1e10. These matter more than a digit slip in the value, because anything that propagates uncertainty would be silently wrong by orders of magnitude.

## 5. Unit errors (4)

| NIST # | rs # | Identifier | NIST unit | Current | Note |
|---|---|---|---|---|---|
| 38 | 38 | `ATOMIC_UNIT_OF_MAGNETIZABILITY` | J T^-2 | T | wrong source (same entry as in section 3) |
| 137 | 136 | `HELION_ELECTRON_MASS_RATIO` | (none) | kg | a mass ratio has no unit |
| 245 | 243 | `NEUTRON_PROTON_MASS_DIFFERENCE` | kg | None | NIST gives kg; current is dimensionless |
| 254 | 252 | `NEWTONIAN_CONSTANT_OF_GRAVITATION_OVER_H_BAR_C` | (GeV/c^2)^-2 | (GeV/c^2)^2 | exponent sign |

## 6. Names and identifiers

### 6.1 Policy decision: NIST abbreviations (59 entries)
NIST writes `mag. mom.`, `gyromag.` and `mag. flux`. `constants.rs` expands them to `magnetic moment`, etc. This is consistent and arguably better, but it means the `name` field is not a verbatim copy of the source. **Decision needed** (see section 8).

### 6.2 Typos in `name` (21)
`raduis`, `electic`, `circluation`, `relatinship`, `electricy`, `convention value` (x3, should be `conventional`), `Thomas` (should be `Thomson`), `atomic unit electrical field gradient`, `mag. mom to` (missing period), `helion to mass energy equivalent` (stray `to`), `natural unit momentum` (missing `of`), `shielding difference to d and p` (NIST: `of d and p`), and the shortened `Sackur-Tetrode (...)` and `Wien wavelength displacement law` (NIST adds `constant`).

Three of these change meaning and deserve attention (numbers are `constants.rs` `//N` positions):
- #13 name omits `in MeV` (`ATOMIC_MASS_CONSTANT_ENERGY_EQUIVALENT_IN_MEV`), which also creates the one duplicate name.
- #54 name drops `inverse` (`Boltzmann constant in meter per kelvin`).
- #199 states **273.13 K**; NIST says **273.15 K**. The wrong temperature is also in the identifier (`..._273_13_K_...`).

### 6.3 Wrong labels on correct data (4)
The values and units are right (they match NIST at that position). The identifier and name describe a different constant.

| rs # | Identifier now | Data is actually | Suggested identifier |
|---|---|---|---|
| 11 | `ANOMALOUS_MAGNETIC_MOMENT_OF_THE_MUON` | atomic mass constant (1.66053906892e-27 kg) | `ATOMIC_MASS_CONSTANT` |
| 55 | `BOLTZMANN_CONSTANT_IN_KELVIN_PER_TESLA` | characteristic impedance of vacuum (376.73 ohm) | `CHARACTERISTIC_IMPEDANCE_OF_VACUUM` |
| 58 | `COMPTON_WAVELENGTH_OVER_2_PI` | conductance quantum (7.748e-5 S) | `CONDUCTANCE_QUANTUM` |
| 59 | `CONDUCTANCE_QUANTUM` | conventional value of ampere-90 (1.00000008887 A) | `CONVENTIONAL_VALUE_OF_AMPERE_90` |

### 6.4 Wrong identifiers on entries whose `name` is correct (4)

| rs # | Identifier now | Name (matches NIST) | Suggested identifier |
|---|---|---|---|
| 7 | `ALPHA_PARTICLE_NEUTRON_MASS_RATIO` | alpha particle-proton mass ratio | `ALPHA_PARTICLE_PROTON_MASS_RATIO` |
| 8 | `ALPHA_PARTICLE_PROTON_MASS_RATIO` | alpha particle relative atomic mass | `ALPHA_PARTICLE_RELATIVE_ATOMIC_MASS` |
| 289 | `REDUCED_ELECTRON_COMPTON_WAVELENGTH` | reduced muon Compton wavelength | `REDUCED_MUON_COMPTON_WAVELENGTH` |
| 199 | `MOLAR_VOLUME_IDEAL_GAS_273_13_K_101_325_KPA` | (see 6.2, 273.15 K) | `..._273_15_K_...` |

Note #7 and #8: the identifier of #7 is taken by #8's data, so fixing #7 first creates a temporary duplicate. Change both in one commit.

### 6.5 Typos in identifiers (8)
`SECOND_RADATION_CONSTANT`, `TAU_ELECTRON_MASS_RATION`, `SHIELDED_PROTON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATION`, `TRITION_MASS_ENERGY_EQUIVALENT_IN_MEV`, `VON_KLITIZING_CONSTANT`, `VACUUM_ELECTRICITY_PERMITTIVITY` (NIST: electric), `TRITON_MAGNETIC_MOMENT_TO_BOHR_RATIO` (missing MAGNETON), `TRITON_MOMENT_TO_NUCLEAR_MAGNETON_RATIO` (missing MAGNETIC).

### 6.6 Inconsistent unit-in-name convention
Names ending in "/T" use at least three spellings in identifiers: `_PER_TESLA`, `_PER_T`, `_T` (for example `ELECTRON_..._IN_MHZ_PER_TESLA`, `PROTON_..._IN_MHZ_PER_T`, `NEUTRON_..._IN_MHZ_T`). Pick one before search is built, because a search key depends on it.

## 7. Model-level findings (not defects, but they shape the refactor)

1. **`si_base_units` is a misnomer.** Of 261 entries that have a unit, **165 (63%)** use units that are not SI base units: `J`, `Hz`, `eV`, `MeV`, `u`, `E_h`, `GeV`, `T`, `V`, `ohm`, and so on. The field is really `unit`. Only the other 96 are pure combinations of `m kg s A K mol`.
2. **`None` conflates two things.** 92 entries have `si_base_units: None`, and `Display` prints "Dimensionless". For ratios that is right. But the unit-error check shows one ratio wrongly given `kg` and one kg-valued entry given `None`, so `None` cannot currently be trusted to mean dimensionless.
3. **Exact constants are stored with limited digits.** NIST marks 81 entries `(exact)`, and 62 of them show only the first 9 to 12 significant digits (NIST ends them with `...`). They are computed from defined constants (h, c, e, k, N_A), so f64 could hold about 16 digits. Today uncertainty `None` claims "exact" while the stored value is truncated at 9 to 12 significant digits, a relative error of up to about 1e-9. The `//...` marker comments are inconsistent: 46 have `//...`, 2 have `//..`, **14 have none**, and `INVERSE_METER_HARTREE_RELATIONSHIP` has a marker although NIST gives it an uncertainty.
4. **Uncertainty is stored as an absolute number**, but NIST's exponent is shared with the value. That is why exponent slips (section 4) are easy to make. The migration can keep the absolute form and still preserve every digit.
5. **No collection.** Each constant is a standalone `pub static`, and Rust cannot enumerate those. Search needs one list of all entries (a slice, or a macro that builds both the statics and the list).
6. **`Display` embeds ANSI colour codes** (`\x1b[32m`) in a science type. Terminal styling belongs in the command layer.
7. **Style only:** one literal has an underscore before the exponent (`2.350_517_570_77_e5`, which is also the wrong-source entry), and the symbol and alias TODO in the file header is still open.

## 8. Decisions needed before the migration

1. **Names:** keep the expanded spellings (`magnetic moment`) or restore NIST's verbatim text? I suggest keeping a `name` in full words and storing NIST's text separately only if needed.
2. **The 2 missing entries:** add them as part of the migration, or migrate 353 first and add them in a second commit? I suggest second, so the migration is a verifiable copy.
3. **What `migrate` preserves:** I suggest the copy is byte-faithful, defects included, and fixes land afterward one class at a time (values, then uncertainties, then units, then labels), each with its own commit and a test against NIST.
4. **Representation of units:** keep the free-text string for now and decide the typed form (tied to `Dimensions` and `Unit` in `science::physical::units`) later.

## 9. Reproducing this audit

The audit is deterministic. The comparison treats `allascii.txt` as the authority and uses exact decimal parsing. The fixed form of this check should become a Rust test (every constant's value, uncertainty and unit equals the NIST table) so the library keeps proving itself.

