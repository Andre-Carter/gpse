use std::fmt;

#[derive(Debug)]
pub enum Uncertainty {
    Exact,
    Known(f64),
    NotProvided,
    NotApplicable,
}

#[derive(Debug)]
pub struct Constant {
    // identifier
    pub name: &'static str,
    // TODO: Revisit symbol/alias metadata once the constant API is established.
    pub symbol: Option<&'static str>,
    pub value: f64,
    pub units: Option<&'static str>,
    pub uncertainty: Uncertainty,
    //pub uncertainty: Option<f64>,
}

/*


example: uncertainty: Uncertainty,

e.g Uncertainty::Exact,
e.g Uncertainty::Known(0.00015e-11),
Uncertainty::Known(),

pub enum Symbol {
    Primary,
    Secondary,
    NotEstablished,
}
*/

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const GREEN: &str = "\x1b[32m";
        const RESET: &str = "\x1b[0m";

        writeln!(f, "{}Constant: {}{}", GREEN, self.name, RESET)?;
        
        match self.symbol {
            Some(symbol) => writeln!(f, "Symbol: {}", symbol)?,
            None => writeln!(f, "Symbol: None")?,
        }
        
        writeln!(f, "Value: {}", self.value)?;

        match self.units {
            Some(units) => writeln!(f, "Units: {}", units)?,
            None => writeln!(f, "Units: Dimensionless")?,
        }

        match self.uncertainty {
            Uncertainty::Exact => {
                writeln!(f, "Uncertainty: Exact")?;
            }

            Uncertainty::Known(_) => {
                writeln!(f, "Uncertainty: {:?}", self.uncertainty)?;
            }

            Uncertainty::NotProvided => {
                writeln!(f, "Uncertainty: Not Provided")?;
            }

            Uncertainty::NotApplicable => {
                writeln!(f, "Uncertainty: Not Applicable")?;
            }
        }

        Ok(())
    }
}

// Source: https://pml.nist.gov/cuu/Constants/Table/allascii.txt

//1
pub static ALPHA_PARTICLE_ELECTRON_MASS_RATIO: Constant = Constant {
    name: "alpha particle-electron mass ratio",
    symbol: None,
    value: 7_294.299_541_71,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_17),
};
//2
pub static ALPHA_PARTICLE_MASS: Constant = Constant {
    name: "alpha particle mass",
    symbol: None,
    value: 6.644_657_345_0e-27,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_000_002_1e-27),
};
//3
pub static ALPHA_PARTICLE_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "alpha particle mass energy equivalent",
    symbol: None,
    value: 5.971_920_199_7e-10,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_001_9e-10),
};
//4
pub static ALPHA_PARTICLE_MASS_ENERGY_EQUIVALENT_IN_MEGA_ELECTRON_VOLT: Constant = Constant {
    name: "alpha particle mass energy equivalent in mega electron volt",
    symbol: None,
    value: 3_727.379_411_8,
    units: Some("MeV"),
    uncertainty: Uncertainty::Known(0.000_001_2),
};
//5
pub static ALPHA_PARTICLE_MASS_IN_U: Constant = Constant {
    name: "alpha particle mass in u",
    symbol: None,
    value: 4.001_506_179_129,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_000_062),
};
//6
pub static ALPHA_PARTICLE_MOLAR_MASS: Constant = Constant {
    name: "alpha particle molar mass",
    symbol: None,
    value: 4.001_506_183_3e-3,
    units: Some("kg mol^-1"),
    uncertainty: Uncertainty::Known(0.000_000_001_2e-3),
};
//7
pub static ALPHA_PARTICLE_PROTON_MASS_RATIO: Constant = Constant {
    name: "alpha particle-proton mass ratio",
    symbol: None,
    value: 3.972_599_690_252,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_07),
};
//8
pub static ALPHA_PARTICLE_RELATIVE_ATOMIC_MASS: Constant = Constant {
    name: "alpha particle relative atomic mass",
    symbol: None,
    value: 4.001_506_179_129,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_062),
};
//9
pub static ALPHA_PARTICLE_RMS_CHARGE_RADIUS: Constant = Constant {
    name: "alpha particle rms charge radius",
    symbol: None,
    value: 1.678_5e-15,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.002_1e-15),
};
//10
pub static ANGSTROM_STAR: Constant = Constant {
    name: "Angstrom star", // A is capitalized on nist
    symbol: None,
    value: 1.000_014_95e-10,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_9e-10),
};
//11
pub static ATOMIC_MASS_CONSTANT: Constant = Constant {
    name: "atomic mass constant",
    symbol: Some(r"\(m_u\)"),
    value: 1.660_539_068_92e-27,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_000_000_52e-27),
};
//12
pub static ATOMIC_MASS_CONSTANT_ENERGY_EQUIVALENT: Constant = Constant {
    name: "atomic mass constant energy equivalent",
    symbol: None,
    value: 1.492_418_087_68e-10,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_000_46e-10),
};
//13
pub static ATOMIC_MASS_CONSTANT_ENERGY_EQUIVALENT_IN_MEGA_ELECTRON_VOLT: Constant = Constant {
    name: "atomic mass constant energy equivalent in mega electron volt",
    symbol: None,
    value: 931.494_103_72,
    units: Some("MeV"),
    uncertainty: Uncertainty::Known(0.000_000_29),
};
//14
pub static ATOMIC_MASS_UNIT_ELECTRON_VOLT_RELATIONSHIP: Constant = Constant {
    name: "atomic mass unit-electron volt relationship",
    symbol: None,
    value: 9.314_941_037_2e8,
    units: Some("eV"),
    uncertainty: Uncertainty::Known(0.000_000_002_9e8),
};
//15
pub static ATOMIC_MASS_UNIT_HARTREE_RELATIONSHIP: Constant = Constant {
    name: "atomic mass unit-hartree relationship",
    symbol: None,
    value: 3.423_177_692_2e7,
    units: Some("E_h"),
    uncertainty: Uncertainty::Known(0.000_000_001_1e7),
};
//16
pub static ATOMIC_MASS_UNIT_HERTZ_RELATIONSHIP: Constant = Constant {
    name: "atomic mass unit-hertz relationship",
    symbol: None,
    value: 2.252_342_721_85e23,
    units: Some("Hz"),
    uncertainty: Uncertainty::Known(0.000_000_000_7e23),
};
//17
pub static ATOMIC_MASS_UNIT_INVERSE_METER_RELATIONSHIP: Constant = Constant {
    name: "atomic mass unit-inverse meter relationship",
    symbol: None,
    value: 7.513_006_620_9e14,
    units: Some("m^-1"),
    uncertainty: Uncertainty::Known(0.000_000_002_3e14),
};
//18
pub static ATOMIC_MASS_UNIT_JOULE_RELATIONSHIP: Constant = Constant {
    name: "atomic mass unit-joule relationship",
    symbol: None,
    value: 1.492_418_087_68e-10,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_000_46e-10),
};
//19
pub static ATOMIC_MASS_UNIT_KELVIN_RELATIONSHIP: Constant = Constant {
    name: "atomic mass unit-kelvin relationship",
    symbol: None,
    value: 1.080_954_020_67e13,
    units: Some("K"),
    uncertainty: Uncertainty::Known(0.000_000_000_34e13),
};
//20
pub static ATOMIC_MASS_UNIT_KILOGRAM_RELATIONSHIP: Constant = Constant {
    name: "atomic mass unit-kilogram relationship",
    symbol: None,
    value: 1.660_539_068_92e-27,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_000_000_52e-27),
};
//21
pub static ATOMIC_UNIT_OF_1ST_HYPERPOLARIZABILITY: Constant = Constant {
    name: "atomic unit of 1st hyperpolarizability",
    symbol: None,
    value: 3.206_361_299_6e-53,
    units: Some("C^3 m^3 J^-2"),
    uncertainty: Uncertainty::Known(0.000_000_001_5e-53),
};
//22
pub static ATOMIC_UNIT_OF_2ND_HYPERPOLARIZABILITY: Constant = Constant {
    name: "atomic unit of 2nd hyperpolarizability",
    symbol: None,
    value: 6.235_379_973_5e-65,
    units: Some("C^4 m^4 J^-3"),
    uncertainty: Uncertainty::Known(0.000_000_003_9e-65),
};
//23
pub static ATOMIC_UNIT_OF_ACTION: Constant = Constant {
    name: "atomic unit of action",
    symbol: None,
    value: 1.054_571_817e-34, //...
    units: Some("J s"),
    uncertainty: Uncertainty::Exact,
};
//24
pub static ATOMIC_UNIT_OF_CHARGE: Constant = Constant {
    name: "atomic unit of charge",
    symbol: None,
    value: 1.602_176_634e-19,
    units: Some("C"),
    uncertainty: Uncertainty::Exact,
};
//25
pub static ATOMIC_UNIT_OF_CHARGE_DENSITY: Constant = Constant {
    name: "atomic unit of charge density",
    symbol: None,
    value: 1.081_202_386_77e12,
    units: Some("C m^-3"),
    uncertainty: Uncertainty::Known(0.000_000_000_51e12),
};
//26
pub static ATOMIC_UNIT_OF_CURRENT: Constant = Constant {
    name: "atomic unit of current",
    symbol: None,
    value: 6.623_618_237_508_2e-3,
    units: Some("A"),
    uncertainty: Uncertainty::Known(0.000_000_000_007_2e-3),
};
//27
pub static ATOMIC_UNIT_OF_ELECTRIC_DIPOLE_MOMENT: Constant = Constant {
    name: "atomic unit of electric dipole moment",
    symbol: None,
    value: 8.478_353_619_8e-30,
    units: Some("C m"),
    uncertainty: Uncertainty::Known(0.000_000_001_3e-30),
};
//28
pub static ATOMIC_UNIT_OF_ELECTRIC_FIELD: Constant = Constant {
    name: "atomic unit of electric field",
    symbol: None,
    value: 5.142_206_751_12e11,
    units: Some("V m^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_80e11),
};
//29
pub static ATOMIC_UNIT_OF_ELECTRIC_FIELD_GRADIENT: Constant = Constant {
    name: "atomic unit of electric field gradient",
    symbol: None,
    value: 9.717_362_442_4e21,
    units: Some("V m^-2"),
    uncertainty: Uncertainty::Known(0.000_000_003e21),
};
//30
pub static ATOMIC_UNIT_OF_ELECTRIC_POLARIZABILITY: Constant = Constant {
    name: "atomic unit of electric polarizability",
    symbol: None,
    value: 1.648_777_272_12e-41,
    units: Some("C^2 m^2 J^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_51e-41),
};
//31
pub static ATOMIC_UNIT_OF_ELECTRIC_POTENTIAL: Constant = Constant {
    name: "atomic unit of electric potential",
    symbol: None,
    value: 27.211_386_245_981,
    units: Some("V"),
    uncertainty: Uncertainty::Known(0.000_000_000_03),
};
//32
pub static ATOMIC_UNIT_OF_ELECTRIC_QUADRUPOLE_MOMENT: Constant = Constant {
    name: "atomic unit of electric quadrupole moment",
    symbol: None,
    value: 4.486_551_518_5e-40,
    units: Some("C m^2"),
    uncertainty: Uncertainty::Known(0.000_000_001_4e-40),
};
//33
pub static ATOMIC_UNIT_OF_ENERGY: Constant = Constant {
    name: "atomic unit of energy",
    symbol: None,
    value: 4.359_744_722_206e-18,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_000_004_8e-18),
};
//34
pub static ATOMIC_UNIT_OF_FORCE: Constant = Constant {
    name: "atomic unit of force",
    symbol: None,
    value: 8.238_723_503_8e-8,
    units: Some("N"),
    uncertainty: Uncertainty::Known(0.000_000_001_3e-8),
};
//35
pub static ATOMIC_UNIT_OF_LENGTH: Constant = Constant {
    name: "atomic unit of length",
    symbol: None,
    value: 5.291_772_105_44e-11,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_000_82e-11),
};
//36
pub static ATOMIC_UNIT_OF_MAGNETIC_DIPOLE_MOMENT: Constant = Constant {
    name: "atomic unit of magnetic dipole moment",
    symbol: None,
    value: 1.854_802_013_15e-23,
    units: Some("J T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_58e-23),
};
//37
pub static ATOMIC_UNIT_OF_MAGNETIC_FLUX_DENSITY: Constant = Constant {
    name: "atomic unit of magnetic flux density",
    symbol: None,
    value: 2.350_517_570_77e5,
    units: Some("T"),
    uncertainty: Uncertainty::Known(0.000_000_000_73e5),
};
//38
pub static ATOMIC_UNIT_OF_MAGNETIZABILITY: Constant = Constant {
    name: "atomic unit of magnetizability",
    symbol: None,
    value: 7.891_036_579_4e-29,
    units: Some("J T^-2"),
    uncertainty: Uncertainty::Known(0.000_000_004_9e-29),
};
//39
pub static ATOMIC_UNIT_OF_MASS: Constant = Constant {
    name: "atomic unit of mass",
    symbol: None,
    value: 9.109_383_713_9e-31,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_000_002_8e-31),
};
//40
pub static ATOMIC_UNIT_OF_MOMENTUM: Constant = Constant {
    name: "atomic unit of momentum",
    symbol: None,
    value: 1.992_851_915_45e-24,
    units: Some("kg m s^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_31e-24),
};
//41
pub static ATOMIC_UNIT_OF_PERMITTIVITY: Constant = Constant {
    name: "atomic unit of permittivity",
    symbol: None,
    value: 1.112_650_056_20e-10,
    units: Some("F m^-1"), 
    uncertainty: Uncertainty::Known(0.000_000_000_17e-10),
};
//42
pub static ATOMIC_UNIT_OF_TIME: Constant = Constant {
    name: "atomic unit of time",
    symbol: None,
    value: 2.418_884_326_586_4e-17,
    units: Some("s"),
    uncertainty: Uncertainty::Known(0.000_000_000_002_6e-17),
};
//43
pub static ATOMIC_UNIT_OF_VELOCITY: Constant = Constant {
    name: "atomic unit of velocity",
    symbol: None,
    value: 2.187_691_262_16e6,
    units: Some("m s^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_34e6),
};
//44
pub static AVOGADRO_CONSTANT: Constant = Constant {
    name: "Avogadro constant", //
    symbol: Some(r"\(N_A\)"),
    value: 6.022_140_76e23,
    units: Some("mol^-1"),
    uncertainty: Uncertainty::Exact,
};
//45
pub static BOHR_MAGNETON: Constant = Constant {
    name: "Bohr magneton",
    symbol: None,
    value: 9.274_010_065_7e-24,
    units: Some("J T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_002_9e-24),
};
//46
pub static BOHR_MAGNETON_IN_ELECTRON_VOLT_PER_TESLA: Constant = Constant {
    name: "Bohr magneton in electron volt per tesla",
    symbol: None,
    value: 5.788_381_798_2e-5,
    units: Some("eV T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_001_8e-5),
};
//47
pub static BOHR_MAGNETON_IN_HZ_PER_TESLA: Constant = Constant {
    name: "Bohr magneton in Hz per tesla",
    symbol: None,
    value: 1.399_624_491_71e10,
    units: Some("Hz T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_44e10),
};
//48
pub static BOHR_MAGNETON_IN_INVERSE_METER_PER_TESLA: Constant = Constant {
    name: "Bohr magneton in inverse meter per tesla",
    symbol: None,
    value: 46.686_447_719,
    units: Some("m^-1 T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_015),
};
//49
pub static BOHR_MAGNETON_IN_KELVIN_PER_TESLA: Constant = Constant {
    name: "Bohr magneton in K per tesla",
    symbol: None,
    value: 0.671_713_814_72,
    units: Some("K T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_21),
};
//50
pub static BOHR_RADIUS: Constant = Constant {
    name: "Bohr radius",
    symbol: None,
    value: 5.291_772_105_44e-11,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_000_82e-11),
};
//51
pub static BOLTZMANN_CONSTANT: Constant = Constant {
    name: "Boltzmann constant",
    symbol: Some("k"),
    value: 1.380_649e-23,
    units: Some("J K^-1"),
    uncertainty: Uncertainty::Exact,
};
//52
pub static BOLTZMANN_CONSTANT_IN_ELECTRON_VOLT_PER_KELVIN: Constant = Constant {
    name: "Boltzmann constant in electron volt per kelvin",
    symbol: None,
    value: 8.617_333_262e-5, //...
    units: Some("eV K^-1"),
    uncertainty: Uncertainty::Exact,
};
//53
pub static BOLTZMANN_CONSTANT_IN_HZ_PER_K: Constant = Constant {
    name: "Boltzmann constant in Hz/K",
    symbol: None,
    value: 2.083_661_912e10, //...
    units: Some("Hz K^-1"),
    uncertainty: Uncertainty::Exact,
};
//54
pub static BOLTZMANN_CONSTANT_IN_INVERSE_METER_PER_K: Constant = Constant {
    name: "Boltzmann constant in inverse meter per kelvin",
    symbol: None,
    value: 69.503_480_04, //...
    units: Some("m^-1 K^-1"),
    uncertainty: Uncertainty::Exact,
};
//55
pub static CHARACTERISTIC_IMPEDANCE_OF_VACUUM: Constant = Constant {
    name: "characteristic impedance of vacuum",
    symbol: None,
    value: 376.730_313_412,
    units: Some("ohm"),
    uncertainty: Uncertainty::Known(0.000_000_059),
};
//56
pub static CLASSICAL_ELECTRON_RADIUS: Constant = Constant {
    name: "classical electron radius",
    symbol: None,
    value: 2.817_940_320_5e-15,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_001_3e-15),
};
//57
pub static COMPTON_WAVELENGTH: Constant = Constant {
    name: "Compton wavelength",
    symbol: None,
    value: 2.426_310_235_38e-12,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_000_76e-12),
};
//58
pub static CONDUCTANCE_QUANTUM: Constant = Constant {
    name: "conductance quantum",
    symbol: None,
    value: 7.748_091_729e-5, //...
    units: Some("S"),
    uncertainty: Uncertainty::Exact,
};
//59
pub static CONVENTIONAL_VALUE_OF_AMPERE_90: Constant = Constant {
    name: "conventional value of ampere-90",
    symbol: None,
    value: 1.000_000_088_87, //...
    units: Some("A"),
    uncertainty: Uncertainty::Exact,
};
//60
pub static CONVENTIONAL_VALUE_OF_COULOMB_90: Constant = Constant {
    name: "conventional value of coulomb-90",
    symbol: None,
    value: 1.000_000_088_87, //...
    units: Some("C"),
    uncertainty: Uncertainty::Exact,
};
//61
pub static CONVENTIONAL_VALUE_OF_FARAD_90: Constant = Constant {
    name: "conventional value of farad-90",
    symbol: None,
    value: 0.999_999_982_20, //...
    units: Some("F"),
    uncertainty: Uncertainty::Exact,
};
//62
pub static CONVENTIONAL_VALUE_OF_HENRY_90: Constant = Constant {
    name: "conventional value of henry-90",
    symbol: None,
    value: 1.000_000_017_79, //...
    units: Some("H"),
    uncertainty: Uncertainty::Exact,
};
//63
pub static CONVENTIONAL_VALUE_OF_JOSEPHSON_CONSTANT: Constant = Constant {
    name: "conventional value of Josephson constant",
    symbol: None,
    value: 483_597.9e9,
    units: Some("Hz V^-1"),
    uncertainty: Uncertainty::Exact,
};
//64
pub static CONVENTIONAL_VALUE_OF_OHM_90: Constant = Constant {
    name: "conventional value of ohm-90",
    symbol: None,
    value: 1.000_000_017_79, //...
    units: Some("ohm"),
    uncertainty: Uncertainty::Exact,
};
//65
pub static CONVENTIONAL_VALUE_OF_VOLT_90: Constant = Constant {
    name: "conventional value of volt-90",
    symbol: None,
    value: 1.000_000_106_66, //...
    units: Some("V"),
    uncertainty: Uncertainty::Exact,
};
//66
pub static CONVENTIONAL_VALUE_OF_VON_KLITZING_CONSTANT: Constant = Constant {
    name: "conventional value of von Klitzing constant",
    symbol: None,
    value: 25_812.807,
    units: Some("ohm"),
    uncertainty: Uncertainty::Exact,
};
//67
pub static CONVENTIONAL_VALUE_OF_WATT_90: Constant = Constant {
    name: "conventional value of watt-90",
    symbol: None,
    value: 1.000_000_195_53, //...
    units: Some("W"),
    uncertainty: Uncertainty::Exact,
};
//68
pub static COPPER_X_UNIT: Constant = Constant {
    name: "Copper x unit",
    symbol: None,
    value: 1.002_076_97e-13,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_28e-13),
};
//69
pub static DEUTERON_ELECTRON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "deuteron-electron magnetic moment ratio",
    symbol: None,
    value: -4.664_345_550e-4,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_012e-4),
};
//70
pub static DEUTERON_ELECTRON_MASS_RATIO: Constant = Constant {
    name: "deuteron-electron mass ratio",
    symbol: None,
    value: 3_670.482_967_655,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_063),
};
//71
pub static DEUTERON_G_FACTOR: Constant = Constant {
    name: "deuteron g factor",
    symbol: None,
    value: 0.857_438_233_5,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_002_2),
};
//72
pub static DEUTERON_MAGNETIC_MOMENT: Constant = Constant {
    name: "deuteron magnetic moment",
    symbol: None,
    value: 4.330_735_087e-27,
    units: Some("J T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_011e-27),
};
//73
pub static DEUTERON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "deuteron magnetic moment to Bohr magneton ratio",
    symbol: None,
    value: 4.669_754_568e-4,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_012e-4),
};
//74
pub static DEUTERON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "deuteron magnetic moment to nuclear magneton ratio",
    symbol: None,
    value: 0.857_438_233_5,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_002_2),
};
//75
pub static DEUTERON_MASS: Constant = Constant {
    name: "deuteron mass",
    symbol: None,
    value: 3.343_583_776_8e-27,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_000_001e-27),
};
//76
pub static DEUTERON_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "deuteron mass energy equivalent",
    symbol: None,
    value: 3.005_063_234_91e-10,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_000_94e-10),
};
//77
pub static DEUTERON_MASS_ENERGY_EQUIVALENT_IN_MEGA_ELECTRON_VOLT: Constant = Constant {
    name: "deuteron mass energy equivalent in mega electron volt",
    symbol: None,
    value: 1_875.612_945,
    units: Some("MeV"),
    uncertainty: Uncertainty::Known(0.000_000_58),
};
//78
pub static DEUTERON_MASS_IN_U: Constant = Constant {
    name: "deuteron mass in u",
    symbol: None,
    value: 2.013_553_212_544,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_000_015),
};
//79
pub static DEUTERON_MOLAR_MASS: Constant = Constant {
    name: "deuteron molar mass",
    symbol: None,
    value: 2.013_553_214_66e-3,
    units: Some("kg mol^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_63e-3),
};
//80
pub static DEUTERON_NEUTRON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "deuteron-neutron magnetic moment ratio",
    symbol: None,
    value: -0.448_206_52,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_11),
};
//81
pub static DEUTERON_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "deuteron-proton magnetic moment ratio",
    symbol: None,
    value: 0.307_012_209_3,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_79),
};
//82
pub static DEUTERON_PROTON_MASS_RATIO: Constant = Constant {
    name: "deuteron-proton mass ratio",
    symbol: None,
    value: 1.999_007_501_269_9,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_008_4),
};
//83     
pub static DEUTERON_RELATIVE_ATOMIC_MASS: Constant = Constant {
    name: "deuteron relative atomic mass",
    symbol: None,
    value: 2.013_553_212_544,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_015),
};
//84
pub static DEUTERON_RMS_CHARGE_RADIUS: Constant = Constant {
    name: "deuteron rms charge radius",
    symbol: None,
    value: 2.127_78e-15,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_27e-15),
};
//85
pub static ELECTRON_CHARGE_TO_MASS_QUOTIENT: Constant = Constant {
    name: "electron charge to mass quotient",
    symbol: None,
    value: -1.758_820_008_38e11,
    units: Some("C kg^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_55e11),
};
//86
pub static ELECTRON_DEUTERON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "electron-deuteron magnetic moment ratio",
    symbol: None,
    value: -2_143.923_492_1,
    units: None,
    uncertainty: Uncertainty::Known(0.000_005_6),
};
//87
pub static ELECTRON_DEUTERON_MASS_RATIO: Constant = Constant {
    name: "electron-deuteron mass ratio",
    symbol: None,
    value: 2.724_437_107_629e-4,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_047e-4),
};
//88
pub static ELECTRON_G_FACTOR: Constant = Constant {
    name: "electron g factor",
    symbol: None,
    value: -2.002_319_304_360_92,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_000_36),
};
//89
pub static ELECTRON_GYROMAGNETIC_RATIO: Constant = Constant {
    name: "electron gyromagnetic ratio",
    symbol: None,
    value: 1.760_859_627_84e11,
    units: Some("s^-1 T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_55e11),
};
//90
pub static ELECTRON_GYROMAGNETIC_RATIO_IN_MEGAHERTZ_PER_TESLA: Constant = Constant {
    name: "electron gyromagnetic ratio in megahertz per tesla",
    symbol: None,
    value: 28_024.951_386_1,
    units: Some("MHz T^-1"),
    uncertainty: Uncertainty::Known(0.000_008_7),
};
//91
pub static ELECTRON_HELION_MASS_RATIO: Constant = Constant {
    name: "electron-helion mass ratio",
    symbol: None,
    value: 1.819_543_074_649e-4,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_053e-4),
};
//92
pub static ELECTRON_MAGNETIC_MOMENT: Constant = Constant {
    name: "electron magnetic moment",
    symbol: None,
    value: -9.284_764_691_7e-24,
    units: Some("J T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_002_9e-24),
};
//93
pub static ELECTRON_MAGNETIC_MOMENT_ANOMALY: Constant = Constant {
    name: "electron magnetic moment anomaly",
    symbol: None,
    value: 1.159_652_180_46e-3,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_18e-3),
};
//94     
pub static ELECTRON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "electron magnetic moment to Bohr magneton ratio",
    symbol: None,
    value: -1.001_159_652_180_46,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_000_18),
};
//95
pub static ELECTRON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "electron magnetic moment to nuclear magneton ratio",
    symbol: None,
    value: -1_838.281_971_877,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_032),
};
//96
pub static ELECTRON_MASS: Constant = Constant {
    name: "electron mass",
    symbol: None,
    value: 9.109_383_713_9e-31,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_000_002_8e-31),
};
//97
pub static ELECTRON_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "electron mass energy equivalent",
    symbol: None,
    value: 8.187_105_788e-14,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_002_6e-14),
};
//98
pub static ELECTRON_MASS_ENERGY_EQUIVALENT_IN_MEV: Constant = Constant {
    name: "electron mass energy equivalent in MeV",
    symbol: None,
    value: 0.510_998_950_69,
    units: Some("MeV"),
    uncertainty: Uncertainty::Known(0.000_000_000_16),
};
//99
pub static ELECTRON_MASS_IN_U: Constant = Constant {
    name: "electron mass in u",
    symbol: None,
    value: 5.485_799_090_441e-4,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_000_097e-4),
};
//100
pub static ELECTRON_MOLAR_MASS: Constant = Constant {
    name: "electron molar mass",
    symbol: None,
    value: 5.485_799_096_2e-7,
    units: Some("kg mol^-1"),
    uncertainty: Uncertainty::Known(0.000_000_001_7e-7),
};
//101
pub static ELECTRON_MUON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "electron-muon magnetic moment ratio",
    symbol: None,
    value: 206.766_988_1,
    units: None,
    uncertainty: Uncertainty::Known(0.000_004_6),
};
//102
pub static ELECTRON_MUON_MASS_RATIO: Constant = Constant {
    name: "electron-muon mass ratio",
    symbol: None,
    value: 4.836_331_70e-3,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_11e-3)
};
//103
pub static ELECTRON_NEUTRON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "electron-neutron magnetic moment ratio",
    symbol: None,
    value: 960.920_48,
    units: None,
    uncertainty: Uncertainty::Known(0.000_23),
};
//104
pub static ELECTRON_NEUTRON_MASS_RATIO: Constant = Constant {
    name: "electron-neutron mass ratio",
    symbol: None,
    value: 5.438_673_441_6e-4,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_002_2e-4),
};
//105
pub static ELECTRON_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "electron-proton magnetic moment ratio",
    symbol: None,
    value: -658.210_687_89,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_19),
};
//106
pub static ELECTRON_PROTON_MASS_RATIO: Constant = Constant {
    name: "electron-proton mass ratio",
    symbol: None,
    value: 5.446_170_214_889e-4,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_094e-4),
};
//107  
pub static ELECTRON_RELATIVE_ATOMIC_MASS: Constant = Constant {
    name: "electron relative atomic mass",
    symbol: None,
    value: 5.485_799_090_441e-4,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_097e-4),
};
//108
pub static ELECTRON_TAU_MASS_RATIO: Constant = Constant {
    name: "electron-tau mass ratio",
    symbol: None,
    value: 2.875_85e-4,
    units: None,
    uncertainty: Uncertainty::Known(0.000_19e-4),
};
//109
pub static ELECTRON_TO_ALPHA_PARTICLE_MASS_RATIO: Constant = Constant {
    name: "electron to alpha particle mass ratio",
    symbol: None,
    value: 1.370_933_554_733e-4,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_032e-4),
};
//110
pub static ELECTRON_TO_SHIELDED_HELION_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "electron to shielded helion magnetic moment ratio",
    symbol: None,
    value: 864.058_239_86,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_7),
};
//111
pub static ELECTRON_TO_SHIELDED_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "electron to shielded proton magnetic moment ratio",
    symbol: None,
    value: -658.227_585_6,
    units: None,
    uncertainty: Uncertainty::Known(0.000_002_7),
};
//112
pub static ELECTRON_TRITON_MASS_RATIO: Constant = Constant {
    name: "electron-triton mass ratio",
    symbol: None,
    value: 1.819_200_062_327e-4,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_068e-4),
};
//113
pub static ELECTRON_VOLT: Constant = Constant {
    name: "electron volt",
    symbol: None,
    value: 1.602_176_634e-19,
    units: Some("J"),
    uncertainty: Uncertainty::Exact,
};
//114
pub static ELECTRON_VOLT_ATOMIC_MASS_UNIT_RELATIONSHIP: Constant = Constant {
    name: "electron volt-atomic mass unit relationship",
    symbol: None,
    value: 1.073_544_100_83e-9,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_000_33e-9),
};
//115
pub static ELECTRON_VOLT_HARTREE_RELATIONSHIP: Constant = Constant {
    name: "electron volt-hartree relationship",
    symbol: None,
    value: 3.674_932_217_566_5e-2,
    units: Some("E_h"),
    uncertainty: Uncertainty::Known(0.000_000_000_004e-2),
};
//116
pub static ELECTRON_VOLT_HERTZ_RELATIONSHIP: Constant = Constant {
    name: "electron volt-hertz relationship",
    symbol: None,
    value: 2.417_989_242e14, //...
    units: Some("Hz"),
    uncertainty: Uncertainty::Exact,
};
//117
pub static ELECTRON_VOLT_INVERSE_METER_RELATIONSHIP: Constant = Constant {
    name: "electron volt-inverse meter relationship",
    symbol: None,
    value: 8.065_543_937e5, //...
    units: Some("m^-1"),
    uncertainty: Uncertainty::Exact,
};
//118
pub static ELECTRON_VOLT_JOULE_RELATIONSHIP: Constant = Constant {
    name: "electron volt-joule relationship",
    symbol: None,
    value: 1.602_176_634e-19,
    units: Some("J"),
    uncertainty: Uncertainty::Exact,
};
//119
pub static ELECTRON_VOLT_KELVIN_RELATIONSHIP: Constant = Constant {
    name: "electron volt-kelvin relationship",
    symbol: None,
    value: 1.160_451_812e4, //...
    units: Some("K"),
    uncertainty: Uncertainty::Exact,
};
//120
pub static ELECTRON_VOLT_KILOGRAM_RELATIONSHIP: Constant = Constant {
    name: "electron volt-kilogram relationship",
    symbol: None,
    value: 1.782_661_921e-36, //...
    units: Some("kg"),
    uncertainty: Uncertainty::Exact,
};
//121
pub static ELEMENTARY_CHARGE: Constant = Constant {
    name: "elementary charge",
    symbol: Some("e"),
    value: 1.602_176_634e-19,
    units: Some("C"),
    uncertainty: Uncertainty::Exact,
};
//122
pub static ELEMENTARY_CHARGE_OVER_H_BAR: Constant = Constant {
    name: "elementary charge over h-bar",
    symbol: None,
    value: 1.519_267_447e15, //...
    units: Some("A J^-1"),
    uncertainty: Uncertainty::Exact,
};
//123
pub static FARADAY_CONSTANT: Constant = Constant {
    name: "Faraday constant",
    symbol: None,
    value: 96_485.332_12, //...
    units: Some("C mol^-1"),
    uncertainty: Uncertainty::Exact,
};
//124
pub static FERMI_COUPLING_CONSTANT: Constant = Constant {
    name: "Fermi coupling constant",
    symbol: None,
    value: 1.166_378_7e-5,
    units: Some("GeV^-2"),
    uncertainty: Uncertainty::Known(0.000_000_6e-5),
};
//125
pub static FINE_STRUCTURE_CONSTANT: Constant = Constant {
    name: "fine-structure constant",
    symbol: None,
    value: 7.297_352_564_3e-3,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_001_1e-3),
};
//126
pub static FIRST_RADIATION_CONSTANT: Constant = Constant {
    name: "first radiation constant",
    symbol: None,
    value: 3.741_771_852e-16, //...
    units: Some("W m^2"),
    uncertainty: Uncertainty::Exact,
};
//127
pub static FIRST_RADIATION_CONSTANT_FOR_SPECTRAL_RADIANCE: Constant = Constant {
    name: "first radiation constant for spectral radiance",
    symbol: None,
    value: 1.191_042_972e-16, //...
    units: Some("W m^2 sr^-1"),
    uncertainty: Uncertainty::Exact,
};
//128
pub static HARTREE_ATOMIC_MASS_UNIT_RELATIONSHIP: Constant = Constant {
    name: "hartree-atomic mass unit relationship",
    symbol: None,
    value: 2.921_262_317_97e-8,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_000_91e-8),
};
//129
pub static HARTREE_ELECTRON_VOLT_RELATIONSHIP: Constant = Constant {
    name: "hartree-electron volt relationship",
    symbol: None,
    value: 27.211_386_245_981,
    units: Some("eV"),
    uncertainty: Uncertainty::Known(0.000_000_000_03),
};
//130
pub static HARTREE_ENERGY: Constant = Constant {
    name: "Hartree energy",
    symbol: None,
    value: 4.359_744_722_206e-18,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_000_004_8e-18),
};
//131
pub static HARTREE_ENERGY_IN_EV: Constant = Constant {
    name: "Hartree energy in eV",
    symbol: None,
    value: 27.211_386_245_981,
    units: Some("eV"),
    uncertainty: Uncertainty::Known(0.000_000_000_03),
};
//132
pub static HARTREE_HERTZ_RELATIONSHIP: Constant = Constant {
    name: "hartree-hertz relationship",
    symbol: None,
    value: 6.579_683_920_499_9e15,
    units: Some("Hz"),
    uncertainty: Uncertainty::Known(0.000_000_000_007_2e15),
};
//133
pub static HARTREE_INVERSE_METER_RELATIONSHIP: Constant = Constant {
    name: "hartree-inverse meter relationship",
    symbol: None,
    value: 2.194_746_313_631_4e7,
    units: Some("m^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_002_4e7),
};
//134
pub static HARTREE_JOULE_RELATIONSHIP: Constant = Constant {
    name: "hartree-joule relationship",
    symbol: None,
    value: 4.359_744_722_206e-18,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_000_004_8e-18),
};
//135
pub static HARTREE_KELVIN_RELATIONSHIP: Constant = Constant {
    name: "hartree-kelvin relationship",
    symbol: None,
    value: 3.157_750_248_039_8e5,
    units: Some("K"),
    uncertainty: Uncertainty::Known(0.000_000_000_003_4e5),
};
//136
pub static HARTREE_KILOGRAM_RELATIONSHIP: Constant = Constant {
    name: "hartree-kilogram relationship",
    symbol: None,
    value: 4.850_870_209_541_9e-35,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_000_000_005_3e-35),
};
//137
pub static HELION_ELECTRON_MASS_RATIO: Constant = Constant {
    name: "helion-electron mass ratio",
    symbol: None,
    value: 5_495.885_279_84,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_16),
};
//138
pub static HELION_G_FACTOR: Constant = Constant {
    name: "helion g factor",
    symbol: None,
    value: -4.255_250_699_5,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_003_4),
};
//139
pub static HELION_MAGNETIC_MOMENT: Constant = Constant {
    name: "helion magnetic moment",
    symbol: None,
    value: -1.074_617_551_98e-26,
    units: Some("J T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_93e-26),
};
//140
pub static HELION_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "helion magnetic moment to Bohr magneton ratio",
    symbol: None,
    value: -1.158_740_980_83e-3,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_94e-3),
};
//141
pub static HELION_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "helion magnetic moment to nuclear magneton ratio",
    symbol: None,
    value: -2.127_625_349_8,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_001_7),
};
//142
pub static HELION_MASS: Constant = Constant {
    name: "helion mass",
    symbol: None,
    value: 5.006_412_786_2e-27,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_000_001_6e-27),
};
//143
pub static HELION_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "helion mass energy equivalent",
    symbol: None,
    value: 4.499_539_418_5e-10,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_001_4e-10),
};
//144
pub static HELION_MASS_ENERGY_EQUIVALENT_IN_MEV: Constant = Constant {
    name: "helion mass energy equivalent in MeV",
    symbol: None,
    value: 2_808.391_611_12,
    units: Some("MeV"),
    uncertainty: Uncertainty::Known(0.000_000_88),
};
//145
pub static HELION_MASS_IN_U: Constant = Constant {
    name: "helion mass in u",
    symbol: None,
    value: 3.014_932_246_932,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_000_074),
};
//146
pub static HELION_MOLAR_MASS: Constant = Constant {
    name: "helion molar mass",
    symbol: None,
    value: 3.014_932_250_1e-3,
    units: Some("kg mol^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_94e-3),
};
//147
pub static HELION_PROTON_MASS_RATIO: Constant = Constant {
    name: "helion-proton mass ratio",
    symbol: None,
    value: 2.993_152_671_552,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_07),
};
//148
pub static HELION_RELATIVE_ATOMIC_MASS: Constant = Constant {
    name: "helion relative atomic mass",
    symbol: None,
    value: 3.014_932_246_932,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_074),
};
//149
pub static HELION_SHIELDING_SHIFT: Constant = Constant {
    name: "helion shielding shift",
    symbol: None,
    value: 5.996_702_9e-5,
    units: None,
    uncertainty: Uncertainty::Known(0.000_002_3e-5),
};
//150
pub static HERTZ_ATOMIC_MASS_UNIT_RELATIONSHIP: Constant = Constant {
    name: "hertz-atomic mass unit relationship",
    symbol: None,
    value: 4.439_821_659e-24,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_001_4e-24),
};
//151
pub static HERTZ_ELECTRON_VOLT_RELATIONSHIP: Constant = Constant {
    name: "hertz-electron volt relationship",
    symbol: None,
    value: 4.135_667_696e-15,
    units: Some("eV"),
    uncertainty: Uncertainty::Exact,
};
//152
pub static HERTZ_HARTREE_RELATIONSHIP: Constant = Constant {
    name: "hertz-hartree relationship",
    symbol: None,
    value: 1.519_829_846_057_4e-16,
    units: Some("E_h"),
    uncertainty: Uncertainty::Known(0.000_000_000_001_7e-16),
};
//153
pub static HERTZ_INVERSE_METER_RELATIONSHIP: Constant = Constant {
    name: "hertz-inverse meter relationship",
    symbol: None,
    value: 3.335_640_951e-9,
    units: Some("m^-1"),
    uncertainty: Uncertainty::Exact,
};
//154
pub static HERTZ_JOULE_RELATIONSHIP: Constant = Constant {
    name: "hertz-joule relationship",
    symbol: None,
    value: 6.626_070_15e-34,
    units: Some("J"),
    uncertainty: Uncertainty::Exact,
};
//155
pub static HERTZ_KELVIN_RELATIONSHIP: Constant = Constant {
    name: "hertz-kelvin relationship",
    symbol: None,
    value: 4.799_243_073e-11, //...
    units: Some("K"),
    uncertainty: Uncertainty::Exact,
};
//156
pub static HERTZ_KILOGRAM_RELATIONSHIP: Constant = Constant {
    name: "hertz-kilogram relationship",
    symbol: None,
    value: 7.372_497_323e-51,
    units: Some("kg"),
    uncertainty: Uncertainty::Exact,
};
//157
pub static HYPERFINE_TRANSITION_FREQUENCY_OF_CS_133: Constant = Constant {
    name: "hyperfine transition frequency of Cs-133",
    symbol: None,
    value: 9_192_631_770.0,
    units: Some("Hz"),
    uncertainty: Uncertainty::Exact,
};
//158
pub static INVERSE_FINE_STRUCTURE_CONSTANT: Constant = Constant {
    name: "inverse fine-structure constant",
    symbol: None,
    value: 137.035_999_177,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_021),
};
//159
pub static INVERSE_METER_ATOMIC_MASS_UNIT_RELATIONSHIP: Constant = Constant {
    name: "inverse meter-atomic mass unit relationship",
    symbol: None,
    value: 1.331_025_048_24e-15,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_000_41e-15),
};
//160
pub static INVERSE_METER_ELECTRON_VOLT_RELATIONSHIP: Constant = Constant {
    name: "inverse meter-electron volt relationship",
    symbol: None,
    value: 1.239_841_984e-6, //...
    units: Some("eV"),
    uncertainty: Uncertainty::Exact,
};
//161
pub static INVERSE_METER_HARTREE_RELATIONSHIP: Constant = Constant {
    name: "inverse meter-hartree relationship",
    symbol: None,
    value: 4.556_335_252_913_2e-8, //...
    units: Some("E_h"),
    uncertainty: Uncertainty::Known(0.000_000_000_005e-8),
};
//162
pub static INVERSE_METER_HERTZ_RELATIONSHIP: Constant = Constant {
    name: "inverse meter-hertz relationship",
    symbol: None,
    value: 299_792_458.0,
    units: Some("Hz"),
    uncertainty: Uncertainty::Exact,
};
//163
pub static INVERSE_METER_JOULE_RELATIONSHIP: Constant = Constant {
    name: "inverse meter-joule relationship",
    symbol: None,
    value: 1.986_445_857e-25, //...
    units: Some("J"),
    uncertainty: Uncertainty::Exact,
};
//164
pub static INVERSE_METER_KELVIN_RELATIONSHIP: Constant = Constant {
    name: "inverse meter-kelvin relationship",
    symbol: None,
    value: 1.438_776_877e-2, //...
    units: Some("K"),
    uncertainty: Uncertainty::Exact,
};
//165
pub static INVERSE_METER_KILOGRAM_RELATIONSHIP: Constant = Constant {
    name: "inverse meter-kilogram relationship",
    symbol: None,
    value: 2.210_219_094e-42,
    units: Some("kg"),
    uncertainty: Uncertainty::Exact,
};
//166
pub static INVERSE_OF_CONDUCTANCE_QUANTUM: Constant = Constant {
    name: "inverse of conductance quantum",
    symbol: None,
    value: 12_906.403_72, //...
    units: Some("ohm"),
    uncertainty: Uncertainty::Exact,
};
//167
pub static JOSEPHSON_CONSTANT: Constant = Constant {
    name: "Josephson constant",
    symbol: None,
    value: 483_597.848_4e9, //...
    units: Some("Hz V^-1"),
    uncertainty: Uncertainty::Exact,
};
//168
pub static JOULE_ATOMIC_MASS_UNIT_RELATIONSHIP: Constant = Constant {
    name: "joule-atomic mass unit relationship",
    symbol: None,
    value: 6.700_535_247_1e9,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_002_1e9),
};
//169
pub static JOULE_ELECTRON_VOLT_RELATIONSHIP: Constant = Constant {
    name: "joule-electron volt relationship",
    symbol: None,
    value: 6.241_509_074e18, //..
    units: Some("eV"),
    uncertainty: Uncertainty::Exact,
};
//170
pub static JOULE_HARTREE_RELATIONSHIP: Constant = Constant {
    name: "joule-hartree relationship",
    symbol: None,
    value: 2.293_712_278_396_9e17,
    units: Some("E_h"),
    uncertainty: Uncertainty::Known(0.000_000_000_002_5e17),
};
//171
pub static JOULE_HERTZ_RELATIONSHIP: Constant = Constant {
    name: "joule-hertz relationship",
    symbol: None,
    value: 1.509_190_179e33, //..
    units: Some("Hz"),
    uncertainty: Uncertainty::Exact,
};
//172
pub static JOULE_INVERSE_METER_RELATIONSHIP: Constant = Constant {
    name: "joule-inverse meter relationship",
    symbol: None,
    value: 5.034_116_567e24, //...
    units: Some("m^-1"),
    uncertainty: Uncertainty::Exact,
};
//173
pub static JOULE_KELVIN_RELATIONSHIP: Constant = Constant {
    name: "joule-kelvin relationship",
    symbol: None,
    value: 7.242_970_516e22, //...
    units: Some("K"),
    uncertainty: Uncertainty::Exact,
};
//174
pub static JOULE_KILOGRAM_RELATIONSHIP: Constant = Constant {
    name: "joule-kilogram relationship",
    symbol: None,
    value: 1.112_650_056e-17,
    units: Some("kg"),
    uncertainty: Uncertainty::Exact,
};
//175
pub static KELVIN_ATOMIC_MASS_UNIT_RELATIONSHIP: Constant = Constant {
    name: "kelvin-atomic mass unit relationship",
    symbol: None,
    value: 9.251_087_288_4e-14,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_002_9e-14),
};
//176
pub static KELVIN_ELECTRON_VOLT_RELATIONSHIP: Constant = Constant {
    name: "kelvin-electron volt relationship",
    symbol: None,
    value: 8.617_333_262e-5,
    units: Some("eV"),
    uncertainty: Uncertainty::Exact,
};
//177
pub static KELVIN_HARTREE_RELATIONSHIP: Constant = Constant {
    name: "kelvin-hartree relationship",
    symbol: None,
    value: 3.166_811_563_456_4e-6,
    units: Some("E_h"),
    uncertainty: Uncertainty::Known(0.000_000_000_003_5e-6),
};
//178
pub static KELVIN_HERTZ_RELATIONSHIP: Constant = Constant {
    name: "kelvin-hertz relationship",
    symbol: None,
    value: 2.083_661_912e10, //...
    units: Some("Hz"),
    uncertainty: Uncertainty::Exact,
};
//179
pub static KELVIN_INVERSE_METER_RELATIONSHIP: Constant = Constant {
    name: "kelvin-inverse meter relationship",
    symbol: None,
    value: 69.503_480_04, //...
    units: Some("m^-1"),
    uncertainty: Uncertainty::Exact,
};
//180
pub static KELVIN_JOULE_RELATIONSHIP: Constant = Constant {
    name: "kelvin-joule relationship",
    symbol: None,
    value: 1.380_649e-23,
    units: Some("J"),
    uncertainty: Uncertainty::Exact,
};
//181
pub static KELVIN_KILOGRAM_RELATIONSHIP: Constant = Constant {
    name: "kelvin-kilogram relationship",
    symbol: None,
    value: 1.536_179_187e-40, //...
    units: Some("kg"),
    uncertainty: Uncertainty::Exact,
};
//182
pub static KILOGRAM_ATOMIC_MASS_UNIT_RELATIONSHIP: Constant = Constant {
    name: "kilogram-atomic mass unit relationship",
    symbol: None,
    value: 6.022_140_753_7e26,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_001_9e26),
};
//183
pub static KILOGRAM_ELECTRON_VOLT_RELATIONSHIP: Constant = Constant {
    name: "kilogram-electron volt relationship",
    symbol: None,
    value: 5.609_588_603e35, //...
    units: Some("eV"),
    uncertainty: Uncertainty::Exact,
};
//184
pub static KILOGRAM_HARTREE_RELATIONSHIP: Constant = Constant {
    name: "kilogram-hartree relationship",
    symbol: None,
    value: 2.061_485_788_741_5e34,
    units: Some("E_h"),
    uncertainty: Uncertainty::Known(0.000_000_000_002_2e34),
};
//185
pub static KILOGRAM_HERTZ_RELATIONSHIP: Constant = Constant {
    name: "kilogram-hertz relationship",
    symbol: None,
    value: 1.356_392_489e50, //...
    units: Some("Hz"),
    uncertainty: Uncertainty::Exact,
};
//186
pub static KILOGRAM_INVERSE_METER_RELATIONSHIP: Constant = Constant {
    name: "kilogram-inverse meter relationship",
    symbol: None,
    value: 4.524_438_335e41, //...
    units: Some("m^-1"),
    uncertainty: Uncertainty::Exact,
};
//187
pub static KILOGRAM_JOULE_RELATIONSHIP: Constant = Constant {
    name: "kilogram-joule relationship",
    symbol: None,
    value: 8.987_551_787e16, //...
    units: Some("J"),
    uncertainty: Uncertainty::Exact,
};
//188
pub static KILOGRAM_KELVIN_RELATIONSHIP: Constant = Constant {
    name: "kilogram-kelvin relationship",
    symbol: None,
    value: 6.509_657_26e39, //...
    units: Some("K"),
    uncertainty: Uncertainty::Exact,
};
//189
pub static LATTICE_PARAMETER_OF_SILICON: Constant = Constant {
    name: "lattice parameter of silicon",
    symbol: None,
    value: 5.431_020_511e-10,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_089e-10),
};
//190
pub static LATTICE_SPACING_OF_IDEAL_SI_220: Constant = Constant {
    name: "lattice spacing of ideal Si (220)",
    symbol: None,
    value: 1.920_155_716e-10,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_032e-10),
};
//191
pub static LOSCHMIDT_CONSTANT_273_15_K_100_KPA: Constant = Constant {
    name: "Loschmidt constant (273.15 K, 100 kPa)",
    symbol: None,
    value: 2.651_645_804e25, //...
    units: Some("m^-3"),
    uncertainty: Uncertainty::Exact,
};
//192
pub static LOSCHMIDT_CONSTANT_273_15_K_101_325_KPA: Constant = Constant {
    name: "Loschmidt constant (273.15 K, 101.325 kPa)",
    symbol: None,
    value: 2.686_780_111e25, //...
    units: Some("m^-3"),
    uncertainty: Uncertainty::Exact,
};
//193
pub static LUMINOUS_EFFICACY: Constant = Constant {
    name: "luminous efficacy",
    symbol: None,
    value: 683.0,
    units: Some("lm W^-1"),
    uncertainty: Uncertainty::Exact,
};
//194
pub static MAGNETIC_FLUX_QUANTUM: Constant = Constant {
    name: "magnetic flux quantum",
    symbol: None,
    value: 2.067_833_848e-15, //...
    units: Some("Wb"),
    uncertainty: Uncertainty::Exact,
};
//195
pub static MOLAR_GAS_CONSTANT: Constant = Constant {
    name: "molar gas constant",
    symbol: None,
    value: 8.314_462_618, //...
    units: Some("J mol^-1 K^-1"),
    uncertainty: Uncertainty::Exact,
};
//196
pub static MOLAR_MASS_CONSTANT: Constant = Constant {
    name: "molar mass constant",
    symbol: None,
    value: 1.000_000_001_05e-3,
    units: Some("kg mol^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_31e-3),
};
//197
pub static MOLAR_MASS_OF_CARBON_12: Constant = Constant {
    name: "molar mass of carbon-12",
    symbol: None,
    value: 12.000_000_012_6e-3,
    units: Some("kg mol^-1"),
    uncertainty: Uncertainty::Known(0.000_000_003_7e-3),
};
//198
pub static MOLAR_PLANCK_CONSTANT: Constant = Constant {
    name: "molar Planck constant",
    symbol: None,
    value: 3.990_312_712e-10, //...
    units: Some("J Hz^-1 mol^-1"),
    uncertainty: Uncertainty::Exact,
};
//199
pub static MOLAR_VOLUME_IDEAL_GAS_273_15_K_100_KPA: Constant = Constant {
    name: "molar volume of ideal gas (273.15 K, 100 kPa)",
    symbol: None,
    value: 22.710_954_64e-3,
    units: Some("m^3 mol^-1"),
    uncertainty: Uncertainty::Exact,
};
//200
pub static MOLAR_VOLUME_IDEAL_GAS_273_15_K_101_325_KPA: Constant = Constant { // note: how to represent . in identifier
    name: "molar volume of ideal gas (273.15 K, 101.325 kPa)",
    symbol: None,
    value: 22.413_969_54e-3,
    units: Some("m^3 mol^-1"),
    uncertainty: Uncertainty::Exact,
};
//201
pub static MOLAR_VOLUME_OF_SILICON: Constant = Constant {
    name: "molar volume of silicon",
    symbol: None,
    value: 1.205_883_199e-5,
    units: Some("m^3 mol^-1"),
    uncertainty: Uncertainty::Known(0.000_000_06e-5),
};
//202
pub static MOLYBDENUM_X_UNIT: Constant = Constant {
    name: "Molybdenum x unit",
    symbol: None,
    value: 1.002_099_52e-13,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_53e-13),
};
//203
pub static MUON_COMPTON_WAVELENGTH: Constant = Constant {
    name: "muon Compton wavelength",
    symbol: None,
    value: 1.173_444_110e-14,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_026e-14),
};
//204 
pub static MUON_ELECTRON_MASS_RATIO: Constant = Constant {
    name: "muon-electron mass ratio",
    symbol: None,
    value: 206.768_282_7,
    units: None,
    uncertainty: Uncertainty::Known(0.000_004_6),
};
//205
pub static MUON_G_FACTOR: Constant = Constant {
    name: "muon g factor",
    symbol: None,
    value: -2.002_331_841_23,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_82),
};
//206
pub static MUON_MAGNETIC_MOMENT: Constant = Constant {
    name: "muon magnetic moment",
    symbol: None,
    value: -4.490_448_3e-26,
    units: Some("J T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_1e-26),
};
//207
pub static MUON_MAGNETIC_MOMENT_ANOMALY: Constant = Constant {
    name: "muon magnetic moment anomaly",
    symbol: None,
    value: 1.165_920_62e-3,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_41e-3),
};
//208
pub static MUON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "muon magnetic moment to Bohr magneton ratio",
    symbol: None,
    value: -4.841_970_48e-3,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_11e-3),
};
//209
pub static MUON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "muon magnetic moment to nuclear magneton ratio",
    symbol: None,
    value: -8.890_597_04,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_2),
};
//210
pub static MUON_MASS: Constant = Constant {
    name: "muon mass",
    symbol: None,
    value: 1.883_531_627e-28,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_000_042e-28),
};
//211
pub static MUON_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "muon mass energy equivalent",
    symbol: None,
    value: 1.692_833_804e-11,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_038e-11),
};
//212
pub static MUON_MASS_ENERGY_EQUIVALENT_IN_MEV: Constant = Constant {
    name: "muon mass energy equivalent in MeV",
    symbol: None,
    value: 105.658_375_5,
    units: Some("MeV"),
    uncertainty: Uncertainty::Known(0.000_002_3),
};
//213
pub static MUON_MASS_IN_U: Constant = Constant {
    name: "muon mass in u",
    symbol: None,
    value: 0.113_428_925_7,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_002_5),
};
//214
pub static MUON_MOLAR_MASS: Constant = Constant {
    name: "muon molar mass",
    symbol: None,
    value: 1.134_289_258e-4,
    units: Some("kg mol^-1"),
    uncertainty: Uncertainty::Known(0.000_000_025e-4),
};
//215
pub static MUON_NEUTRON_MASS_RATIO: Constant = Constant {
    name: "muon-neutron mass ratio",
    symbol: None,
    value: 0.112_454_516_8,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_002_5),
};
//216
pub static MUON_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "muon-proton magnetic moment ratio",
    symbol: None,
    value: -3.183_345_146,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_071),
};
//217
pub static MUON_PROTON_MASS_RATIO: Constant = Constant {
    name: "muon-proton mass ratio",
    symbol: None,
    value: 0.112_609_526_2,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_002_5),
};
//218
pub static MUON_TAU_MASS_RATIO: Constant = Constant {
    name: "muon-tau mass ratio",
    symbol: None,
    value: 5.946_35e-2,
    units: None,
    uncertainty: Uncertainty::Known(0.000_4e-2),
};
//219
pub static NATURAL_UNIT_OF_ACTION: Constant = Constant {
    name: "natural unit of action",
    symbol: None,
    value: 1.054_571_817e-34, //...
    units: Some("J s"),
    uncertainty: Uncertainty::Exact,
};
//220
pub static NATURAL_UNIT_OF_ACTION_IN_EV_S: Constant = Constant {
    name: "natural unit of action in eV s",
    symbol: None,
    value: 6.582_119_569e-16,
    units: Some("eV s"),
    uncertainty: Uncertainty::Exact,
};
//221
pub static NATURAL_UNIT_OF_ENERGY: Constant = Constant {
    name: "natural unit of energy",
    symbol: None,
    value: 8.187_105_788e-14,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_002_6e-14),
};
//222 
pub static NATURAL_UNIT_OF_ENERGY_IN_MEV: Constant = Constant {
    name: "natural unit of energy in MeV",
    symbol: None,
    value: 0.510_998_950_69,
    units: Some("MeV"),
    uncertainty: Uncertainty::Known(0.000_000_000_16),
};
//223
pub static NATURAL_UNIT_OF_LENGTH: Constant = Constant {
    name: "natural unit of length",
    symbol: None,
    value: 3.861_592_674_4e-13,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_001_2e-13),
};
//224
pub static NATURAL_UNIT_OF_MASS: Constant = Constant {
    name: "natural unit of mass",
    symbol: None,
    value: 9.109_383_713_9e-31,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_000_002_8e-31),
};
//225
pub static NATURAL_UNIT_OF_MOMENTUM: Constant = Constant {
    name: "natural unit of momentum",
    symbol: None,
    value: 2.730_924_534_46e-22,
    units: Some("kg m s^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_85e-22),
};
//226
pub static NATURAL_UNIT_OF_MOMENTUM_IN_MEV_C: Constant = Constant {
    name: "natural unit of momentum in MeV/c",
    symbol: None,
    value: 0.510_998_950_69,
    units: Some("MeV/c"),
    uncertainty: Uncertainty::Known(0.000_000_000_16),
};
//227
pub static NATURAL_UNIT_OF_TIME: Constant = Constant {
    name: "natural unit of time",
    symbol: None,
    value: 1.288_088_666_44e-21,
    units: Some("s"),
    uncertainty: Uncertainty::Known(0.000_000_000_4e-21),
};
//228
pub static NATURAL_UNIT_OF_VELOCITY: Constant = Constant {
    name: "natural unit of velocity",
    symbol: None,
    value: 299_792_458.0,
    units: Some("m s^-1"),
    uncertainty: Uncertainty::Exact,
};
//229
pub static NEUTRON_COMPTON_WAVELENGTH: Constant = Constant {
    name: "neutron Compton wavelength",
    symbol: None,
    value: 1.319_590_903_82e-15,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_000_67e-15),
};
//230
pub static NEUTRON_ELECTRON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "neutron-electron magnetic moment ratio",
    symbol: None,
    value: 1.040_668_84e-3,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_24e-3),
};
//231
pub static NEUTRON_ELECTRON_MASS_RATIO: Constant = Constant {
    name: "neutron-electron mass ratio",
    symbol: None,
    value: 1_838.683_662,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_74),
};
//232
pub static NEUTRON_G_FACTOR: Constant = Constant {
    name: "neutron g factor",
    symbol: None,
    value: -3.826_085_52,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_9),
};
//233
pub static NEUTRON_GYROMAGNETIC_RATIO: Constant = Constant {
    name: "neutron gyromagnetic ratio",
    symbol: None,
    value: 1.832_471_74e8,
    units: Some("s^-1 T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_43e8),
};
//234
pub static NEUTRON_GYROMAGNETIC_RATIO_IN_MEGAHERTZ_PER_TESLA: Constant = Constant {
    name: "neutron gyromagnetic ratio in megahertz per tesla",
    symbol: None,
    value: 29.164_693_5,
    units: Some("MHz T^-1"),
    uncertainty: Uncertainty::Known(0.000_006_9),
};
//235
pub static NEUTRON_MAGNETIC_MOMENT: Constant = Constant {
    name: "neutron magnetic moment",
    symbol: None,
    value: -9.662_365_3e-27,
    units: Some("J T^-1"),
    uncertainty: Uncertainty::Known(0.000_002_3e-27),
};
//236
pub static NEUTRON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "neutron magnetic moment to Bohr magneton ratio",
    symbol: None,
    value: -1.041_875_65e-3,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_25e-3),
};
//237
pub static NEUTRON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "neutron magnetic moment to nuclear magneton ratio",
    symbol: None,
    value: -1.913_042_76,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_45),
};
//238
pub static NEUTRON_MASS: Constant = Constant {
    name: "neutron mass",
    symbol: None,
    value: 1.674_927_500_56e-27,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_000_000_85e-27),
};
//239
pub static NEUTRON_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "neutron mass energy equivalent",
    symbol: None,
    value: 1.505_349_765_14e-10,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_000_76e-10),
};
//240
pub static NEUTRON_MASS_ENERGY_EQUIVALENT_IN_MEV: Constant = Constant {
    name: "neutron mass energy equivalent in MeV",
    symbol: None,
    value: 939.565_421_94,
    units: Some("MeV"),
    uncertainty: Uncertainty::Known(0.000_000_48),
};
//241
pub static NEUTRON_MASS_IN_U: Constant = Constant {
    name: "neutron mass in u",
    symbol: None,
    value: 1.008_664_916_06,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_000_4),
};
//242
pub static NEUTRON_MOLAR_MASS: Constant = Constant {
    name: "neutron molar mass",
    symbol: None,
    value: 1.008_664_917_12e-3,
    units: Some("kg mol^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_51e-3),
};
//243
pub static NEUTRON_MUON_MASS_RATIO: Constant = Constant {
    name: "neutron-muon mass ratio",
    symbol: None,
    value: 8.892_484_08,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_2),
};
//244
pub static NEUTRON_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "neutron-proton magnetic moment ratio",
    symbol: None,
    value: -0.684_979_35,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_16),
};
//245
pub static NEUTRON_PROTON_MASS_DIFFERENCE: Constant = Constant {
    name: "neutron-proton mass difference",
    symbol: None,
    value: 2.305_574_61e-30,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_000_67e-30),
};
//246
pub static NEUTRON_PROTON_MASS_DIFFERENCE_ENERGY_EQUIVALENT: Constant = Constant {
    name: "neutron-proton mass difference energy equivalent",
    symbol: None,
    value: 2.072_147_12e-13,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_6e-13),
};
//247
pub static NEUTRON_PROTON_MASS_DIFFERENCE_ENERGY_EQUIVALENT_IN_MEV: Constant = Constant {
    name: "neutron-proton mass difference energy equivalent in MeV",
    symbol: None,
    value: 1.293_332_51,
    units: Some("MeV"),
    uncertainty: Uncertainty::Known(0.000_000_38),
};
//248
pub static NEUTRON_PROTON_MASS_DIFFERENCE_IN_U: Constant = Constant {
    name: "neutron-proton mass difference in u",
    symbol: None,
    value: 1.388_449_48e-3,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_4e-3),
};
//249
pub static NEUTRON_PROTON_MASS_RATIO: Constant = Constant {
    name: "neutron-proton mass ratio",
    symbol: None,
    value: 1.001_378_419_46,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_4),
};
//250
pub static NEUTRON_RELATIVE_ATOMIC_MASS: Constant = Constant {
    name: "neutron relative atomic mass",
    symbol: None,
    value: 1.008_664_916_06,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_4),
};
//251
pub static NEUTRON_TAU_MASS_RATIO: Constant = Constant {
    name: "neutron-tau mass ratio",
    symbol: None,
    value: 0.528_779,
    units: None,
    uncertainty: Uncertainty::Known(0.000_036),
};
//252
pub static NEUTRON_TO_SHIELDED_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "neutron to shielded proton magnetic moment ratio",
    symbol: None,
    value: -0.684_996_94,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_16),
};
//253
pub static NEWTONIAN_CONSTANT_OF_GRAVITATION: Constant = Constant {
    name: "Newtonian constant of gravitation",
    symbol: Some("G"),
    value: 6.674_3e-11,
    units: Some("m^3 kg^-1 s^-2"),
    uncertainty: Uncertainty::Known(0.000_15e-11),
};
//254
pub static NEWTONIAN_CONSTANT_OF_GRAVITATION_OVER_H_BAR_C: Constant = Constant {
    name: "Newtonian constant of gravitation over h-bar c",
    symbol: None,
    value: 6.708_83e-39,
    units: Some("(GeV/c^2)^-2"),
    uncertainty: Uncertainty::Known(0.000_15e-39),
};
//255
pub static NUCLEAR_MAGNETON: Constant = Constant {
    name: "nuclear magneton",
    symbol: None,
    value: 5.050_783_739_3e-27,
    units: Some("J T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_001_6e-27),
};
//256
pub static NUCLEAR_MAGNETON_IN_ELECTRON_VOLT_PER_TESLA: Constant = Constant {
    name: "nuclear magneton in eV per tesla",
    symbol: None,
    value: 3.152_451_254_17e-8,
    units: Some("eV T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_98e-8),
};
//257
pub static NUCLEAR_MAGNETON_IN_INVERSE_METER_PER_TESLA: Constant = Constant {
    name: "nuclear magneton in inverse meter per tesla",
    symbol: None,
    value: 2.542_623_410_09e-2,
    units: Some("m^-1 T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_79e-2),
};
//258
pub static NUCLEAR_MAGNETON_IN_KELVIN_PER_TESLA: Constant = Constant {
    name: "nuclear magneton in kelvin per tesla",
    symbol: None,
    value: 3.658_267_770_6e-4,
    units: Some("K T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_001_1e-4),
};
//259
pub static NUCLEAR_MAGNETON_IN_MEGAHERTZ_PER_TESLA: Constant = Constant {
    name: "nuclear magneton in megahertz per tesla",
    symbol: None,
    value: 7.622_593_218_8,
    units: Some("MHz T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_002_4),
};
//260
pub static PLANCK_CONSTANT: Constant = Constant {
    name: "Planck constant",
    symbol: Some("h"),
    value: 6.626_070_15e-34,
    units: Some("J Hz^-1"),
    uncertainty: Uncertainty::Exact,
};
//261
pub static PLANCK_CONSTANT_IN_EV_PER_HZ: Constant = Constant {
    name: "Planck constant in eV/Hz",
    symbol: None,
    value: 4.135_667_696e-15, //...
    units: Some("eV Hz^-1"),
    uncertainty: Uncertainty::Exact,
};
//262
pub static PLANCK_LENGTH: Constant = Constant {
    name: "Planck length",
    symbol: None,
    value: 1.616_255e-35,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_018e-35),
};
//263
pub static PLANCK_MASS: Constant = Constant {
    name: "Planck mass",
    symbol: None,
    value: 2.176_434e-8,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_024e-8),
};
//264
pub static PLANCK_MASS_ENERGY_EQUIVALENT_IN_GEV: Constant = Constant {
    name: "Planck mass energy equivalent in GeV",
    symbol: None,
    value: 1.220_890e19,
    units: Some("GeV"),
    uncertainty: Uncertainty::Known(0.000_014e19),
};
//265
pub static PLANCK_TEMPERATURE: Constant = Constant {
    name: "Planck temperature",
    symbol: None,
    value: 1.416_784e32,
    units: Some("K"),
    uncertainty: Uncertainty::Known(0.000_016e32),
};
//266
pub static PLANCK_TIME: Constant = Constant {
    name: "Planck time",
    symbol: None,
    value: 5.391_247e-44,
    units: Some("s"),
    uncertainty: Uncertainty::Known(0.000_06e-44),
};
//267
pub static PROTON_CHARGE_TO_MASS_QUOTIENT: Constant = Constant {
    name: "proton charge to mass quotient",
    symbol: None,
    value: 9.578_833_143e7,
    units: Some("C kg^-1"),
    uncertainty: Uncertainty::Known(0.000_000_003e7),
};
//268
pub static PROTON_COMPTON_WAVELENGTH: Constant = Constant {
    name: "proton Compton wavelength",
    symbol: None,
    value: 1.321_409_853_6e-15,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_000_41e-15),
};
//269
pub static PROTON_ELECTRON_MASS_RATIO: Constant = Constant {
    name: "proton-electron mass ratio",
    symbol: None,
    value: 1_836.152_673_426,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_032),
};
//270
pub static PROTON_G_FACTOR: Constant = Constant {
    name: "proton g factor",
    symbol: None,
    value: 5.585_694_689_3,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_001_6),
};
//271
pub static PROTON_GYROMAGNETIC_RATIO: Constant = Constant {
    name: "proton gyromagnetic ratio",
    symbol: None,
    value: 2.675_221_870_8e8,
    units: Some("s^-1 T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_001_1e8),
};
//272
pub static PROTON_GYROMAGNETIC_RATIO_IN_MEGAHERTZ_PER_TESLA: Constant = Constant {
    name: "proton gyromagnetic ratio in megahertz per tesla",
    symbol: None,
    value: 42.577_478_461,
    units: Some("MHz T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_018),
};
//273
pub static PROTON_MAGNETIC_MOMENT: Constant = Constant {
    name: "proton magnetic moment",
    symbol: None,
    value: 1.410_606_795_45e-26,
    units: Some("J T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_6e-26),
};
//274
pub static PROTON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "proton magnetic moment to Bohr magneton ratio",
    symbol: None,
    value: 1.521_032_202_3e-3,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_45e-3),
};
//275
pub static PROTON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "proton magnetic moment to nuclear magneton ratio",
    symbol: None,
    value: 2.792_847_344_63,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_82),
};
//276
pub static PROTON_MAGNETIC_SHIELDING_CORRECTION: Constant = Constant {
    name: "proton magnetic shielding correction",
    symbol: None,
    value: 2.567_15e-5,
    units: None,
    uncertainty: Uncertainty::Known(0.000_41e-5),
};
//277
pub static PROTON_MASS: Constant = Constant {
    name: "proton mass",
    symbol: None,
    value: 1.672_621_925_95e-27,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_000_000_52e-27),
};
//278
pub static PROTON_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "proton mass energy equivalent",
    symbol: None,
    value: 1.503_277_618_02e-10,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_000_47e-10),
};
//279
pub static PROTON_MASS_ENERGY_EQUIVALENT_IN_MEV: Constant = Constant {
    name: "proton mass energy equivalent in MeV",
    symbol: None,
    value: 938.272_089_43,
    units: Some("MeV"),
    uncertainty: Uncertainty::Known(0.000_000_29),
};
//280
pub static PROTON_MASS_IN_U: Constant = Constant {
    name: "proton mass in u",
    symbol: None,
    value: 1.007_276_466_578_9,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_000_008_3),
};
//281
pub static PROTON_MOLAR_MASS: Constant = Constant {
    name: "proton molar mass",
    symbol: None,
    value: 1.007_276_467_64e-3,
    units: Some("kg mol^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_31e-3),
};
//282
pub static PROTON_MUON_MASS_RATIO: Constant = Constant {
    name: "proton-muon mass ratio",
    symbol: None,
    value: 8.880_243_38,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_2),
};
//283
pub static PROTON_NEUTRON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "proton-neutron magnetic moment ratio",
    symbol: None,
    value: -1.459_898_02,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_34),
};
//284
pub static PROTON_NEUTRON_MASS_RATIO: Constant = Constant {
    name: "proton-neutron mass ratio",
    symbol: None,
    value: 0.998_623_477_97,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_4),
};
//285
pub static PROTON_RELATIVE_ATOMIC_MASS: Constant = Constant {
    name: "proton relative atomic mass",
    symbol: None,
    value: 1.007_276_466_578_9,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_008_3),
};
//286
pub static PROTON_RMS_CHARGE_RADIUS: Constant = Constant {
    name: "proton rms charge radius",
    symbol: None,
    value: 8.407_5e-16,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.006_4e-16),
};
//287
pub static PROTON_TAU_MASS_RATIO: Constant = Constant {
    name: "proton-tau mass ratio",
    symbol: None,
    value: 0.528_051,
    units: None,
    uncertainty: Uncertainty::Known(0.000_036),
};
//288
pub static QUANTUM_OF_CIRCULATION: Constant = Constant {
    name: "quantum of circulation",
    symbol: None,
    value: 3.636_947_546_7e-4,
    units: Some("m^2 s^-1"),
    uncertainty: Uncertainty::Known(0.000_000_001_1e-4),
};
//289
pub static QUANTUM_OF_CIRCULATION_TIMES_2: Constant = Constant {
    name: "quantum of circulation times 2",
    symbol: None,
    value: 7.273_895_093_4e-4,
    units: Some("m^2 s^-1"),
    uncertainty: Uncertainty::Known(0.000_000_002_3e-4),
};
//290
pub static REDUCED_COMPTON_WAVELENGTH: Constant = Constant {
    name: "reduced Compton wavelength",
    symbol: None,
    value: 3.861_592_674_4e-13,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_001_2e-13),
};
//291
pub static REDUCED_MUON_COMPTON_WAVELENGTH: Constant = Constant {
    name: "reduced muon Compton wavelength",
    symbol: None,
    value: 1.867_594_306e-15,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_042e-15),
};
//292
pub static REDUCED_NEUTRON_COMPTON_WAVELENGTH: Constant = Constant {
    name: "reduced neutron Compton wavelength",
    symbol: None,
    value: 2.100_194_152e-16,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_001_1e-16),
};
//293
pub static REDUCED_PLANCK_CONSTANT: Constant = Constant {
    name: "reduced Planck constant",
    symbol: None,
    value: 1.054_571_817e-34, //...
    units: Some("J s"),
    uncertainty: Uncertainty::Exact,
};
//294
pub static REDUCED_PLANCK_CONSTANT_IN_EV_S: Constant = Constant {
    name: "reduced Planck constant in eV s",
    symbol: None,
    value: 6.582_119_569e-16, //...
    units: Some("eV s"),
    uncertainty: Uncertainty::Exact,
};
//295
pub static REDUCED_PLANCK_CONSTANT_TIMES_C_IN_MEV_FM: Constant = Constant {
    name: "reduced Planck constant times c in MeV fm",
    symbol: None,
    value: 197.326_980_4,
    units: Some("MeV fm"),
    uncertainty: Uncertainty::Exact,
};
//296
pub static REDUCED_PROTON_COMPTON_WAVELENGTH: Constant = Constant {
    name: "reduced proton Compton wavelength",
    symbol: None,
    value: 2.103_089_100_51e-16,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_000_000_66e-16),
};
//297
pub static REDUCED_TAU_COMPTON_WAVELENGTH: Constant = Constant {
    name: "reduced tau Compton wavelength",
    symbol: None,
    value: 1.110_538e-16,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_075e-16),
};
//298
pub static RYDBERG_CONSTANT: Constant = Constant {
    name: "Rydberg constant",
    symbol: None,
    value: 10_973_731.568_157,
    units: Some("m^-1"),
    uncertainty: Uncertainty::Known(0.000_012),
};
//299
pub static RYDBERG_CONSTANT_TIMES_C_IN_HZ: Constant = Constant {
    name: "Rydberg constant times c in Hz",
    symbol: None,
    value: 3.289_841_960_25e15,
    units: Some("Hz"),
    uncertainty: Uncertainty::Known(0.000_000_000_003_6e15),
};
//300
pub static RYDBERG_CONSTANT_TIMES_HC_IN_ELECTRON_VOLT: Constant = Constant {
    name: "Rydberg constant times hc in electron volt",
    symbol: None,
    value: 13.605_693_122_99,
    units: Some("eV"),
    uncertainty: Uncertainty::Known(0.000_000_000_015),
};
//301
pub static RYDBERG_CONSTANT_TIMES_HC_IN_J: Constant = Constant {
    name: "Rydberg constant times hc in J",
    symbol: None,
    value: 2.179_872_361_103e-18,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_000_002_4e-18),
};
//302
pub static SACKUR_TETRODE_1K_100KPA: Constant = Constant {
    name: "Sackur-Tetrode constant (1 K, 100 kPa)",
    symbol: None,
    value: -1.151_707_534_96,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_47),
};
//303
pub static SACKUR_TETRODE_1K_101P325KPA: Constant = Constant {
    name: "Sackur-Tetrode constant (1 K, 101.325 kPa)",
    symbol: None,
    value: -1.164_870_521_49,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_47),
};
//304
pub static SECOND_RADIATION_CONSTANT: Constant = Constant {
    name: "second radiation constant",
    symbol: None,
    value: 1.438_776_877e-2, //...
    units: Some("m K"),
    uncertainty: Uncertainty::Exact,
};
//305
pub static SHIELDED_HELION_GYROMAGNETIC_RATIO: Constant = Constant {
    name: "shielded helion gyromagnetic ratio",
    symbol: None,
    value: 2.037_894_607_8e8,
    units: Some("s^-1 T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_001_8e8),
};
//306
pub static SHIELDED_HELION_GYROMAGNETIC_RATIO_IN_MEGAHERTZ_PER_TESLA: Constant = Constant {
    name: "shielded helion gyromagnetic ratio in megahertz per tesla",
    symbol: None,
    value: 32.434_100_033,
    units: Some("MHz T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_028),
};
//307
pub static SHIELDED_HELION_MAGNETIC_MOMENT: Constant = Constant {
    name: "shielded helion magnetic moment",
    symbol: None,
    value: -1.074_553_110_35e-26,
    units: Some("J T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_93e-26),
};
//308
pub static SHIELDED_HELION_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "shielded helion magnetic moment to Bohr magneton ratio",
    symbol: None,
    value: -1.158_671_494_57e-3,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_94e-3),
};
//309
pub static SHIELDED_HELION_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "shielded helion magnetic moment to nuclear magneton ratio",
    symbol: None,
    value: -2.127_497_762_4,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_001_7),
};
//310
pub static SHIELDED_HELION_TO_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "shielded helion to proton magnetic moment ratio",
    symbol: None,
    value: -0.761_766_577_21,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_66),
};
//311
pub static SHIELDED_HELION_TO_SHIELDED_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "shielded helion to shielded proton magnetic moment ratio",
    symbol: None,
    value: -0.761_786_133_4,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_003_1),
};
//312
pub static SHIELDED_PROTON_GYROMAGNETIC_RATIO: Constant = Constant {
    name: "shielded proton gyromagnetic ratio",
    symbol: None,
    value: 2.675_153_194e8,
    units: Some("s^-1 T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_011e8),
};
//313
pub static SHIELDED_PROTON_GYROMAGNETIC_RATIO_IN_MEGAHERTZ_PER_TESLA: Constant = Constant {
    name: "shielded proton gyromagnetic ratio in megahertz per tesla",
    symbol: None,
    value: 42.576_385_43,
    units: Some("MHz T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_17),
};
//314
pub static SHIELDED_PROTON_MAGNETIC_MOMENT: Constant = Constant {
    name: "shielded proton magnetic moment",
    symbol: None,
    value: 1.410_570_583e-26,
    units: Some("J T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_005_8e-26),
};
//315
pub static SHIELDED_PROTON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "shielded proton magnetic moment to Bohr magneton ratio",
    symbol: None,
    value: 1.520_993_155_1e-3,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_006_2e-3),
};
//316  
pub static SHIELDED_PROTON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "shielded proton magnetic moment to nuclear magneton ratio",
    symbol: None,
    value: 2.792_775_648,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_011),
};
//317
pub static SHIELDING_DIFFERENCE_TO_D_AND_P_IN_HD: Constant = Constant {
    name: "shielding difference of d and p in HD",
    symbol: None,
    value: 1.987_70e-8,
    units: None,
    uncertainty: Uncertainty::Known(0.000_1e-8),
};
//318
pub static SHIELDING_DIFFERENCE_OF_T_AND_P_IN_HT: Constant = Constant {
    name: "shielding difference of t and p in HT",
    symbol: None,
    value: 2.394_5e-8,
    units: None,
    uncertainty: Uncertainty::Known(0.000_2e-8),
};
//319
pub static SPEED_OF_LIGHT_IN_VACUUM: Constant = Constant {
    name: "speed of light in vacuum",
    symbol: Some("c"),
    value: 299_792_458.0,
    units: Some("m s^-1"),
    uncertainty: Uncertainty::Exact,
};
//320
pub static STANDARD_ACCELERATION_OF_GRAVITY: Constant = Constant {
    name: "standard acceleration of gravity",
    symbol: None,
    value: 9.806_65,
    units: Some("m s^-2"),
    uncertainty: Uncertainty::Exact,
};
//321
pub static STANDARD_ATMOSPHERE: Constant = Constant {
    name: "standard atmosphere",
    symbol: None,
    value: 101_325.0,
    units: Some("Pa"),
    uncertainty: Uncertainty::Exact,
};
//322
pub static STANDARD_STATE_PRESSURE: Constant = Constant {
    name: "standard-state pressure",
    symbol: None,
    value: 100_000.0,
    units: Some("Pa"),
    uncertainty: Uncertainty::Exact,
};
//323
pub static STEFAN_BOLTZMANN_CONSTANT: Constant = Constant {
    name: "Stefan-Boltzmann constant",
    symbol: None,
    value: 5.670_374_419e-8,
    units: Some("W m^-2 K^-4"),
    uncertainty: Uncertainty::Exact,
};
//324
pub static TAU_COMPTON_WAVELENGTH: Constant = Constant {
    name: "tau Compton wavelength",
    symbol: None,
    value: 6.977_71e-16,
    units: Some("m"),
    uncertainty: Uncertainty::Known(0.000_47e-16),
};
//325
pub static TAU_ELECTRON_MASS_RATIO: Constant = Constant {
    name: "tau-electron mass ratio",
    symbol: None,
    value: 3_477.23,
    units: None,
    uncertainty: Uncertainty::Known(0.23),
};
//326
pub static TAU_ENERGY_EQUIVALENT: Constant = Constant {
    name: "tau energy equivalent",
    symbol: None,
    value: 1_776.86,
    units: Some("MeV"),
    uncertainty: Uncertainty::Known(0.12),
};
//327
pub static TAU_MASS: Constant = Constant {
    name: "tau mass",
    symbol: None,
    value: 3.167_54e-27,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_21e-27),
};
//328
pub static TAU_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "tau mass energy equivalent",
    symbol: None,
    value: 2.846_84e-10,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_19e-10),
};
//329
pub static TAU_MASS_IN_U: Constant = Constant {
    name: "tau mass in u",
    symbol: None,
    value: 1.907_54,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_13),
};
//330
pub static TAU_MOLAR_MASS: Constant = Constant {
    name: "tau molar mass",
    symbol: None,
    value: 1.907_54e-3,
    units: Some("kg mol^-1"),
    uncertainty: Uncertainty::Known(0.000_13e-3),
};
//331
pub static TAU_MUON_MASS_RATIO: Constant = Constant {
    name: "tau-muon mass ratio",
    symbol: None,
    value: 16.817,
    units: None,
    uncertainty: Uncertainty::Known(0.001_1),
};
//332
pub static TAU_NEUTRON_MASS_RATIO: Constant = Constant {
    name: "tau-neutron mass ratio",
    symbol: None,
    value: 1.891_15,
    units: None,
    uncertainty: Uncertainty::Known(0.000_13),
};
//333
pub static TAU_PROTON_MASS_RATIO: Constant = Constant {
    name: "tau-proton mass ratio",
    symbol: None,
    value: 1.893_76,
    units: None,
    uncertainty: Uncertainty::Known(0.000_13),
};
//334
pub static THOMSON_CROSS_SECTION: Constant = Constant {
    name: "Thomson cross section",
    symbol: None,
    value: 6.652_458_705_1e-29,
    units: Some("m^2"),
    uncertainty: Uncertainty::Known(0.000_000_006_2e-29),
};
//335   
pub static TRITON_ELECTRON_MASS_RATIO: Constant = Constant {
    name: "triton-electron mass ratio",
    symbol: None,
    value: 5_496.921_535_51,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_21),
};
//336
pub static TRITON_G_FACTOR: Constant = Constant {
    name: "triton g factor",
    symbol: None,
    value: 5.957_924_93,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_012),
};
//337
pub static TRITON_MAGNETIC_MOMENT: Constant = Constant {
    name: "triton magnetic moment",
    symbol: None,
    value: 1.504_609_517_8e-26,
    units: Some("J T^-1"),
    uncertainty: Uncertainty::Known(0.000_000_003e-26),
};
//338
pub static TRITON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "triton magnetic moment to Bohr magneton ratio",
    symbol: None,
    value: 1.622_393_664_8e-3,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_003_2e-3),
};
//339
pub static TRITON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "triton magnetic moment to nuclear magneton ratio",
    symbol: None,
    value: 2.978_962_465,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_005_9),
};
//340
pub static TRITON_MASS: Constant = Constant {
    name: "triton mass",
    symbol: None,
    value: 5.007_356_751_2e-27,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_000_001_6e-27),
};
//341
pub static TRITON_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "triton mass energy equivalent",
    symbol: None,
    value: 4.500_387_811_9e-10,
    units: Some("J"),
    uncertainty: Uncertainty::Known(0.000_000_001_4e-10),
};
//342
pub static TRITON_MASS_ENERGY_EQUIVALENT_IN_MEGA_ELECTRON_VOLT: Constant = Constant {
    name: "triton mass energy equivalent in mega electron volt",
    symbol: None,
    value: 2_808.921_136_68,
    units: Some("MeV"),
    uncertainty: Uncertainty::Known(0.000_000_88),
};
//343
pub static TRITON_MASS_IN_U: Constant = Constant {
    name: "triton mass in u",
    symbol: None,
    value: 3.015_500_715_97,
    units: Some("u"),
    uncertainty: Uncertainty::Known(0.000_000_000_1),
};
//344
pub static TRITON_MOLAR_MASS: Constant = Constant {
    name: "triton molar mass",
    symbol: None,
    value: 3.015_500_719_13e-3,
    units: Some("kg mol^-1"),
    uncertainty: Uncertainty::Known(0.000_000_000_94e-3),
};
//345
pub static TRITON_PROTON_MASS_RATIO: Constant = Constant {
    name: "triton-proton mass ratio",
    symbol: None,
    value: 2.993_717_034_03,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_1),
};
//346
pub static TRITON_RELATIVE_ATOMIC_MASS: Constant = Constant {
    name: "triton relative atomic mass",
    symbol: None,
    value: 3.015_500_715_97,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_000_1),
};
//347
pub static TRITON_TO_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "triton to proton magnetic moment ratio",
    symbol: None,
    value: 1.066_639_918_9,
    units: None,
    uncertainty: Uncertainty::Known(0.000_000_002_1),
};
//348
pub static UNIFIED_ATOMIC_MASS_UNIT: Constant = Constant {
    name: "unified atomic mass unit",
    symbol: None,
    value: 1.660_539_068_92e-27,
    units: Some("kg"),
    uncertainty: Uncertainty::Known(0.000_000_000_52e-27),
};
//349
pub static VACUUM_ELECTRIC_PERMITTIVITY: Constant = Constant {
    name: "vacuum electric permittivity",
    symbol: None,
    value: 8.854_187_818_8e-12,
    units: Some("F m^-1"),
    uncertainty: Uncertainty::Known(0.000_000_001_4e-12),
};
//350
pub static VACUUM_MAGNETIC_PERMEABILITY: Constant = Constant {
    name: "vacuum magnetic permeability",
    symbol: None,
    value: 1.256_637_061_27e-6,
    units: Some("N A^-2"),
    uncertainty: Uncertainty::Known(0.000_000_000_2e-6),
};
//351
pub static VON_KLITZING_CONSTANT: Constant = Constant {
    name: "von Klitzing constant",
    symbol: None,
    value: 25_812.807_45,
    units: Some("ohm"),
    uncertainty: Uncertainty::Exact,
};
//352
pub static WEAK_MIXING_ANGLE: Constant = Constant {
    name: "weak mixing angle",
    symbol: None,
    value: 0.223_05,
    units: None,
    uncertainty: Uncertainty::Known(0.000_23),
};
//353
pub static WIEN_FREQUENCY_DISPLACEMENT_LAW_CONSTANT: Constant = Constant {
    name: "Wien frequency displacement law constant",
    symbol: None,
    value: 5.878_925_757e10,
    units: Some("Hz K^-1"),
    uncertainty: Uncertainty::Exact,
};
//354
pub static WIEN_WAVELENGTH_DISPLACEMENT_LAW_CONSTANT: Constant = Constant {
    name: "Wien wavelength displacement law constant",
    symbol: None,
    value: 2.897_771_955e-3,
    units: Some("m K"),
    uncertainty: Uncertainty::Exact,
};
//355
pub static W_TO_Z_MASS_RATIO: Constant = Constant {
    name: "W to Z mass ratio",
    symbol: None,
    value: 0.881_45,
    units: None,
    uncertainty: Uncertainty::Known(0.000_13),
};

// CATALOG

pub static CONSTANTS: [&Constant; 355] = [
    &ALPHA_PARTICLE_ELECTRON_MASS_RATIO, //001
    &ALPHA_PARTICLE_MASS, //002
    &ALPHA_PARTICLE_MASS_ENERGY_EQUIVALENT, //003
    &ALPHA_PARTICLE_MASS_ENERGY_EQUIVALENT_IN_MEGA_ELECTRON_VOLT, //004
    &ALPHA_PARTICLE_MASS_IN_U, //005
    &ALPHA_PARTICLE_MOLAR_MASS, //006
    &ALPHA_PARTICLE_PROTON_MASS_RATIO, //007
    &ALPHA_PARTICLE_RELATIVE_ATOMIC_MASS, //008
    &ALPHA_PARTICLE_RMS_CHARGE_RADIUS, //009
    &ANGSTROM_STAR, //010
    &ATOMIC_MASS_CONSTANT, //011
    &ATOMIC_MASS_CONSTANT_ENERGY_EQUIVALENT, //012
    &ATOMIC_MASS_CONSTANT_ENERGY_EQUIVALENT_IN_MEGA_ELECTRON_VOLT, //013
    &ATOMIC_MASS_UNIT_ELECTRON_VOLT_RELATIONSHIP, //014
    &ATOMIC_MASS_UNIT_HARTREE_RELATIONSHIP, //015
    &ATOMIC_MASS_UNIT_HERTZ_RELATIONSHIP, //016
    &ATOMIC_MASS_UNIT_INVERSE_METER_RELATIONSHIP, //017
    &ATOMIC_MASS_UNIT_JOULE_RELATIONSHIP, //018
    &ATOMIC_MASS_UNIT_KELVIN_RELATIONSHIP, //019
    &ATOMIC_MASS_UNIT_KILOGRAM_RELATIONSHIP, //020
    &ATOMIC_UNIT_OF_1ST_HYPERPOLARIZABILITY, //021
    &ATOMIC_UNIT_OF_2ND_HYPERPOLARIZABILITY, //022
    &ATOMIC_UNIT_OF_ACTION, //023
    &ATOMIC_UNIT_OF_CHARGE, //024
    &ATOMIC_UNIT_OF_CHARGE_DENSITY, //025
    &ATOMIC_UNIT_OF_CURRENT, //026
    &ATOMIC_UNIT_OF_ELECTRIC_DIPOLE_MOMENT, //027
    &ATOMIC_UNIT_OF_ELECTRIC_FIELD, //028
    &ATOMIC_UNIT_OF_ELECTRIC_FIELD_GRADIENT, //029
    &ATOMIC_UNIT_OF_ELECTRIC_POLARIZABILITY, //030
    &ATOMIC_UNIT_OF_ELECTRIC_POTENTIAL, //031
    &ATOMIC_UNIT_OF_ELECTRIC_QUADRUPOLE_MOMENT, //032
    &ATOMIC_UNIT_OF_ENERGY, //033
    &ATOMIC_UNIT_OF_FORCE, //034
    &ATOMIC_UNIT_OF_LENGTH, //035
    &ATOMIC_UNIT_OF_MAGNETIC_DIPOLE_MOMENT, //036
    &ATOMIC_UNIT_OF_MAGNETIC_FLUX_DENSITY, //037
    &ATOMIC_UNIT_OF_MAGNETIZABILITY, //038
    &ATOMIC_UNIT_OF_MASS, //039
    &ATOMIC_UNIT_OF_MOMENTUM, //040
    &ATOMIC_UNIT_OF_PERMITTIVITY, //041
    &ATOMIC_UNIT_OF_TIME, //042
    &ATOMIC_UNIT_OF_VELOCITY, //043
    &AVOGADRO_CONSTANT, //044
    &BOHR_MAGNETON, //045
    &BOHR_MAGNETON_IN_ELECTRON_VOLT_PER_TESLA, //046
    &BOHR_MAGNETON_IN_HZ_PER_TESLA, //047
    &BOHR_MAGNETON_IN_INVERSE_METER_PER_TESLA, //048
    &BOHR_MAGNETON_IN_KELVIN_PER_TESLA, //049
    &BOHR_RADIUS, //050
    &BOLTZMANN_CONSTANT, //051
    &BOLTZMANN_CONSTANT_IN_ELECTRON_VOLT_PER_KELVIN, //052
    &BOLTZMANN_CONSTANT_IN_HZ_PER_K, //053
    &BOLTZMANN_CONSTANT_IN_INVERSE_METER_PER_K, //054
    &CHARACTERISTIC_IMPEDANCE_OF_VACUUM, //055
    &CLASSICAL_ELECTRON_RADIUS, //056
    &COMPTON_WAVELENGTH, //057
    &CONDUCTANCE_QUANTUM, //058
    &CONVENTIONAL_VALUE_OF_AMPERE_90, //059
    &CONVENTIONAL_VALUE_OF_COULOMB_90, //060
    &CONVENTIONAL_VALUE_OF_FARAD_90, //061
    &CONVENTIONAL_VALUE_OF_HENRY_90, //062
    &CONVENTIONAL_VALUE_OF_JOSEPHSON_CONSTANT, //063
    &CONVENTIONAL_VALUE_OF_OHM_90, //064
    &CONVENTIONAL_VALUE_OF_VOLT_90, //065
    &CONVENTIONAL_VALUE_OF_VON_KLITZING_CONSTANT, //066
    &CONVENTIONAL_VALUE_OF_WATT_90, //067
    &COPPER_X_UNIT, //068
    &DEUTERON_ELECTRON_MAGNETIC_MOMENT_RATIO, //069
    &DEUTERON_ELECTRON_MASS_RATIO, //070
    &DEUTERON_G_FACTOR, //071
    &DEUTERON_MAGNETIC_MOMENT, //072
    &DEUTERON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO, //073
    &DEUTERON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO, //074
    &DEUTERON_MASS, //075
    &DEUTERON_MASS_ENERGY_EQUIVALENT, //076
    &DEUTERON_MASS_ENERGY_EQUIVALENT_IN_MEGA_ELECTRON_VOLT, //077
    &DEUTERON_MASS_IN_U, //078
    &DEUTERON_MOLAR_MASS, //079
    &DEUTERON_NEUTRON_MAGNETIC_MOMENT_RATIO, //080
    &DEUTERON_PROTON_MAGNETIC_MOMENT_RATIO, //081
    &DEUTERON_PROTON_MASS_RATIO, //082
    &DEUTERON_RELATIVE_ATOMIC_MASS, //083
    &DEUTERON_RMS_CHARGE_RADIUS, //084
    &ELECTRON_CHARGE_TO_MASS_QUOTIENT, //085
    &ELECTRON_DEUTERON_MAGNETIC_MOMENT_RATIO, //086
    &ELECTRON_DEUTERON_MASS_RATIO, //087
    &ELECTRON_G_FACTOR, //088
    &ELECTRON_GYROMAGNETIC_RATIO, //089
    &ELECTRON_GYROMAGNETIC_RATIO_IN_MEGAHERTZ_PER_TESLA, //090
    &ELECTRON_HELION_MASS_RATIO, //091
    &ELECTRON_MAGNETIC_MOMENT, //092
    &ELECTRON_MAGNETIC_MOMENT_ANOMALY, //093
    &ELECTRON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO, //094
    &ELECTRON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO, //095
    &ELECTRON_MASS, //096
    &ELECTRON_MASS_ENERGY_EQUIVALENT, //097
    &ELECTRON_MASS_ENERGY_EQUIVALENT_IN_MEV, //098
    &ELECTRON_MASS_IN_U, //099
    &ELECTRON_MOLAR_MASS, //100
    &ELECTRON_MUON_MAGNETIC_MOMENT_RATIO, //101
    &ELECTRON_MUON_MASS_RATIO, //102
    &ELECTRON_NEUTRON_MAGNETIC_MOMENT_RATIO, //103
    &ELECTRON_NEUTRON_MASS_RATIO, //104
    &ELECTRON_PROTON_MAGNETIC_MOMENT_RATIO, //105
    &ELECTRON_PROTON_MASS_RATIO, //106
    &ELECTRON_RELATIVE_ATOMIC_MASS, //107
    &ELECTRON_TAU_MASS_RATIO, //108
    &ELECTRON_TO_ALPHA_PARTICLE_MASS_RATIO, //109
    &ELECTRON_TO_SHIELDED_HELION_MAGNETIC_MOMENT_RATIO, //110
    &ELECTRON_TO_SHIELDED_PROTON_MAGNETIC_MOMENT_RATIO, //111
    &ELECTRON_TRITON_MASS_RATIO, //112
    &ELECTRON_VOLT, //113
    &ELECTRON_VOLT_ATOMIC_MASS_UNIT_RELATIONSHIP, //114
    &ELECTRON_VOLT_HARTREE_RELATIONSHIP, //115
    &ELECTRON_VOLT_HERTZ_RELATIONSHIP, //116
    &ELECTRON_VOLT_INVERSE_METER_RELATIONSHIP, //117
    &ELECTRON_VOLT_JOULE_RELATIONSHIP, //118
    &ELECTRON_VOLT_KELVIN_RELATIONSHIP, //119
    &ELECTRON_VOLT_KILOGRAM_RELATIONSHIP, //120
    &ELEMENTARY_CHARGE, //121
    &ELEMENTARY_CHARGE_OVER_H_BAR, //122
    &FARADAY_CONSTANT, //123
    &FERMI_COUPLING_CONSTANT, //124
    &FINE_STRUCTURE_CONSTANT, //125
    &FIRST_RADIATION_CONSTANT, //126
    &FIRST_RADIATION_CONSTANT_FOR_SPECTRAL_RADIANCE, //127
    &HARTREE_ATOMIC_MASS_UNIT_RELATIONSHIP, //128
    &HARTREE_ELECTRON_VOLT_RELATIONSHIP, //129
    &HARTREE_ENERGY, //130
    &HARTREE_ENERGY_IN_EV, //131
    &HARTREE_HERTZ_RELATIONSHIP, //132
    &HARTREE_INVERSE_METER_RELATIONSHIP, //133
    &HARTREE_JOULE_RELATIONSHIP, //134
    &HARTREE_KELVIN_RELATIONSHIP, //135
    &HARTREE_KILOGRAM_RELATIONSHIP, //136
    &HELION_ELECTRON_MASS_RATIO, //137
    &HELION_G_FACTOR, //138
    &HELION_MAGNETIC_MOMENT, //139
    &HELION_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO, //140
    &HELION_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO, //141
    &HELION_MASS, //142
    &HELION_MASS_ENERGY_EQUIVALENT, //143
    &HELION_MASS_ENERGY_EQUIVALENT_IN_MEV, //144
    &HELION_MASS_IN_U, //145
    &HELION_MOLAR_MASS, //146
    &HELION_PROTON_MASS_RATIO, //147
    &HELION_RELATIVE_ATOMIC_MASS, //148
    &HELION_SHIELDING_SHIFT, //149
    &HERTZ_ATOMIC_MASS_UNIT_RELATIONSHIP, //150
    &HERTZ_ELECTRON_VOLT_RELATIONSHIP, //151
    &HERTZ_HARTREE_RELATIONSHIP, //152
    &HERTZ_INVERSE_METER_RELATIONSHIP, //153
    &HERTZ_JOULE_RELATIONSHIP, //154
    &HERTZ_KELVIN_RELATIONSHIP, //155
    &HERTZ_KILOGRAM_RELATIONSHIP, //156
    &HYPERFINE_TRANSITION_FREQUENCY_OF_CS_133, //157
    &INVERSE_FINE_STRUCTURE_CONSTANT, //158
    &INVERSE_METER_ATOMIC_MASS_UNIT_RELATIONSHIP, //159
    &INVERSE_METER_ELECTRON_VOLT_RELATIONSHIP, //160
    &INVERSE_METER_HARTREE_RELATIONSHIP, //161
    &INVERSE_METER_HERTZ_RELATIONSHIP, //162
    &INVERSE_METER_JOULE_RELATIONSHIP, //163
    &INVERSE_METER_KELVIN_RELATIONSHIP, //164
    &INVERSE_METER_KILOGRAM_RELATIONSHIP, //165
    &INVERSE_OF_CONDUCTANCE_QUANTUM, //166
    &JOSEPHSON_CONSTANT, //167
    &JOULE_ATOMIC_MASS_UNIT_RELATIONSHIP, //168
    &JOULE_ELECTRON_VOLT_RELATIONSHIP, //169
    &JOULE_HARTREE_RELATIONSHIP, //170
    &JOULE_HERTZ_RELATIONSHIP, //171
    &JOULE_INVERSE_METER_RELATIONSHIP, //172
    &JOULE_KELVIN_RELATIONSHIP, //173
    &JOULE_KILOGRAM_RELATIONSHIP, //174
    &KELVIN_ATOMIC_MASS_UNIT_RELATIONSHIP, //175
    &KELVIN_ELECTRON_VOLT_RELATIONSHIP, //176
    &KELVIN_HARTREE_RELATIONSHIP, //177
    &KELVIN_HERTZ_RELATIONSHIP, //178
    &KELVIN_INVERSE_METER_RELATIONSHIP, //179
    &KELVIN_JOULE_RELATIONSHIP, //180
    &KELVIN_KILOGRAM_RELATIONSHIP, //181
    &KILOGRAM_ATOMIC_MASS_UNIT_RELATIONSHIP, //182
    &KILOGRAM_ELECTRON_VOLT_RELATIONSHIP, //183
    &KILOGRAM_HARTREE_RELATIONSHIP, //184
    &KILOGRAM_HERTZ_RELATIONSHIP, //185
    &KILOGRAM_INVERSE_METER_RELATIONSHIP, //186
    &KILOGRAM_JOULE_RELATIONSHIP, //187
    &KILOGRAM_KELVIN_RELATIONSHIP, //188
    &LATTICE_PARAMETER_OF_SILICON, //189
    &LATTICE_SPACING_OF_IDEAL_SI_220, //190
    &LOSCHMIDT_CONSTANT_273_15_K_100_KPA, //191
    &LOSCHMIDT_CONSTANT_273_15_K_101_325_KPA, //192
    &LUMINOUS_EFFICACY, //193
    &MAGNETIC_FLUX_QUANTUM, //194
    &MOLAR_GAS_CONSTANT, //195
    &MOLAR_MASS_CONSTANT, //196
    &MOLAR_MASS_OF_CARBON_12, //197
    &MOLAR_PLANCK_CONSTANT, //198
    &MOLAR_VOLUME_IDEAL_GAS_273_15_K_100_KPA, //199
    &MOLAR_VOLUME_IDEAL_GAS_273_15_K_101_325_KPA, //200
    &MOLAR_VOLUME_OF_SILICON, //201
    &MOLYBDENUM_X_UNIT, //202
    &MUON_COMPTON_WAVELENGTH, //203
    &MUON_ELECTRON_MASS_RATIO, //204
    &MUON_G_FACTOR, //205
    &MUON_MAGNETIC_MOMENT, //206
    &MUON_MAGNETIC_MOMENT_ANOMALY, //207
    &MUON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO, //208
    &MUON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO, //209
    &MUON_MASS, //210
    &MUON_MASS_ENERGY_EQUIVALENT, //211
    &MUON_MASS_ENERGY_EQUIVALENT_IN_MEV, //212
    &MUON_MASS_IN_U, //213
    &MUON_MOLAR_MASS, //214
    &MUON_NEUTRON_MASS_RATIO, //215
    &MUON_PROTON_MAGNETIC_MOMENT_RATIO, //216
    &MUON_PROTON_MASS_RATIO, //217
    &MUON_TAU_MASS_RATIO, //218
    &NATURAL_UNIT_OF_ACTION, //219
    &NATURAL_UNIT_OF_ACTION_IN_EV_S, //220
    &NATURAL_UNIT_OF_ENERGY, //221
    &NATURAL_UNIT_OF_ENERGY_IN_MEV, //222
    &NATURAL_UNIT_OF_LENGTH, //223
    &NATURAL_UNIT_OF_MASS, //224
    &NATURAL_UNIT_OF_MOMENTUM, //225
    &NATURAL_UNIT_OF_MOMENTUM_IN_MEV_C, //226
    &NATURAL_UNIT_OF_TIME, //227
    &NATURAL_UNIT_OF_VELOCITY, //228
    &NEUTRON_COMPTON_WAVELENGTH, //229
    &NEUTRON_ELECTRON_MAGNETIC_MOMENT_RATIO, //230
    &NEUTRON_ELECTRON_MASS_RATIO, //231
    &NEUTRON_G_FACTOR, //232
    &NEUTRON_GYROMAGNETIC_RATIO, //233
    &NEUTRON_GYROMAGNETIC_RATIO_IN_MEGAHERTZ_PER_TESLA, //234
    &NEUTRON_MAGNETIC_MOMENT, //235
    &NEUTRON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO, //236
    &NEUTRON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO, //237
    &NEUTRON_MASS, //238
    &NEUTRON_MASS_ENERGY_EQUIVALENT, //239
    &NEUTRON_MASS_ENERGY_EQUIVALENT_IN_MEV, //240
    &NEUTRON_MASS_IN_U, //241
    &NEUTRON_MOLAR_MASS, //242
    &NEUTRON_MUON_MASS_RATIO, //243
    &NEUTRON_PROTON_MAGNETIC_MOMENT_RATIO, //244
    &NEUTRON_PROTON_MASS_DIFFERENCE, //245
    &NEUTRON_PROTON_MASS_DIFFERENCE_ENERGY_EQUIVALENT, //246
    &NEUTRON_PROTON_MASS_DIFFERENCE_ENERGY_EQUIVALENT_IN_MEV, //247
    &NEUTRON_PROTON_MASS_DIFFERENCE_IN_U, //248
    &NEUTRON_PROTON_MASS_RATIO, //249
    &NEUTRON_RELATIVE_ATOMIC_MASS, //250
    &NEUTRON_TAU_MASS_RATIO, //251
    &NEUTRON_TO_SHIELDED_PROTON_MAGNETIC_MOMENT_RATIO, //252
    &NEWTONIAN_CONSTANT_OF_GRAVITATION, //253
    &NEWTONIAN_CONSTANT_OF_GRAVITATION_OVER_H_BAR_C, //254
    &NUCLEAR_MAGNETON, //255
    &NUCLEAR_MAGNETON_IN_ELECTRON_VOLT_PER_TESLA, //256
    &NUCLEAR_MAGNETON_IN_INVERSE_METER_PER_TESLA, //257
    &NUCLEAR_MAGNETON_IN_KELVIN_PER_TESLA, //258
    &NUCLEAR_MAGNETON_IN_MEGAHERTZ_PER_TESLA, //259
    &PLANCK_CONSTANT, //260
    &PLANCK_CONSTANT_IN_EV_PER_HZ, //261
    &PLANCK_LENGTH, //262
    &PLANCK_MASS, //263
    &PLANCK_MASS_ENERGY_EQUIVALENT_IN_GEV, //264
    &PLANCK_TEMPERATURE, //265
    &PLANCK_TIME, //266
    &PROTON_CHARGE_TO_MASS_QUOTIENT, //267
    &PROTON_COMPTON_WAVELENGTH, //268
    &PROTON_ELECTRON_MASS_RATIO, //269
    &PROTON_G_FACTOR, //270
    &PROTON_GYROMAGNETIC_RATIO, //271
    &PROTON_GYROMAGNETIC_RATIO_IN_MEGAHERTZ_PER_TESLA, //272
    &PROTON_MAGNETIC_MOMENT, //273
    &PROTON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO, //274
    &PROTON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO, //275
    &PROTON_MAGNETIC_SHIELDING_CORRECTION, //276
    &PROTON_MASS, //277
    &PROTON_MASS_ENERGY_EQUIVALENT, //278
    &PROTON_MASS_ENERGY_EQUIVALENT_IN_MEV, //279
    &PROTON_MASS_IN_U, //280
    &PROTON_MOLAR_MASS, //281
    &PROTON_MUON_MASS_RATIO, //282
    &PROTON_NEUTRON_MAGNETIC_MOMENT_RATIO, //283
    &PROTON_NEUTRON_MASS_RATIO, //284
    &PROTON_RELATIVE_ATOMIC_MASS, //285
    &PROTON_RMS_CHARGE_RADIUS, //286
    &PROTON_TAU_MASS_RATIO, //287
    &QUANTUM_OF_CIRCULATION, //288
    &QUANTUM_OF_CIRCULATION_TIMES_2, //289
    &REDUCED_COMPTON_WAVELENGTH, //290
    &REDUCED_MUON_COMPTON_WAVELENGTH, //291
    &REDUCED_NEUTRON_COMPTON_WAVELENGTH, //292
    &REDUCED_PLANCK_CONSTANT, //293
    &REDUCED_PLANCK_CONSTANT_IN_EV_S, //294
    &REDUCED_PLANCK_CONSTANT_TIMES_C_IN_MEV_FM, //295
    &REDUCED_PROTON_COMPTON_WAVELENGTH, //296
    &REDUCED_TAU_COMPTON_WAVELENGTH, //297
    &RYDBERG_CONSTANT, //298
    &RYDBERG_CONSTANT_TIMES_C_IN_HZ, //299
    &RYDBERG_CONSTANT_TIMES_HC_IN_ELECTRON_VOLT, //300
    &RYDBERG_CONSTANT_TIMES_HC_IN_J, //301
    &SACKUR_TETRODE_1K_100KPA, //302
    &SACKUR_TETRODE_1K_101P325KPA, //303
    &SECOND_RADIATION_CONSTANT, //304
    &SHIELDED_HELION_GYROMAGNETIC_RATIO, //305
    &SHIELDED_HELION_GYROMAGNETIC_RATIO_IN_MEGAHERTZ_PER_TESLA, //306
    &SHIELDED_HELION_MAGNETIC_MOMENT, //307
    &SHIELDED_HELION_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO, //308
    &SHIELDED_HELION_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO, //309
    &SHIELDED_HELION_TO_PROTON_MAGNETIC_MOMENT_RATIO, //310
    &SHIELDED_HELION_TO_SHIELDED_PROTON_MAGNETIC_MOMENT_RATIO, //311
    &SHIELDED_PROTON_GYROMAGNETIC_RATIO, //312
    &SHIELDED_PROTON_GYROMAGNETIC_RATIO_IN_MEGAHERTZ_PER_TESLA, //313
    &SHIELDED_PROTON_MAGNETIC_MOMENT, //314
    &SHIELDED_PROTON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO, //315
    &SHIELDED_PROTON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO, //316
    &SHIELDING_DIFFERENCE_TO_D_AND_P_IN_HD, //317
    &SHIELDING_DIFFERENCE_OF_T_AND_P_IN_HT, //318
    &SPEED_OF_LIGHT_IN_VACUUM, //319
    &STANDARD_ACCELERATION_OF_GRAVITY, //320
    &STANDARD_ATMOSPHERE, //321
    &STANDARD_STATE_PRESSURE, //322
    &STEFAN_BOLTZMANN_CONSTANT, //323
    &TAU_COMPTON_WAVELENGTH, //324
    &TAU_ELECTRON_MASS_RATIO, //325
    &TAU_ENERGY_EQUIVALENT, //326
    &TAU_MASS, //327
    &TAU_MASS_ENERGY_EQUIVALENT, //328
    &TAU_MASS_IN_U, //329
    &TAU_MOLAR_MASS, //330
    &TAU_MUON_MASS_RATIO, //331
    &TAU_NEUTRON_MASS_RATIO, //332
    &TAU_PROTON_MASS_RATIO, //333
    &THOMSON_CROSS_SECTION, //334
    &TRITON_ELECTRON_MASS_RATIO, //335
    &TRITON_G_FACTOR, //336
    &TRITON_MAGNETIC_MOMENT, //337
    &TRITON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO, //338
    &TRITON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO, //339
    &TRITON_MASS, //340
    &TRITON_MASS_ENERGY_EQUIVALENT, //341
    &TRITON_MASS_ENERGY_EQUIVALENT_IN_MEGA_ELECTRON_VOLT, //342
    &TRITON_MASS_IN_U, //343
    &TRITON_MOLAR_MASS, //344
    &TRITON_PROTON_MASS_RATIO, //345
    &TRITON_RELATIVE_ATOMIC_MASS, //346
    &TRITON_TO_PROTON_MAGNETIC_MOMENT_RATIO, //347
    &UNIFIED_ATOMIC_MASS_UNIT, //348
    &VACUUM_ELECTRIC_PERMITTIVITY, //349
    &VACUUM_MAGNETIC_PERMEABILITY, //350
    &VON_KLITZING_CONSTANT, //351
    &WEAK_MIXING_ANGLE, //352
    &WIEN_FREQUENCY_DISPLACEMENT_LAW_CONSTANT, //353
    &WIEN_WAVELENGTH_DISPLACEMENT_LAW_CONSTANT, //354
    &W_TO_Z_MASS_RATIO, //355
];