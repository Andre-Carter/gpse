pub struct Element {
    pub atomic_number: u8,
    pub symbol: &'static str,
    pub name: &'static str,
    pub chemical_group: &'static str,
    pub atomic_mass: f64,
}

pub static HYDROGEN: Element = Element {
    atomic_number: 1,
    symbol: "H",
    name: "Hydrogen",
    chemical_group: "Nonmetal",
    atomic_mass: 1.0080,
};

pub static HELIUM: Element = Element {
    atomic_number: 2,
    symbol: "He",
    name: "Helium",
    chemical_group: "Noble Gas",
    atomic_mass: 4.00260,
};

pub static LITHIUM: Element = Element {
    atomic_number: 3,
    symbol: "Li",
    name: "Lithium",
    chemical_group: "Alkali Metal",
    atomic_mass: 7.0,
};

pub static BERYLLIUM: Element = Element {
    atomic_number: 4,
    symbol: "Be",
    name: "Beryllium",
    chemical_group: "Alkaline Earth Metal",
    atomic_mass: 9.012183,
};

pub static BORON: Element = Element {
    atomic_number: 5,
    symbol: "B",
    name: "Boron",
    chemical_group: "Metalloid",
    atomic_mass: 10.81,
};

pub static CARBON: Element = Element {
    atomic_number: 6,
    symbol: "C",
    name: "Carbon",
    chemical_group: "Nonmetal",
    atomic_mass: 12.011,
};

pub static NITROGEN: Element = Element {
    atomic_number: 7,
    symbol: "N",
    name: "Nitrogen",
    chemical_group: "Nonmetal",
    atomic_mass: 14.007,
};

pub static OXYGEN: Element = Element {
    atomic_number: 8,
    symbol: "O",
    name: "Oxygen",
    chemical_group: "Nonmetal",
    atomic_mass: 15.999,
};

pub static FLUORINE: Element = Element {
    atomic_number: 9,
    symbol: "F",
    name: "Fluorine",
    chemical_group: "Halogen",
    atomic_mass: 18.99840316,
};

pub static NEON: Element = Element {
    atomic_number: 10,
    symbol: "Ne",
    name: "Neon",
    chemical_group: "Noble Gas",
    atomic_mass: 20.180,
};

pub static SODIUM: Element = Element {
    atomic_number: 11,
    symbol: "Na",
    name: "Sodium",
    chemical_group: "Alkali Metal",
    atomic_mass: 22.9897693,
};

pub static MAGNESIUM: Element = Element {
    atomic_number: 12,
    symbol: "Mg",
    name: "Magnesium",
    chemical_group: "Alkaline Earth Metal",
    atomic_mass: 24.305,
};

pub static ALUMINUM: Element = Element {
    atomic_number: 13,
    symbol: "Al",
    name: "Aluminum",
    chemical_group: "Post-Transition Metal",
    atomic_mass: 26.981538,
};

pub static SILICON: Element = Element {
    atomic_number: 14,
    symbol: "Si",
    name: "Silicon",
    chemical_group: "Metalloid",
    atomic_mass: 28.085,
};

pub static PHOSPHORUS: Element = Element {
    atomic_number: 15,
    symbol: "P",
    name: "Phosphorus",
    chemical_group: "Nonmetal",
    atomic_mass: 30.97376200,
};

pub static SULFUR: Element = Element {
    atomic_number: 16,
    symbol: "S",
    name: "Sulfur",
    chemical_group: "Nonmetal",
    atomic_mass: 32.07,
};

pub static CHLORINE: Element = Element {
    atomic_number: 17,
    symbol: "Cl",
    name: "Chlorine",
    chemical_group: "Halogen",
    atomic_mass: 35.45,
};

pub static ARGON: Element = Element {
    atomic_number: 18,
    symbol: "Ar",
    name: "Argon",
    chemical_group: "Noble Gas",
    atomic_mass: 39.9,
};

pub static POTASSIUM: Element = Element {
    atomic_number: 19,
    symbol: "K",
    name: "Potassium",
    chemical_group: "Alkali Metal",
    atomic_mass: 39.0983,
};

pub static CALCIUM: Element = Element {
    atomic_number: 20,
    symbol: "Ca",
    name: "Calcium",
    chemical_group: "Alkaline Earth Metal",
    atomic_mass: 40.08,
};

pub static SCANDIUM: Element = Element {
    atomic_number: 21,
    symbol: "Sc",
    name: "Scandium",
    chemical_group: "Scandium",
    atomic_mass: 44.95591,
};

pub static TITANIUM: Element = Element {
    atomic_number: 22,
    symbol: "Ti",
    name: "Titanium",
    chemical_group: "Transition Metal",
    atomic_mass: 47.867,
};

pub static VANADIUM: Element = Element {
    atomic_number: 23,
    symbol: "V",
    name: "Vanadium",
    chemical_group: "Transition Metal",
    atomic_mass: 50.9415,
};

pub static CHROMIUM: Element = Element {
    atomic_number: 24,
    symbol: "Cr",
    name: "Chromium",
    chemical_group: "Transition Metal",
    atomic_mass: 51.996,
};

pub static MANGANESE: Element = Element {
    atomic_number: 25,
    symbol: "Mn",
    name: "Manganese",
    chemical_group: "Transition Metal",
    atomic_mass: 54.93804,
};

pub static IRON: Element = Element {
    atomic_number: 26,
    symbol: "Fe",
    name: "Iron",
    chemical_group: "Transition Metal",
    atomic_mass: 55.84,
};

pub static COBALT: Element = Element {
    atomic_number: 27,
    symbol: "Co",
    name: "Cobalt",
    chemical_group: "Transition Metal",
    atomic_mass: 58.93319,
};

pub static NICKLE: Element = Element {
    atomic_number: 28,
    symbol: "Ni",
    name: "Nickle",
    chemical_group: "Transition Metal",
    atomic_mass: 58.693,
};

pub static COPPER: Element = Element {
    atomic_number: 29,
    symbol: "Cu",
    name: "Copper",
    chemical_group: "Transition Metal",
    atomic_mass: 63.55,
};

pub static ZINC: Element = Element {
    atomic_number: 30,
    symbol: "Zn",
    name: "Zinc",
    chemical_group: "Zinc",
    atomic_mass: 65.4,
};

pub static GALLIUM: Element = Element {
    atomic_number: 31,
    symbol: "Ga",
    name: "Gallium",
    chemical_group: "Post-Transition Metal",
    atomic_mass: 69.723,
};

pub static GERMANIUM: Element = Element {
    atomic_number: 32,
    symbol: "Ge",
    name: "Germanium",
    chemical_group: "Metalloid",
    atomic_mass: 72.63,
};

pub static ARSENIC: Element = Element {
    atomic_number: 33,
    symbol: "As",
    name: "Arsenic",
    chemical_group: "Metalloid",
    atomic_mass: 74.92159,
};

pub static SELENIUM: Element = Element {
    atomic_number: 34,
    symbol: "Se",
    name: "Selenium",
    chemical_group: "Nonmetal",
    atomic_mass: 78.97,
};

pub static BROMINE: Element = Element {
    atomic_number: 35,
    symbol: "Br",
    name: "Bromine",
    chemical_group: "Halogen",
    atomic_mass: 79.90,
};

pub static KRYPTON: Element = Element {
    atomic_number: 36,
    symbol: "Kr",
    name: "Krypton",
    chemical_group: "Nobel Gas",
    atomic_mass: 83.80,
};

pub static RUBIDIUM: Element = Element {
    atomic_number: 37,
    symbol: "Rb",
    name: "Rubidium",
    chemical_group: "Alkali Metal",
    atomic_mass: 85.468,
};

pub static STRONTIUM: Element = Element {
    atomic_number: 38,
    symbol: "Sr",
    name: "Strontium",
    chemical_group: "Alkaline Earth Metal",
    atomic_mass: 87.62,
};

pub static YTTRIUM: Element = Element {
    atomic_number: 39,
    symbol: "Y",
    name: "Yttrium",
    chemical_group: "Transition Metal",
    atomic_mass: 88.90584,
};

pub static ZIRCONIUM: Element = Element {
    atomic_number: 40,
    symbol: "Zr",
    name: "Zirconium",
    chemical_group: "Transition Metal",
    atomic_mass: 91.22,
};

pub static NIOBIUM: Element = Element {
    atomic_number: 41,
    symbol: "Nb",
    name: "Niobium",
    chemical_group: "Transition Metal",
    atomic_mass: 92.90637,
};

pub static MOLYBDENUM: Element = Element {
    atomic_number: 42,
    symbol: "Mo",
    name: "Molybdenum",
    chemical_group: "Transition Metal",
    atomic_mass: 95.95,
};

pub static TECHNETIUM: Element = Element {
    atomic_number: 43,
    symbol: "Tc",
    name: "Technetium",
    chemical_group: "Transition Metal",
    atomic_mass: 96.90636,
};

pub static RUTHENIUM: Element = Element {
    atomic_number: 44,
    symbol: "Ru",
    name: "Ruthenium",
    chemical_group: "Transition Metal",
    atomic_mass: 101.1,
};

pub static RHODIUM: Element = Element {
    atomic_number: 45,
    symbol: "Rh",
    name: "Rhodium",
    chemical_group: "Transition Metal",
    atomic_mass: 102.9055,
};

pub static PALLADIUM: Element = Element {
    atomic_number: 46,
    symbol: "Pd",
    name: "Palladium",
    chemical_group: "Transition Metal",
    atomic_mass: 106.42,
};

pub static SILVER: Element = Element {
    atomic_number: 47,
    symbol: "Ag",
    name: "Silver",
    chemical_group: "Transition Metal",
    atomic_mass: 107.868,
};

pub static CADMIUM: Element = Element {
    atomic_number: 48,
    symbol: "Cd",
    name: "Cadmium",
    chemical_group: "Transition Metal",
    atomic_mass: 112.41,
};

pub static INDIUM: Element = Element {
    atomic_number: 49,
    symbol: "In",
    name: "Indium",
    chemical_group: "Post-Transition Metal",
    atomic_mass: 114.818,
};

pub static TIN: Element = Element {
    atomic_number: 50,
    symbol: "Sn",
    name: "Tin",
    chemical_group: "Post-Transition Metal",
    atomic_mass: 118.71,
};

pub static ANTIMONY: Element = Element {
    atomic_number: 51,
    symbol: "Sb",
    name: "Antimony",
    chemical_group: "Metalloid",
    atomic_mass: 121.760,
};

pub static TELLURIUM: Element = Element {
    atomic_number: 52,
    symbol: "Te",
    name: "Tellurium",
    chemical_group: "Metalloid",
    atomic_mass: 127.6,
};

pub static IODINE: Element = Element {
    atomic_number: 53,
    symbol: "I",
    name: "Iodine",
    chemical_group: "Halogen",
    atomic_mass: 126.9045,
};

pub static XENON: Element = Element {
    atomic_number: 54,
    symbol: "Xe",
    name: "Xenon",
    chemical_group: "Noble Gas",
    atomic_mass: 131.29,
};

pub static CESIUM: Element = Element {
    atomic_number: 55,
    symbol: "Cs",
    name: "Cesium",
    chemical_group: "Alkali Metal",
    atomic_mass: 132.9054520,
};

pub static BARIUM: Element = Element {
    atomic_number: 56,
    symbol: "Ba",
    name: "Barium",
    chemical_group: "Alkaline Earth Metal",
    atomic_mass: 137.33,
};

pub static LANTHANUM: Element = Element {
    atomic_number: 57,
    symbol: "La",
    name: "Lanthanum",
    chemical_group: "Lanthanide",
    atomic_mass: 138.9055,
};

pub static CERIUM: Element = Element {
    atomic_number: 58,
    symbol: "Ce",
    name: "Cerium",
    chemical_group: "Lanthanide",
    atomic_mass: 140.116,
};

pub static PRASEODYMIUM: Element = Element {
    atomic_number: 59,
    symbol: "Pr",
    name: "Praseodymium",
    chemical_group: "Lanthanide",
    atomic_mass: 140.90766,
};

pub static NEODYMIUM: Element = Element {
    atomic_number: 60,
    symbol: "Nd",
    name: "Neodymium",
    chemical_group: "Lanthanide",
    atomic_mass: 144.24,
};

pub static PROMETHIUM: Element = Element {
    atomic_number: 61,
    symbol: "Pm",
    name: "Promethium",
    chemical_group: "Lanthanide",
    atomic_mass: 144.91276,
};

pub static SAMARIUM: Element = Element {
    atomic_number: 62,
    symbol: "Sm",
    name: "Samarium",
    chemical_group: "Lanthanide",
    atomic_mass: 150.4,
};

pub static EUROPIUM: Element = Element {
    atomic_number: 63,
    symbol: "Eu",
    name: "Europium",
    chemical_group: "Lanthanide",
    atomic_mass: 151.964,
};

pub static GADOLINIUM: Element = Element {
    atomic_number: 64,
    symbol: "Gd",
    name: "Gadolinium",
    chemical_group: "Lanthanide",
    atomic_mass: 157.25,
};

pub static TERBIUM: Element = Element {
    atomic_number: 65,
    symbol: "Tb",
    name: "Terbium",
    chemical_group: "Lanthanide",
    atomic_mass: 158.92535,
};

pub static DYSPROSIUM: Element = Element {
    atomic_number: 66,
    symbol: "Dy",
    name: "Dysprosium",
    chemical_group: "Lanthanide",
    atomic_mass: 162.500,
};

pub static HOLMIUM: Element = Element {
    atomic_number: 67,
    symbol: "Ho",
    name: "Holmium",
    chemical_group: "Lanthanide",
    atomic_mass: 164.93033,
};

pub static ERBIUM: Element = Element {
    atomic_number: 68,
    symbol: "Er",
    name: "Erbium",
    chemical_group: "Lanthanide",
    atomic_mass: 167.26,
};

pub static THULIUM: Element = Element {
    atomic_number: 69,
    symbol: "Tm",
    name: "Thulium",
    chemical_group: "Lanthanide",
    atomic_mass: 168.93422,
};

pub static YTTERBIUM: Element = Element {
    atomic_number: 70,
    symbol: "Yb",
    name: "Ytterbium",
    chemical_group: "Lanthanide",
    atomic_mass: 173.05,
};

pub static LUTETIUM: Element = Element {
    atomic_number: 71,
    symbol: "Lu",
    name: "Lutetium",
    chemical_group: "Lanthanide",
    atomic_mass: 174.9667,
};

pub static HAFNIUM: Element = Element {
    atomic_number: 72,
    symbol: "Hf",
    name: "Hafnium",
    chemical_group: "Transition Metal",
    atomic_mass: 178.49,
};

pub static TANTALUM: Element = Element {
    atomic_number: 73,
    symbol: "Ta",
    name: "Tantalum",
    chemical_group: "Transition Metal",
    atomic_mass: 180.9479,
};

pub static TUNGSTEN: Element = Element {
    atomic_number: 74,
    symbol: "W",
    name: "Tungsten",
    chemical_group: "Transition Metal",
    atomic_mass: 183.84,
};

pub static RHENIUM: Element = Element {
    atomic_number: 75,
    symbol: "Re",
    name: "Rhenium",
    chemical_group: "Transition Metal",
    atomic_mass: 186.207,
};

pub static OSMIUM: Element = Element {
    atomic_number: 76,
    symbol: "Os",
    name: "Osmium",
    chemical_group: "Transition Metal",
    atomic_mass: 190.2,
};

pub static IRIDIUM: Element = Element {
    atomic_number: 77,
    symbol: "Ir",
    name: "Iridium",
    chemical_group: "Transition Metal",
    atomic_mass: 192.22,
};

pub static PLATINUM: Element = Element {
    atomic_number: 78,
    symbol: "Pt",
    name: "Platinum",
    chemical_group: "Transition Metal",
    atomic_mass: 195.08,
};

pub static GOLD: Element = Element {
    atomic_number: 79,
    symbol: "Au",
    name: "Gold",
    chemical_group: "Transition Metal",
    atomic_mass: 196.96657,
};

pub static MERCURY: Element = Element {
    atomic_number: 80,
    symbol: "Hg",
    name: "Mercury",
    chemical_group: "Transition Metal",
    atomic_mass: 200.59,
};

pub static THALLIUM: Element = Element {
    atomic_number: 81,
    symbol: "Tl",
    name: "Thallium",
    chemical_group: "Post-Transition Metal",
    atomic_mass: 204.383,
};

pub static LEAD: Element = Element {
    atomic_number: 82,
    symbol: "Pb",
    name: "Lead",
    chemical_group: "Post-Transition Metal",
    atomic_mass: 207.0,
};

pub static BISMUTH: Element = Element {
    atomic_number: 83,
    symbol: "Bi",
    name: "Bismuth",
    chemical_group: "Post-Transition Metal",
    atomic_mass: 208.98040,
};

pub static POLONIUM: Element = Element {
    atomic_number: 84,
    symbol: "Po",
    name: "Polonium",
    chemical_group: "Metalloid",
    atomic_mass: 208.98243,
};

pub static ASTATINE: Element = Element {
    atomic_number: 85,
    symbol: "At",
    name: "Astatine",
    chemical_group: "Halogen",
    atomic_mass: 209.98715,
};

pub static RADON: Element = Element {
    atomic_number: 86,
    symbol: "Rn",
    name: "Radon",
    chemical_group: "Noble Gas",
    atomic_mass: 222.01758,
};

pub static FRANCIUM: Element = Element {
    atomic_number: 87,
    symbol: "Fr",
    name: "Francium",
    chemical_group: "Alkali Metal",
    atomic_mass: 223.01973,
};

pub static RADIUM: Element = Element {
    atomic_number: 88,
    symbol: "Ra",
    name: "Radium",
    chemical_group: "Alkaline Earth Metal",
    atomic_mass: 226.02541,
};

pub static ACTINIUM: Element = Element {
    atomic_number: 89,
    symbol: "Ac",
    name: "Actinium",
    chemical_group: "Actinide",
    atomic_mass: 277.02775,
};

pub static THORIUM: Element = Element {
    atomic_number: 90,
    symbol: "Th",
    name: "Thorium",
    chemical_group: "Actinide",
    atomic_mass: 232.038,
};

pub static PROTACTINIUM: Element = Element {
    atomic_number: 91,
    symbol: "Pa",
    name: "Protactinium",
    chemical_group: "Actinide",
    atomic_mass: 231.03588,
};

pub static URANIUM: Element = Element {
    atomic_number: 92,
    symbol: "U",
    name: "Uranium",
    chemical_group: "Actinide",
    atomic_mass: 238.0289,
};

pub static NEPTUNIUM: Element = Element {
    atomic_number: 93,
    symbol: "Np",
    name: "Neptunium",
    chemical_group: "Actinide",
    atomic_mass: 237.048172,
};

pub static PLUTONIUM: Element = Element {
    atomic_number: 94,
    symbol: "Pu",
    name: "Plutonium",
    chemical_group: "Actinide",
    atomic_mass: 244.06420,
};

pub static AMERICIUM: Element = Element {
    atomic_number: 95,
    symbol: "Am",
    name: "Americium",
    chemical_group: "Actinide",
    atomic_mass: 243.061380,
};

pub static CURIUM: Element = Element {
    atomic_number: 96,
    symbol: "Cm",
    name: "Curium",
    chemical_group: "Actinide",
    atomic_mass: 247.07035,
};

pub static BERKELIUM: Element = Element {
    atomic_number: 97,
    symbol: "Bk",
    name: "Berkelium",
    chemical_group: "Actinide",
    atomic_mass: 247.07031,
};

pub static CALIFORNIUM: Element = Element {
    atomic_number: 98,
    symbol: "Cf",
    name: "Californium",
    chemical_group: "Actinide",
    atomic_mass: 251.07959,
};

pub static EINSTEINIUM: Element = Element {
    atomic_number: 99,
    symbol: "Es",
    name: "Einsteinium",
    chemical_group: "Actinide",
    atomic_mass: 252.0830,
};

pub static FERMIUM: Element = Element {
    atomic_number: 100,
    symbol: "Fm",
    name: "Fermium",
    chemical_group: "Actinide",
    atomic_mass: 257.09511,
};

pub static MENDELEVIUM: Element = Element {
    atomic_number: 101,
    symbol: "Md",
    name: "Mendelevium",
    chemical_group: "Actinide",
    atomic_mass: 258.09843,
};

pub static NOBELIUM: Element = Element {
    atomic_number: 102,
    symbol: "No",
    name: "Nobelium",
    chemical_group: "Actinide",
    atomic_mass: 259.10100,
};

pub static LAWRENCIUM: Element = Element {
    atomic_number: 103,
    symbol: "Lr",
    name: "Lawrencium",
    chemical_group: "Actinide",
    atomic_mass: 266.120,
};

pub static RUTHERFORDIUM: Element = Element {
    atomic_number: 104,
    symbol: "Rf",
    name: "Rutherfordium",
    chemical_group: "Transition Metal",
    atomic_mass: 267.122,
};

pub static DUBNIUM: Element = Element {
    atomic_number: 105,
    symbol: "Db",
    name: "Dubnium",
    chemical_group: "Transition Metal",
    atomic_mass: 268.126,
};

pub static SEABORGIUM: Element = Element {
    atomic_number: 106,
    symbol: "Sg",
    name: "Seaborgium",
    chemical_group: "Transition Metal",
    atomic_mass: 269.128,
};

pub static BOHRIUM: Element = Element {
    atomic_number: 107,
    symbol: "Bh",
    name: "Bohrium",
    chemical_group: "Transition Metal",
    atomic_mass: 270.133,
};

pub static HASSIUM: Element = Element {
    atomic_number: 108,
    symbol: "Hs",
    name: "Hassium",
    chemical_group: "Transition Metal",
    atomic_mass: 269.1336,
};

pub static MEITNERIUM: Element = Element {
    atomic_number: 109,
    symbol: "Mt",
    name: "Meitnerium",
    chemical_group: "Transition Metal",
    atomic_mass: 277.154,
};

pub static DARMSTADTIUM: Element = Element {
    atomic_number: 110,
    symbol: "Dr",
    name: "Darmstadtium",
    chemical_group: "Transition Metal",
    atomic_mass: 282.166,
};

pub static ROENTGENIUM: Element = Element {
    atomic_number: 111,
    symbol: "Rg",
    name: "Roentgenium",
    chemical_group: "Transition Metal",
    atomic_mass: 282.169,
};

pub static COPERNICIUM: Element = Element {
    atomic_number: 112,
    symbol: "Cn",
    name: "Copernicium",
    chemical_group: "Transition Metal",
    atomic_mass: 286.179,
};

pub static NIHONIUM: Element = Element {
    atomic_number: 113,
    symbol: "Nh",
    name: "Nihonium",
    chemical_group: "Post-Transition Metal",
    atomic_mass: 286.182,
};

pub static FLEROVIUM: Element = Element {
    atomic_number: 114,
    symbol: "Fl",
    name: "Flerovium",
    chemical_group: "Post-Transition Metal",
    atomic_mass: 290.192,
};

pub static MOSCOVIUM: Element = Element {
    atomic_number: 115,
    symbol: "Mc",
    name: "Moscovium",
    chemical_group: "Post-Transition Metal",
    atomic_mass: 290.196,
};

pub static LIVERMORIUM: Element = Element {
    atomic_number: 116,
    symbol: "Lv",
    name: "Livermorium",
    chemical_group: "Post-Transition Metal",
    atomic_mass: 293.205,
};

pub static TENNESSINE: Element = Element {
    atomic_number: 117,
    symbol: "Ts",
    name: "Tennessine",
    chemical_group: "Halogen",
    atomic_mass: 294.211,
};

pub static OGANESSON: Element = Element {
    atomic_number: 118,
    symbol: "Og",
    name: "Oganesson",
    chemical_group: "Noble Gas",
    atomic_mass: 295.216,
};

