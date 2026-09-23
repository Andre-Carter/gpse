use std::fmt;
#[derive(Debug)]
pub struct Constant {
    pub name: &'static str,
    // TODO: Revisit symbol/alias metadata once the constant API is established.
    //pub symbol: Option<&'static str>,
    pub value: f64,
    pub si_base_units: Option<&'static str>,
    pub uncertainty: Option<f64>,
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const GREEN: &str = "\x1b[32m";
        const RESET: &str = "\x1b[0m";

        writeln!(f, "{}Constant: {}{}", GREEN, self.name, RESET)?;
        writeln!(f, "Value: {}", self.value)?;

        match self.si_base_units {
            Some(units) => writeln!(f, "SI base units: {}", units)?,
            None => writeln!(f, "SI base units: Dimensionless")?,
        }

        match self.uncertainty {
            Some(uncertainty) => writeln!(f, "Uncertainty: {}", uncertainty)?,
            None => writeln!(f, "Uncertainty: Exact")?,
        }

        Ok(())
    }
}

// START OF ALPHABETICAL DATA FROM NIST https://pml.nist.gov/cuu/Constants/Table/allascii.txt
//1
pub static ALPHA_PARTICLE_ELECTRON_MASS_RATIO: Constant = Constant {
    name: "alpha particle-electron mass ratio",
    value: 7_294.299_541_71,
    si_base_units: None,
    uncertainty: Some(0.000_000_17),
};
//2
pub static ALPHA_PARTICLE_MASS: Constant = Constant {
    name: "alpha particle mass",
    value: 6.644_657_345_0e-27,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_000_002_1e-27),
};
//3
pub static ALPHA_PARTICLE_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "alpha particle mass energy equivalent",
    value: 5.971_920_199_7e-10,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_001_9e-10),
};
//4
pub static ALPHA_PARTICLE_MASS_ENERGY_EQUIVALENT_IN_MEV: Constant = Constant {
    name: "alpha particle mass energy equivalent in MeV",
    value: 3_727.379_411_8,
    si_base_units: Some("MeV"),
    uncertainty: Some(0.000_001_2),
};
//5
pub static ALPHA_PARTICLE_MASS_IN_U: Constant = Constant {
    name: "alpha particle mass in u",
    value: 4.001_506_179_129,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_000_062),
};
//6
pub static ALPHA_PARTICLE_MOLAR_MASS: Constant = Constant {
    name: "alpha particle molar mass",
    value: 4.001_506_183_3e-3,
    si_base_units: Some("kg mol^-1"),
    uncertainty: Some(0.000_000_001_2e-3),
};
//6
pub static ALPHA_PARTICLE_NEUTRON_MASS_RATIO: Constant = Constant {
    name: "alpha particle-proton mass ratio",
    value: 3.972_599_690_252,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_07),
};
//7
pub static ALPHA_PARTICLE_PROTON_MASS_RATIO: Constant = Constant {
    name: "alpha particle relative atomic mass",
    value: 4.001_506_179_129,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_062),
};
//8
pub static ALPHA_PARTICLE_RMS_CHARGE_RADIUS: Constant = Constant {
    name: "alpha particle rms charge raduis",
    value: 1.678_5e-15,
    si_base_units: Some("m"),
    uncertainty: Some(0.002_1e-15),
};
//9
pub static ANGSTROM_STAR: Constant = Constant {
    name: "Angstrom star", // A is capitalized on nist
    value: 1.000_014_95e-10,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_9e-10),
};
//10
pub static ANOMALOUS_MAGNETIC_MOMENT_OF_THE_MUON: Constant = Constant {
    name: "atomic mass constant",
    value: 1.660_539_068_92e-27,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_000_000_52e-27),
};
//11
pub static ATOMIC_MASS_CONSTANT_ENERGY_EQUIVALENT: Constant = Constant {
    name: "atomic mass constant energy equivalent",
    value: 1.492_418_087_68e-10,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_000_46e-10),
};
//12
pub static ATOMIC_MASS_CONSTANT_ENERGY_EQUIVALENT_IN_MEV: Constant = Constant {
    name: "atomic mass constant energy equivalent",
    value: 931.494_103_72,
    si_base_units: Some("MeV"),
    uncertainty: Some(0.000_000_29),
};
//13
pub static ATOMIC_MASS_UNIT_ELECTRON_VOLT_RELATIONSHIP: Constant = Constant {
    name: "atomic mass unit-electron volt relationship",
    value: 9.314_941_037_2e8,
    si_base_units: Some("eV"),
    uncertainty: Some(0.000_000_002_9e8),
};
//14
pub static ATOMIC_MASS_UNIT_HARTREE_RELATIONSHIP: Constant = Constant {
    name: "atomic mass unit-hartree relationship",
    value: 3.423_177_692_2e7,
    si_base_units: Some("E_h"),
    uncertainty: Some(0.000_000_001_1e7),
};
//15
pub static ATOMIC_MASS_UNIT_HERTZ_RELATIONSHIP: Constant = Constant {
    name: "atomic mass unit-hertz relationship",
    value: 2.252_342_721_85e23,
    si_base_units: Some("Hz"),
    uncertainty: Some(0.000_000_000_7e23),
};
//16
pub static ATOMIC_MASS_UNIT_INVERSE_METER_RELATIONSHIP: Constant = Constant {
    name: "atomic mass unit-inverse meter relationship",
    value: 7.513_006_620_9e14,
    si_base_units: Some("m^-1"),
    uncertainty: Some(0.000_000_002_3e14),
};
//17
pub static ATOMIC_MASS_UNIT_JOULE_RELATIONSHIP: Constant = Constant {
    name: "atomic mass unit-joule relationship",
    value: 1.492_418_087_68e-10,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_000_46e-10),
};
//18
pub static ATOMIC_MASS_UNIT_KELVIN_RELATIONSHIP: Constant = Constant {
    name: "atomic mass unit-kelvin relationship",
    value: 1.080_954_020_67e13,
    si_base_units: Some("K"),
    uncertainty: Some(0.000_000_000_34e13),
};
//19
pub static ATOMIC_MASS_UNIT_KILOGRAM_RELATIONSHIP: Constant = Constant {
    name: "atomic mass unit-kilogram relationship",
    value: 1.660_539_068_92e-27,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_000_000_52e-27),
};
//20
pub static ATOMIC_UNIT_OF_1ST_HYPERPOLARIZABILITY: Constant = Constant {
    name: "atomic unit of 1st hyperpolarizability",
    value: 3.206_361_299_6e-53,
    si_base_units: Some("C^3 m^3 J^-2"),
    uncertainty: Some(0.000_000_001_5e-53),
};
//21
pub static ATOMIC_UNIT_OF_2ND_HYPERPOLARIZABILITY: Constant = Constant {
    name: "atomic unit of 2nd hyperpolarizability",
    value: 6.235_379_973_5e-65,
    si_base_units: Some("C^4 m^4 J^-3"),
    uncertainty: Some(0.000_000_003_9e-65),
};
//22
pub static ATOMIC_UNIT_OF_ACTION: Constant = Constant {
    name: "atomic unit of action",
    value: 1.054_571_817e-34, //...
    si_base_units: Some("J s"),
    uncertainty: None,
};
//23
pub static ATOMIC_UNIT_OF_CHARGE: Constant = Constant {
    name: "atomic unit of charge",
    value: 1.602_176_634e-19,
    si_base_units: Some("C"),
    uncertainty: None,
};
//24
pub static ATOMIC_UNIT_OF_CHARGE_DENSITY: Constant = Constant {
    name: "atomic unit of charge density",
    value: 1.081_202_386_77e12,
    si_base_units: Some("C m^-3"),
    uncertainty: Some(0.000_000_000_51e12),
};
//25
pub static ATOMIC_UNIT_OF_CURRENT: Constant = Constant {
    name: "atomic unit of current",
    value: 6.623_618_237_508_2e-3,
    si_base_units: Some("A"),
    uncertainty: Some(0.000_000_000_007_2e-3),
};
//26
pub static ATOMIC_UNIT_OF_ELECTRIC_DIPOLE_MOMENT: Constant = Constant {
    name: "atomic unit of electic dipole mom.", //mom. is short for moment : to be decided if should use moment or follow nist mom.
    value: 8.478_353_619_8e-30,
    si_base_units: Some("C m"),
    uncertainty: Some(0.000_000_001_3e-30),
};
//27
pub static ATOMIC_UNIT_OF_ELECTRIC_FIELD: Constant = Constant {
    name: "atomic unit of electric field",
    value: 5.142_206_751_12e11,
    si_base_units: Some("V m^-1"),
    uncertainty: Some(0.000_000_000_80e11),
};
//28
pub static ATOMIC_UNIT_OF_ELECTRIC_FIELD_GRADIENT: Constant = Constant {
    name: "atomic unit electrical field gradient",
    value: 9.717_362_442_4e21,
    si_base_units: Some("V m^-2"),
    uncertainty: Some(0.000_000_003e21),
};
//29
pub static ATOMIC_UNIT_OF_ELECTRIC_POLARIZABILITY: Constant = Constant {
    name: "atomic unit of electric polarizability",
    value: 1.648_777_272_12e-41,
    si_base_units: Some("C^2 m^2 J^-1"),
    uncertainty: Some(0.000_000_000_51e-41),
};
//30
pub static ATOMIC_UNIT_OF_ELECTRIC_POTENTIAL: Constant = Constant {
    name: "atomic unit of electric potential",
    value: 27.211_386_245_981,
    si_base_units: Some("V"),
    uncertainty: Some(0.000_000_000_03),
};
//31
pub static ATOMIC_UNIT_OF_ELECTRIC_QUADRUPOLE_MOMENT: Constant = Constant {
    name: "atomic unit of electric quadrupole moment",
    value: 4.486_551_518_5e-40,
    si_base_units: Some("C m^2"),
    uncertainty: Some(0.000_000_001_4e-40),
};
//32
pub static ATOMIC_UNIT_OF_ENERGY: Constant = Constant {
    name: "atomic unit of energy",
    value: 4.359_744_722_206e-18,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_000_004_8e-18),
};
//33
pub static ATOMIC_UNIT_OF_FORCE: Constant = Constant {
    name: "atomic unit of force",
    value: 8.238_723_503_8e-8,
    si_base_units: Some("N"),
    uncertainty: Some(0.000_000_001_3e-8),
};
//34
pub static ATOMIC_UNIT_OF_LENGTH: Constant = Constant {
    name: "atomic unit of length",
    value: 5.291_722_105_44e-11,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_000_82e-11),
};
//35
pub static ATOMIC_UNIT_OF_MAGNETIC_DIPOLE_MOMENT: Constant = Constant {
    name: "atomic unit of mag. dipole mom.",
    value: 1.854_802_013_15e-23,
    si_base_units: Some("J T^-1"),
    uncertainty: Some(0.000_000_000_58e-23),
};
//36
pub static ATOMIC_UNIT_OF_MAGNETIC_FLUX_DENSITY: Constant = Constant {
    name: "atomic unit of magnetic flux density",
    value: 2.350_517_570_77e5,
    si_base_units: Some("T"),
    uncertainty: Some(0.000_000_000_73e5),
};
//37
pub static ATOMIC_UNIT_OF_MAGNETIZABILITY: Constant = Constant {
    name: "atomic unit of magnetizability",
    value: 2.350_517_570_77_e5,
    si_base_units: Some("T"),
    uncertainty: Some(0.000_000_000_73e5),
};
//38
pub static ATOMIC_UNIT_OF_MASS: Constant = Constant {
    name: "atomic unit of mass",
    value: 9.109_383_713_9e-31,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_000_002_8e-31),
};
//39
pub static ATOMIC_UNIT_OF_MOMENTUM: Constant = Constant {
    name: "atomic unit of momentum",
    value: 1.992_851_915_45e-24,
    si_base_units: Some("kg m s^-1"),
    uncertainty: Some(0.000_000_000_31e-24),
};
//40
pub static ATOMIC_UNIT_OF_PERMITTIVITY: Constant = Constant {
    name: "atomic unit of permittivity",
    value: 1.112_650_056_20e-10,
    si_base_units: Some("F m^-1"),
    uncertainty: Some(0.000_000_000_17),
};
//41
pub static ATOMIC_UNIT_OF_TIME: Constant = Constant {
    name: "atomic unit of time",
    value: 2.418_884_326_586_4e-17,
    si_base_units: Some("s"),
    uncertainty: Some(0.000_000_000_002_6e-17),
};
//42
pub static ATOMIC_UNIT_OF_VELOCITY: Constant = Constant {
    name: "atomic unit of velocity",
    value: 2.187_691_262_16e6,
    si_base_units: Some("m s^-1"),
    uncertainty: Some(0.000_000_000_34e6),
};
//43
pub static AVOGADRO_CONSTANT: Constant = Constant {
    name: "Avogadro constant", //
    value: 6.022_140_76e23,
    si_base_units: Some("mol^-1"),
    uncertainty: None,
};
//44
pub static BOHR_MAGNETON: Constant = Constant {
    name: "Bohr magneton",
    value: 9.274_010_065_7e-24,
    si_base_units: Some("J T^-1"),
    uncertainty: Some(0.000_000_002_9e-24),
};
//45
pub static BOHR_MAGNETON_IN_EV_PER_TESLA: Constant = Constant {
    name: "Bohr magneton in eV/T",
    value: 5.788_381_798_2e-5,
    si_base_units: Some("eV T^-1"),
    uncertainty: Some(0.000_000_001_8e-5),
};
//46
pub static BOHR_MAGNETON_IN_HZ_PER_TESLA: Constant = Constant {
    name: "Bohr magneton in Hz/T",
    value: 1.399_624_491_71e10,
    si_base_units: Some("Hz T^-1"),
    uncertainty: Some(0.000_000_000_44e10),
};
//47
pub static BOHR_MAGNETON_IN_INVERSE_METER_PER_TESLA: Constant = Constant {
    name: "Bohr magneton in inverse meter per tesla",
    value: 46.686_447_719,
    si_base_units: Some("m^-1 T^-1"),
    uncertainty: Some(0.000_000_015),
};
//48
pub static BOHR_MAGNETON_IN_KELVIN_PER_TESLA: Constant = Constant {
    name: "Bohr magneton in K/T",
    value: 0.671_713_814_72,
    si_base_units: Some("K T^-1"),
    uncertainty: Some(0.000_000_000_21),
};
//49
pub static BOHR_RADIUS: Constant = Constant {
    name: "Bohr radius",
    value: 5.291_772_105_44e-11,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_000_82e-11),
};
//50
pub static BOLTZMANN_CONSTANT: Constant = Constant {
    name: "Boltzmann constant",
    value: 1.380_649e-23,
    si_base_units: Some("J K^-1"),
    uncertainty: None,
};
//51
pub static BOLTZMANN_CONSTANT_IN_EV_PER_K: Constant = Constant {
    name: "Boltzmann constant in eV/K",
    value: 8.617_333_262e-5, //...
    si_base_units: Some("eV K^-1"),
    uncertainty: None,
};
//52
pub static BOLTZMANN_CONSTANT_IN_HZ_PER_K: Constant = Constant {
    name: "Boltzmann constant in Hz/K",
    value: 2.083_661_912e10, //...
    si_base_units: Some("Hz K^-1"),
    uncertainty: None,
};
//53
pub static BOLTZMANN_CONSTANT_IN_INVERSE_METER_PER_K: Constant = Constant {
    name: "Boltzmann constant in meter per kelvin",
    value: 69.503_480_04, //...
    si_base_units: Some("m^-1 K^-1"),
    uncertainty: None,
};
//54
pub static BOLTZMANN_CONSTANT_IN_KELVIN_PER_TESLA: Constant = Constant {
    name: "characteristic of impedance of vacuum",
    value: 376.730_313_412,
    si_base_units: Some("ohm"),
    uncertainty: Some(0.000_000_059),
};
//55
pub static CLASSICAL_ELECTRON_RADIUS: Constant = Constant {
    name: "classical electron radius",
    value: 2.817_940_320_5e-15,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_001_3e-15),
};
//56
pub static COMPTON_WAVELENGTH: Constant = Constant {
    name: "Compton wavelength",
    value: 2.426_310_235_38e-12,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_000_76e-12),
};
//57
pub static COMPTON_WAVELENGTH_OVER_2_PI: Constant = Constant {
    name: "conductance quantum",
    value: 7.748_091_729e-5, //...
    si_base_units: Some("S"),
    uncertainty: None,
};
//58
pub static CONDUCTANCE_QUANTUM: Constant = Constant {
    name: "conventional value of ampere-90",
    value: 1.000_000_088_87, //...
    si_base_units: Some("A"),
    uncertainty: None,
};
//59
pub static CONVENTIONAL_VALUE_OF_COULOMB_90: Constant = Constant {
    name: "conventional value of coulomb-90",
    value: 1.000_000_088_87, //...
    si_base_units: Some("C"),
    uncertainty: None,
};
//60
pub static CONVENTIONAL_VALUE_OF_FARAD_90: Constant = Constant {
    name: "convention value of farad-90",
    value: 0.999_999_982_20, //...
    si_base_units: Some("F"),
    uncertainty: None,
};
//61
pub static CONVENTIONAL_VALUE_OF_HENRY_90: Constant = Constant {
    name: "conventional value of henry-90",
    value: 1.000_000_017_79, //...
    si_base_units: Some("H"),
    uncertainty: None,
};
//62
pub static CONVENTIONAL_VALUE_OF_JOSEPHSON_CONSTANT: Constant = Constant {
    name: "conventional value of Josephson constant",
    value: 483_597.9e9,
    si_base_units: Some("Hz V^-1"),
    uncertainty: None,
};
//63
pub static CONVENTIONAL_VALUE_OF_OHM_90: Constant = Constant {
    name: "conventional value of ohm-90",
    value: 1.000_000_017_79, //...
    si_base_units: Some("ohm"),
    uncertainty: None,
};
//63
pub static CONVENTIONAL_VALUE_OF_VOLT_90: Constant = Constant {
    name: "conventional value of volt-90",
    value: 1.000_000_106_66, //...
    si_base_units: Some("V"),
    uncertainty: None,
};
//64
pub static CONVENTIONAL_VALUE_OF_VON_KLITZING_CONSTANT: Constant = Constant {
    name: "convention value of von Klitzing constant",
    value: 25_812.807,
    si_base_units: Some("ohm"),
    uncertainty: None,
};
//65
pub static CONVENTIONAL_VALUE_OF_WATT_90: Constant = Constant {
    name: "convention value of watt-90",
    value: 1.000_000_195_53, //...
    si_base_units: Some("W"),
    uncertainty: None,
};
//66
pub static COPPER_X_UNIT: Constant = Constant {
    name: "Copper x unit",
    value: 1.002_076_97e-13,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_28e-13),
};
//67
pub static DEUTERON_ELECTRON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "deuteron-electron magnetic moment ratio",
    value: -4.664_345_550e-4,
    si_base_units: None,
    uncertainty: Some(0.000_000_012e-4),
};
//68
pub static DEUTERON_ELECTRON_MASS_RATIO: Constant = Constant {
    name: "deuteron-electron mass ratio",
    value: 3_670.482_967_655,
    si_base_units: None,
    uncertainty: Some(0.000_000_063),
};
//69
pub static DEUTERON_G_FACTOR: Constant = Constant {
    name: "deuteron g factor",
    value: 0.857_438_233_5,
    si_base_units: None,
    uncertainty: Some(0.000_000_002_2),
};
//70
pub static DEUTERON_MAGNETIC_MOMENT: Constant = Constant {
    name: "deuteron magnetic moment",
    value: 4.330_735_087e-27,
    si_base_units: Some("J T^-1"),
    uncertainty: Some(0.000_000_011e-27),
};
//71
pub static DEUTERON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "deuteron magnetic moment to Bohr magneton ratio",
    value: 4.669_754_568e-4,
    si_base_units: None,
    uncertainty: Some(0.000_000_012e-4),
};
//72
pub static DEUTERON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "deuteron magnetic moment to nuclear magneton ratio",
    value: 0.857_438_233_5,
    si_base_units: None,
    uncertainty: Some(0.000_000_002_2),
};
//73
pub static DEUTERON_MASS: Constant = Constant {
    name: "deuteron mass",
    value: 3.343_583_776_8e-27,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_000_001e-27),
};
//74
pub static DEUTERON_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "deuteron mass energy equivalent",
    value: 3.005_063_234_91e-10,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_000_94),
};
//75
pub static DEUTERON_MASS_ENERGY_EQUIVALENT_IN_MEV: Constant = Constant {
    name: "deuteron mass energy equivalent in MeV",
    value: 1_875.612_945,
    si_base_units: Some("MeV"),
    uncertainty: Some(0.000_000_58),
};
//76
pub static DEUTERON_MASS_IN_U: Constant = Constant {
    name: "deuteron mass in u",
    value: 2.013_553_212_544,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_000_015),
};
//77
pub static DEUTERON_MOLAR_MASS: Constant = Constant {
    name: "deuteron molar mass",
    value: 2.013_553_214_66e-3,
    si_base_units: Some("kg mol^-1"),
    uncertainty: Some(0.000_000_000_63e-3),
};
//78
pub static DEUTERON_NEUTRON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "deuteron-neutron magnetic moment ratio",
    value: -0.448_206_52,
    si_base_units: None,
    uncertainty: Some(0.000_000_11),
};
//79
pub static DEUTERON_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "deuteron-proton magnetic moment ratio",
    value: 0.307_012_209_3,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_79),
};
//80
pub static DEUTERON_PROTON_MASS_RATIO: Constant = Constant {
    name: "deuteron-proton mass ratio",
    value: 1.999_007_501_269_9,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_008_4),
};
//81
pub static DEUTERON_RELATIVE_ATOMIC_MASS: Constant = Constant {
    name: "deuteron relative atomic mass",
    value: 2.015_533_212_544,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_015),
};
//82
pub static DEUTERON_RMS_CHARGE_RADIUS: Constant = Constant {
    name: "deuteron rms charge radius",
    value: 2.127_78e-15,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_27e-15),
};
//83
pub static ELECTRON_CHARGE_TO_MASS_QUOTIENT: Constant = Constant {
    name: "electron charge to mass quotient",
    value: -1.758_820_008_38e11,
    si_base_units: Some("C kg^-1"),
    uncertainty: Some(0.000_000_000_55e11),
};
//84
pub static ELECTRON_DEUTERON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "electron-deuteron magnetic moment ratio",
    value: -2_143.923_492_1,
    si_base_units: None,
    uncertainty: Some(0.000_005_6),
};
//85
pub static ELECTRON_DEUTERON_MASS_RATIO: Constant = Constant {
    name: "electron-deuteron mass ratio",
    value: 2.724_437_107_629e-4,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_047e-4),
};
//86
pub static ELECTRON_G_FACTOR: Constant = Constant {
    name: "electron g factor",
    value: -2.002_319_304_360_92,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_000_36),
};
//87
pub static ELECTRON_GYROMAGNETIC_RATIO: Constant = Constant {
    name: "electron gyromagnetic ratio",
    value: 1.760_859_627_84e11,
    si_base_units: Some("s^-1 T^-1"),
    uncertainty: Some(0.000_000_000_55e11),
};
//88
pub static ELECTRON_GYROMAGNETIC_RATIO_IN_MHZ_PER_TESLA: Constant = Constant {
    name: "electron gyromagnetic ratio in MHz/T",
    value: 28_024.951_386_1,
    si_base_units: Some("MHz T^-1"),
    uncertainty: Some(0.000_008_7),
};
//89
pub static ELECTRON_HELION_MASS_RATIO: Constant = Constant {
    name: "electron-helion mass ratio",
    value: 1.819_543_074_649e-4,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_053e-4),
};
//90
pub static ELECTRON_MAGNETIC_MOMENT: Constant = Constant {
    name: "electron magnetic moment",
    value: -9.284_764_691_7e-24,
    si_base_units: Some("J T^-1"),
    uncertainty: Some(0.000_000_002_9e-24),
};
//91
pub static ELECTRON_MAGNETIC_MOMENT_ANOMALY: Constant = Constant {
    name: "electron magnetic moment anomaly",
    value: 1.159_652_180_46e-3,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_18e-3),
};
//92
pub static ELECTRON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "electron magnetic moment to Bohr magneton ratio",
    value: -1.001_159_662_180_46,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_000_18),
};
//93
pub static ELECTRON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "electron magnetic moment to nuclear magneton ratio",
    value: -1_838.281_971_877,
    si_base_units: None,
    uncertainty: Some(0.000_000_032),
};
//94
pub static ELECTRON_MASS: Constant = Constant {
    name: "electron mass",
    value: 9.109_383_713_9e-31,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_000_002_8e-31),
};
//95
pub static ELECTRON_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "electron mass energy equivalent",
    value: 8.187_105_788e-14,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_002_6e-14),
};
//96
pub static ELECTRON_MASS_ENERGY_EQUIVALENT_IN_MEV: Constant = Constant {
    name: "electron mass energy equivalent in MeV",
    value: 0.510_988_950_69,
    si_base_units: Some("MeV"),
    uncertainty: Some(0.000_000_000_16),
};
//97
pub static ELECTRON_MASS_IN_U: Constant = Constant {
    name: "electron mass in u",
    value: 5.484_799_090_441e-4,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_000_097e-4),
};
//98
pub static ELECTRON_MOLAR_MASS: Constant = Constant {
    name: "electron molar mass",
    value: 5.485_799_096_2e-7,
    si_base_units: Some("kg mol^-1"),
    uncertainty: Some(0.000_000_001_7e-7),
};
//99
pub static ELECTRON_MUON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "electron-muon magnetic moment ratio",
    value: 206.766_988_1,
    si_base_units: None,
    uncertainty: Some(0.000_004_6),
};
//100
pub static ELECTRON_NEUTRON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "electron-neutron magnetic moment ratio",
    value: 960.920_48,
    si_base_units: None,
    uncertainty: Some(0.000_23),
};
//101
pub static ELECTRON_NEUTRON_MASS_RATIO: Constant = Constant {
    name: "electron-neutron mass ratio",
    value: 5.438_673_441_6e-4,
    si_base_units: None,
    uncertainty: Some(0.000_000_002_2e-4),
};
//102
pub static ELECTRON_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "electron-proton magnetic moment ratio",
    value: -658.210_687_89,
    si_base_units: None,
    uncertainty: Some(0.000_000_19),
};
//103
pub static ELECTRON_PROTON_MASS_RATIO: Constant = Constant {
    name: "electron-proton mass ratio",
    value: 5.446_170_214_889e-4,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_094e-4),
};
//104
pub static ELECTRON_RELATIVE_ATOMIC_MASS: Constant = Constant {
    name: "electron relative atomic mass",
    value: 5.486_799_090_411e-4,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_097e-4),
};
//105
pub static ELECTRON_TAU_MASS_RATIO: Constant = Constant {
    name: "electron-tau mass ratio",
    value: 2.875_85e-4,
    si_base_units: None,
    uncertainty: Some(0.000_19e-4),
};
//106
pub static ELECTRON_TO_ALPHA_PARTICLE_MASS_RATIO: Constant = Constant {
    name: "electron to alpha particle mass ratio",
    value: 1.370_933_554_733e-4,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_032e-4),
};
//107
pub static ELECTRON_TO_SHIELDED_HELION_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "electron to shielded helion magnetic moment ratio",
    value: 864.058_239_86,
    si_base_units: None,
    uncertainty: Some(0.000_000_7),
};
//108
pub static ELECTRON_TO_SHIELDED_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "electron to shielded proton magnetic moment ratio",
    value: -658.227_585_6,
    si_base_units: None,
    uncertainty: Some(0.000_002_7),
};
//109
pub static ELECTRON_TRITON_MASS_RATIO: Constant = Constant {
    name: "electron-triton mass ratio",
    value: 1.819_200_062_327e-4,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_068e-4),
};
//110
pub static ELECTRON_VOLT: Constant = Constant {
    name: "electron volt",
    value: 1.602_176_634e-19,
    si_base_units: Some("J"),
    uncertainty: None,
};
//111
pub static ELECTRON_VOLT_ATOMIC_MASS_UNIT_RELATIONSHIP: Constant = Constant {
    name: "electron volt-atomic mass unit relationship",
    value: 1.073_544_100_83e-9,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_000_33e-9),
};
//112
pub static ELECTRON_VOLT_HARTREE_RELATIONSHIP: Constant = Constant {
    name: "electron volt-hartree relationship",
    value: 3.674_932_217_566_5e-2,
    si_base_units: Some("E_h"),
    uncertainty: Some(0.000_000_000_004e-2),
};
//113
pub static ELECTRON_VOLT_HERTZ_RELATIONSHIP: Constant = Constant {
    name: "electron volt-hertz relationship",
    value: 2.417_989_242e14, //...
    si_base_units: Some("Hz"),
    uncertainty: None,
};
//114
pub static ELECTRON_VOLT_INVERSE_METER_RELATIONSHIP: Constant = Constant {
    name: "electron volt-inverse meter relationship",
    value: 8.065_543_937e5, //...
    si_base_units: Some("m^-1"),
    uncertainty: None,
};
//115
pub static ELECTRON_VOLT_JOULE_RELATIONSHIP: Constant = Constant {
    name: "electron volt-joule relationship",
    value: 1.602_176_634e-19,
    si_base_units: Some("J"),
    uncertainty: None,
};
//116
pub static ELECTRON_VOLT_KELVIN_RELATIONSHIP: Constant = Constant {
    name: "electron volt-kelvin relationship",
    value: 1.160_451_812e4, //...
    si_base_units: Some("K"),
    uncertainty: None,
};
//117
pub static ELECTRON_VOLT_KILOGRAM_RELATIONSHIP: Constant = Constant {
    name: "electron volt-kilogram relationship",
    value: 1.782_661_921e-36, //...
    si_base_units: Some("kg"),
    uncertainty: None,
};
//118
pub static ELEMENTARY_CHARGE: Constant = Constant {
    name: "elementary charge",
    value: 1.602_176_634e-19,
    si_base_units: Some("C"),
    uncertainty: None,
};
//119
pub static ELEMENTARY_CHARGE_OVER_H_BAR: Constant = Constant {
    name: "elementary charge over h-bar",
    value: 1.519_267_447e15, //...
    si_base_units: Some("A J^-1"),
    uncertainty: None,
};
//120
pub static FARADAY_CONSTANT: Constant = Constant {
    name: "Faraday constant",
    value: 96.485_332_12, //...
    si_base_units: Some("C mol^-1"),
    uncertainty: None,
};
//121
pub static FERMI_COUPLING_CONSTANT: Constant = Constant {
    name: "Fermi coupling constant",
    value: 1.166_378_7e-5,
    si_base_units: Some("GeV^-2"),
    uncertainty: Some(0.000_000_6e-5),
};
//122
pub static FINE_STRUCTURE_CONSTANT: Constant = Constant {
    name: "fine-structure constant",
    value: 7.297_352_564_3e-3,
    si_base_units: None,
    uncertainty: Some(0.000_000_001_1e-3),
};
//123
pub static FIRST_RADIATION_CONSTANT: Constant = Constant {
    name: "first radiation constant",
    value: 3.741_771_852e-16, //...
    si_base_units: Some("W m^2"),
    uncertainty: None,
};
//123
pub static FIRST_RADIATION_CONSTANT_FOR_SPECTRAL_RADIANCE: Constant = Constant {
    name: "first radiation constant for spectral radiance",
    value: 1.191_042_972e-16, //...
    si_base_units: Some("W m^2 sr^-1"),
    uncertainty: None,
};
//124
pub static HARTREE_ATOMIC_MASS_UNIT_RELATIONSHIP: Constant = Constant {
    name: "hartree-atomic mass unit relationship",
    value: 2.921_262_317_97e-8,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_000_91e-8),
};
//125
pub static HARTREE_ELECTRON_VOLT_RELATIONSHIP: Constant = Constant {
    name: "hartree-electron volt relationship",
    value: 27.211_386_245_981,
    si_base_units: Some("eV"),
    uncertainty: Some(0.000_000_000_03),
};
//126
pub static HARTREE_ENERGY: Constant = Constant {
    name: "Hartree energy",
    value: 4.359_744_722_206e-18,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_000_004_8e-18),
};
//127
pub static HARTREE_ENERGY_IN_EV: Constant = Constant {
    name: "Hartree energy in eV",
    value: 27.211_386_245_981,
    si_base_units: Some("eV"),
    uncertainty: Some(0.000_000_000_03),
};
//128
pub static HARTREE_HERTZ_RELATIONSHIP: Constant = Constant {
    name: "hartree-hertz relationship",
    value: 6.579_683_920_499_9e15,
    si_base_units: Some("Hz"),
    uncertainty: Some(0.000_000_000_007_2e15),
};
//129
pub static HARTREE_INVERSE_METER_RELATIONSHIP: Constant = Constant {
    name: "hartree inverse meter relationship",
    value: 2.194_746_313_631_4e7,
    si_base_units: Some("m^-1"),
    uncertainty: Some(0.000_000_000_002_4e7),
};
//130
pub static HARTREE_JOULE_RELATIONSHIP: Constant = Constant {
    name: "hartree-joule relationship",
    value: 4.359_744_722_206e-18,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_000_004_8e-18),
};
//131
pub static HARTREE_KELVIN_RELATIONSHIP: Constant = Constant {
    name: "hartree-kelvin relationship",
    value: 3.157_750_248_039_8e5,
    si_base_units: Some("K"),
    uncertainty: Some(0.000_000_000_003_4),
};
//132
pub static HARTREE_KILOGRAM_RELATIONSHIP: Constant = Constant {
    name: "hartree-kilogram relationship",
    value: 4.850_870_209_541_9e-35,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_000_000_005_3),
};
//133
pub static HELION_ELECTRON_MASS_RATIO: Constant = Constant {
    name: "helion-electron mass ratio",
    value: 5_495.885_279_84,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_000_16),
};
//134
pub static HELION_G_FACTOR: Constant = Constant {
    name: "helion g factor",
    value: -4.255_250_699_5,
    si_base_units: None,
    uncertainty: Some(0.000_000_003_4),
};
//135
pub static HELION_MAGNETIC_MOMENT: Constant = Constant {
    name: "helion magnetic moment",
    value: -1.074_617_551_98e-26,
    si_base_units: Some("J T^-1"),
    uncertainty: Some(0.000_000_000_93e-26),
};
//136
pub static HELION_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "helion magnetic moment to Bohr magneton ratio",
    value: -1.158_740_980_83e-3,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_94e-3),
};
//137
pub static HELION_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "helion magnetic moment to nuclear magneton ratio",
    value: -2.127_625_349_8,
    si_base_units: None,
    uncertainty: Some(0.000_000_001_7),
};
//138
pub static HELION_MASS: Constant = Constant {
    name: "helion mass",
    value: 5.006_412_786_2e-27,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_000_001_6e-27),
};
//139
pub static HELION_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "helion to mass energy equivalent",
    value: 4.499_539_418_5e-10,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_001_4e10),
};
//140
pub static HELION_MASS_ENERGY_EQUIVALENT_IN_MEV: Constant = Constant {
    name: "helion mass energy equivalent in MeV",
    value: 2_808.391_611_12,
    si_base_units: Some("MeV"),
    uncertainty: Some(0.000_000_88),
};
//141
pub static HELION_MASS_IN_U: Constant = Constant {
    name: "helion mass in u",
    value: 3.014_932_246_932,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_000_074),
};
//142
pub static HELION_MOLAR_MASS: Constant = Constant {
    name: "helion molar mass",
    value: 3.014_932_250_1e-3,
    si_base_units: Some("kg mol^-1"),
    uncertainty: Some(0.000_000_000_94e-3),
};
//143
pub static HELION_PROTON_MASS_RATIO: Constant = Constant {
    name: "helion-proton mass ratio",
    value: 2.993_152_617_552,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_07),
};
//144
pub static HELION_RELATIVE_ATOMIC_MASS: Constant = Constant {
    name: "helion relative atomic mass",
    value: 3.014_932_246_932,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_74),
};
//145
pub static HELION_SHIELDING_SHIFT: Constant = Constant {
    name: "helion shielding shift",
    value: 5.996_702_9e-5,
    si_base_units: None,
    uncertainty: Some(0.000_002_3),
};
//146
pub static HERTZ_ATOMIC_MASS_UNIT_RELATIONSHIP: Constant = Constant {
    name: "hertz-atomic mass unit relationship",
    value: 4.439_821_659e-24,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_001_4e-24),
};
//147
pub static HERTZ_ELECTRON_VOLT_RELATIONSHIP: Constant = Constant {
    name: "hertz-electron volt relationship",
    value: 4.135_667_696e-15,
    si_base_units: Some("eV"),
    uncertainty: None,
};
//148
pub static HERTZ_HARTREE_RELATIONSHIP: Constant = Constant {
    name: "hertz-hartree relationship",
    value: 1.519_829_846_057_4e-16,
    si_base_units: Some("E_h"),
    uncertainty: Some(0.000_000_000_001_7e-16),
};
//149
pub static HERTZ_INVERSE_METER_RELATIONSHIP: Constant = Constant {
    name: "hertz-inverse meter relationship",
    value: 3.335_640_951e-9,
    si_base_units: Some("m^-1"),
    uncertainty: None,
};
//150
pub static HERTZ_JOULE_RELATIONSHIP: Constant = Constant {
    name: "hertz-joule relationship",
    value: 6.626_070_15e-34,
    si_base_units: Some("J"),
    uncertainty: None,
};
//151
pub static HERTZ_KELVIN_RELATIONSHIP: Constant = Constant {
    name: "hertz-kelvin relationship",
    value: 4.799_243_073e-11, //...
    si_base_units: Some("K"),
    uncertainty: None,
};
//152
pub static HERTZ_KILOGRAM_RELATIONSHIP: Constant = Constant {
    name: "hertz-kilogram relationship",
    value: 7.372_497_323e51,
    si_base_units: Some("kg"),
    uncertainty: None,
};
//153
pub static HYPERFINE_TRANSITION_FREQUENCY_OF_CS_133: Constant = Constant {
    name: "hyperfine transition frequency of Cs-133",
    value: 9_192_631_770.0,
    si_base_units: Some("Hz"),
    uncertainty: None,
};
//154
pub static INVERSE_FINE_STRUCTURE_CONSTANT: Constant = Constant {
    name: "inverse fine-structure constant",
    value: 137.035_999_177,
    si_base_units: None,
    uncertainty: Some(0.000_000_021),
};
//155
pub static INVERSE_METER_ATOMIC_MASS_UNIT_RELATIONSHIP: Constant = Constant {
    name: "inverse meter-atomic mass unit relationship",
    value: 1.331_025_048_24e-15,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_000_41e-15),
};
//... check check check
//156
pub static INVERSE_METER_ELECTRON_VOLT_RELATIONSHIP: Constant = Constant {
    name: "inverse meter-electron volt relationship",
    value: 1.239_841_984e-6, //...
    si_base_units: Some("eV"),
    uncertainty: None,
};
//157
pub static INVERSE_METER_HARTREE_RELATIONSHIP: Constant = Constant {
    name: "inverse meter-hartree relationship",
    value: 4.556_335_252_913_2e-8, //...
    si_base_units: Some("E_h"),
    uncertainty: Some(0.000_000_000_005e-8),
};
//158
pub static INVERSE_METER_HERTZ_RELATIONSHIP: Constant = Constant {
    name: "inverse meter-hertz relationship",
    value: 299_792_458.0,
    si_base_units: Some("Hz"),
    uncertainty: None,
};
//159
pub static INVERSE_METER_JOULE_RELATIONSHIP: Constant = Constant {
    name: "inverse meter-joule relationship",
    value: 1.986_445_857e-25, //...
    si_base_units: Some("J"),
    uncertainty: None,
};
//160
pub static INVERSE_METER_KELVIN_RELATIONSHIP: Constant = Constant {
    name: "inverse meter-kelvin relatinship",
    value: 1.438_776_877e-2, //...
    si_base_units: Some("K"),
    uncertainty: None,
};
//161
pub static INVERSE_METER_KILOGRAM_RELATIONSHIP: Constant = Constant {
    name: "inverse meter-kilogram relationship",
    value: 2.210_219_094e-42,
    si_base_units: Some("kg"),
    uncertainty: None,
};
//162
pub static INVERSE_OF_CONDUCTANCE_QUANTUM: Constant = Constant {
    name: "inverse of conductance quantum",
    value: 12_906.403_72, //...
    si_base_units: Some("ohm"),
    uncertainty: None,
};
//163
pub static JOSEPHSON_CONSTANT: Constant = Constant {
    name: "Josephson constant",
    value: 483_597.848_4e9, //...
    si_base_units: Some("Hz V^-1"),
    uncertainty: None,
};
//164
pub static JOULE_ATOMIC_MASS_UNIT_RELATIONSHIP: Constant = Constant {
    name: "joule-atomic mass unit relationship",
    value: 6.700_535_247_1e9,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_002_1e9),
};
//165
pub static JOULE_ELECTRON_VOLT_RELATIONSHIP: Constant = Constant {
    name: "joule-electron volt relationship",
    value: 6.241_509_074e18, //..
    si_base_units: Some("eV"),
    uncertainty: None,
};
//166
pub static JOULE_HARTREE_RELATIONSHIP: Constant = Constant {
    name: "joule-hartree relationship",
    value: 2.293_712_278_396_9e17,
    si_base_units: Some("E_h"),
    uncertainty: Some(0.000_000_000_002_5e17),
};
//167
pub static JOULE_HERTZ_RELATIONSHIP: Constant = Constant {
    name: "joule-hertz relationship",
    value: 1.509_190_179e33, //..
    si_base_units: Some("Hz"),
    uncertainty: None,
};
//168
pub static JOULE_INVERSE_METER_RELATIONSHIP: Constant = Constant {
    name: "joule-inverse meter relationship",
    value: 5.034_116_567e24, //...
    si_base_units: Some("m^-1"),
    uncertainty: None,
};
//169
pub static JOULE_KELVIN_RELATIONSHIP: Constant = Constant {
    name: "joule-kelvin relationship",
    value: 7.242_970_516e22, //...
    si_base_units: Some("K"),
    uncertainty: None,
};
//170
pub static JOULE_KILOGRAM_RELATIONSHIP: Constant = Constant {
    name: "joule-kilogram relationship",
    value: 1.112_650_056e-17,
    si_base_units: Some("kg"),
    uncertainty: None,
};
//171
pub static KELVIN_ATOMIC_MASS_UNIT_RELATIONSHIP: Constant = Constant {
    name: "kelvin-atomic mass unit relationship",
    value: 9.251_087_288_4e-14,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_002_9e-14),
};
//172
pub static KELVIN_ELECTRON_VOLT_RELATIONSHIP: Constant = Constant {
    name: "kelvin-electron volt relationship",
    value: 8.617_333_262e-5,
    si_base_units: Some("eV"),
    uncertainty: None,
};
//173
pub static KELVIN_HARTREE_RELATIONSHIP: Constant = Constant {
    name: "kelvin-hartree relationship",
    value: 3.166_811_563_456_4e-6,
    si_base_units: Some("E_h"),
    uncertainty: Some(0.000_000_000_003_5),
};
//174
pub static KELVIN_HERTZ_RELATIONSHIP: Constant = Constant {
    name: "kelvin-hertz relationship",
    value: 2.083_661_912e10, //...
    si_base_units: Some("Hz"),
    uncertainty: None,
};
//175
pub static KELVIN_INVERSE_METER_RELATIONSHIP: Constant = Constant {
    name: "kelvin-inverse meter relationship",
    value: 69.503_480_04, //...
    si_base_units: Some("m^-1"),
    uncertainty: None,
};
//176
pub static KELVIN_JOULE_RELATIONSHIP: Constant = Constant {
    name: "kelvin-joule relationship",
    value: 1.380_649e-23,
    si_base_units: Some("J"),
    uncertainty: None,
};
//177
pub static KELVIN_KILOGRAM_RELATIONSHIP: Constant = Constant {
    name: "kelvin-kilogram relationship",
    value: 1.536_179_187e-40, //...
    si_base_units: Some("kg"),
    uncertainty: None,
};
//178
pub static KILOGRAM_ATOMIC_MASS_UNIT_RELATIONSHIP: Constant = Constant {
    name: "kilogram-atomic mass unit relationship",
    value: 6.002_140_757_7e26,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_001_9e26),
};
//179
pub static KILOGRAM_ELECTRON_VOLT_RELATIONSHIP: Constant = Constant {
    name: "kilogram-electron volt relationship",
    value: 5.609_588_603e35, //...
    si_base_units: Some("eV"),
    uncertainty: None,
};
//180
pub static KILOGRAM_HARTREE_RELATIONSHIP: Constant = Constant {
    name: "kilogram-hartree relationship",
    value: 2.061_458_788_741_5e34,
    si_base_units: Some("E_h"),
    uncertainty: Some(0.000_000_000_002_2e34),
};
//181
pub static KILOGRAM_HERTZ_RELATIONSHIP: Constant = Constant {
    name: "kilogram-hertz relationship",
    value: 1.356_392_489e50, //...
    si_base_units: Some("Hz"),
    uncertainty: None,
};
//182
pub static KILOGRAM_INVERSE_METER_RELATIONSHIP: Constant = Constant {
    name: "kilogram-inverse meter relationship",
    value: 4.524_438_335e41, //...
    si_base_units: Some("m^-1"),
    uncertainty: None,
};
//183
pub static KILOGRAM_JOULE_RELATIONSHIP: Constant = Constant {
    name: "kilogram-joule relationship",
    value: 8.987_551_787e16, //...
    si_base_units: Some("J"),
    uncertainty: None,
};
//184
pub static KILOGRAM_KELVIN_RELATIONSHIP: Constant = Constant {
    name: "kilogram-kelvin relationship",
    value: 6.509_657_26e39, //...
    si_base_units: Some("K"),
    uncertainty: None,
};
//185
pub static LATTICE_PARAMETER_OF_SILICON: Constant = Constant {
    name: "lattice parameter of silicon",
    value: 6.431_020_511e-10,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_089e-10),
};
//186
pub static LATTICE_SPACING_OF_IDEAL_SI_220: Constant = Constant {
    name: "lattice spacing of ideal Si (220)",
    value: 1.920_155_716_716e-10,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_032e-10),
};
//187
pub static LOSCHMIDT_CONSTANT_273_15_K_100_KPA: Constant = Constant {
    name: "Loschmidt constant (273.15 K, 100 kPa)",
    value: 2.651_645_804e25, //...
    si_base_units: Some("m^-3"),
    uncertainty: None,
};
//188
pub static LOSCHMIDT_CONSTANT_273_15_K_101_325_KPA: Constant = Constant {
    name: "Loschmidt constant (273.15 K, 101.325 kPa)",
    value: 2.686_780_111e25, //...
    si_base_units: Some("m^-3"),
    uncertainty: None,
};
//189
pub static LUMINOUS_EFFICACY: Constant = Constant {
    name: "luminous efficacy",
    value: 683.0,
    si_base_units: Some("lm W^-1"),
    uncertainty: None,
};
//190
pub static MAGNETIC_FLUX_QUANTUM: Constant = Constant {
    name: "magnetic flux quantum",
    value: 2.067_833_848e-15, //...
    si_base_units: Some("Wb"),
    uncertainty: None,
};
//191
pub static MOLAR_GAS_CONSTANT: Constant = Constant {
    name: "molar gas constant",
    value: 8.314_462_618, //...
    si_base_units: Some("J mol^-1 K^-1"),
    uncertainty: None,
};
//192
pub static MOLAR_MASS_CONSTANT: Constant = Constant {
    name: "molar mass constant",
    value: 1.000_000_001_05e-3,
    si_base_units: Some("kg mol^-1"),
    uncertainty: Some(0.000_000_000_31e-3),
};
//193
pub static MOLAR_MASS_OF_CARBON_12: Constant = Constant {
    name: "molar mass of carbon-12",
    value: 12.000_000_012_6e-3,
    si_base_units: Some("kg mol^-1"),
    uncertainty: Some(0.000_000_003_7e-3),
};
//194
pub static MOLAR_PLANCK_CONSTANT: Constant = Constant {
    name: "molar Planck constant",
    value: 3.990_312_712e-10, //...
    si_base_units: Some("J Hz^-1 mol^-1"),
    uncertainty: None,
};
//195
pub static MOLAR_VOLUME_IDEAL_GAS_273_15_K_100_KPA: Constant = Constant {
    name: "molar volume ideal gas (273.15 K, 100 kPa)",
    value: 22.710_954_64e-3,
    si_base_units: Some("m^3 mol^-1"),
    uncertainty: None,
};
//196
pub static MOLAR_VOLUME_IDEAL_GAS_273_13_K_101_325_KPA: Constant = Constant {
    name: "molar volume of ideal gas (273.13 K, 101.325 kPa)",
    value: 22.413_969_54e-3,
    si_base_units: Some("m^3 mol^-1"),
    uncertainty: None,
};
//197
pub static MOLAR_VOLUME_OF_SILICON: Constant = Constant {
    name: "molar volume of silicon",
    value: 1.205_883_199e-5,
    si_base_units: Some("m^3 mol^-1"),
    uncertainty: Some(0.000_000_06e-5),
};
//198
pub static MOLYBDENUM_X_UNIT: Constant = Constant {
    name: "Molybdenum x unit",
    value: 1.002_009_52e-13,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_53e-13),
};
//199
pub static MUON_COMPTON_WAVELENGTH: Constant = Constant {
    name: "muon Compton wavelength",
    value: 1.173_444_110e-14,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_026),
};
//200
pub static MUON_ELECTRON_MASS_RATIO: Constant = Constant {
    name: "muon-electron mass ratio",
    value: 206.768_287_7,
    si_base_units: None,
    uncertainty: Some(0.000_004_6),
};
//201
pub static MUON_G_FACTOR: Constant = Constant {
    name: "muon g factor",
    value: -2.002_331_841_23,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_82),
};
//202
pub static MUON_MAGNETIC_MOMENT: Constant = Constant {
    name: "muon magnetic moment",
    value: -4.490_448_3e-26,
    si_base_units: Some("J T^-1"),
    uncertainty: Some(0.000_000_1e-26),
};
//203
pub static MUON_MAGNETIC_MOMENT_ANOMALY: Constant = Constant {
    name: "muon magnetic moment anomaly",
    value: 1.165_920_62e-3,
    si_base_units: None,
    uncertainty: Some(0.000_000_41e-3),
};
//204
//oops we had a dup, so no we gotta renumber, goodnews no syntax error tho, perhaps
//205
pub static MUON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "muon magnetic moment to Bohr magneton ratio",
    value: -4.841_970_48e-3,
    si_base_units: None,
    uncertainty: Some(0.000_000_11e-3),
};
//206
pub static MUON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "muon magnetic moment to nuclear magneton ratio",
    value: -8.890_597_04,
    si_base_units: None,
    uncertainty: Some(0.000_000_2),
};
//207
pub static MUON_MASS: Constant = Constant {
    name: "muon mass",
    value: 1.883_531_627e-28,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_000_42e-28),
};
//208
pub static MUON_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "muon mass energy equivalent",
    value: 1.692_833_804e-11,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_038e-11),
};
//209
pub static MUON_MASS_IN_U: Constant = Constant {
    name: "muon mass in u",
    value: 0.113_428_925_7,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_002_5),
};
//210
pub static MUON_MOLAR_MASS: Constant = Constant {
    name: "muon molar mass",
    value: 1.134_289_258e-4,
    si_base_units: Some("kg mol^-1"),
    uncertainty: Some(0.000_000_025e-4),
};
//211
pub static MUON_NEUTRON_MASS_RATIO: Constant = Constant {
    name: "muon-neutron mass ratio",
    value: 0.112_454_516_8,
    si_base_units: None,
    uncertainty: Some(0.000_000_002_5),
};
//212
pub static MUON_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "muon-proton magnetic moment ratio",
    value: -3.183_345_146,
    si_base_units: None,
    uncertainty: Some(0.000_000_071),
};
//213
pub static MUON_PROTON_MASS_RATIO: Constant = Constant {
    name: "muon-proton mass ratio",
    value: 0.112_609_526_2,
    si_base_units: None,
    uncertainty: Some(0.000_000_002_5),
};
//214
pub static MUON_TAU_MASS_RATIO: Constant = Constant {
    name: "muon-tau mass ratio",
    value: 5.946_35e-2,
    si_base_units: None,
    uncertainty: Some(0.000_4e-2),
};
//215
pub static NATURAL_UNIT_OF_ACTION: Constant = Constant {
    name: "natural unit of action",
    value: 1.054_571_817e-34, //...
    si_base_units: Some("J s"),
    uncertainty: None,
};
//216
pub static NATURAL_UNIT_OF_ACTION_IN_EV_S: Constant = Constant {
    name: "natural unit of action in eV s",
    value: 6.582_119_569e-16,
    si_base_units: Some("eV s"),
    uncertainty: None,
};
//217
pub static NATURAL_UNIT_OF_ENERGY: Constant = Constant {
    name: "natural unit of energy",
    value: 8.187_105_788e-14,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_002_6e-14),
};
//218
pub static NATURAL_UNIT_OF_ENERGY_IN_MEV: Constant = Constant {
    name: "natural unit of energy in MeV",
    value: 0.510_988_950_69,
    si_base_units: Some("MeV"),
    uncertainty: Some(0.000_000_000_16),
};
//219
pub static NATURAL_UNIT_OF_LENGTH: Constant = Constant {
    name: "natural unit of length",
    value: 3.861_592_674_4e-13,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_001_2e-13),
};
//220
pub static NATURAL_UNIT_OF_MASS: Constant = Constant {
    name: "natural unit of mass",
    value: 9.109_383_713_9e-13,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_000_002_8e-31),
};
//221
pub static NATURAL_UNIT_OF_MOMENTUM: Constant = Constant {
    name: "natural unit of momentum",
    value: 2.730_924_534_46e-22,
    si_base_units: Some("kg m s^-1"),
    uncertainty: Some(0.000_000_000_85e-22),
};
//222
pub static NATURAL_UNIT_OF_MOMENTUM_IN_MEV_C: Constant = Constant {
    name: "natural unit momentum in MeV/c",
    value: 0.510_998_950_69,
    si_base_units: Some("MeV/c"),
    uncertainty: Some(0.000_000_000_16),
};
//223
pub static NATURAL_UNIT_OF_TIME: Constant = Constant {
    name: "natural unit of time",
    value: 1.288_088_666_44e-21,
    si_base_units: Some("s"),
    uncertainty: Some(0.000_000_000_4e-21),
};
//224
pub static NATURAL_UNIT_OF_VELOCITY: Constant = Constant {
    name: "natural unit of velocity",
    value: 299_792_458.0,
    si_base_units: Some("m s^-1"),
    uncertainty: None,
};
//225
pub static NEUTRON_COMPTON_WAVELENGTH: Constant = Constant {
    name: "neutron Compton wavelength",
    value: 1.319_590_903_82e-15,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_000_67e-15),
};
//226
pub static NEUTRON_ELECTRON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "neutron-electron magnetic moment ratio",
    value: 1.040_668_84e-3,
    si_base_units: None,
    uncertainty: Some(0.000_000_24e-3),
};
//227
pub static NEUTRON_ELECTRON_MASS_RATIO: Constant = Constant {
    name: "neutron-electron mass ratio",
    value: 1_838.683_662,
    si_base_units: None,
    uncertainty: Some(0.000_000_74),
};
//228
pub static NEUTRON_G_FACTOR: Constant = Constant {
    name: "neutron g factor",
    value: -3.826_085_52,
    si_base_units: None,
    uncertainty: Some(0.000_000_9),
};
//229
pub static NEUTRON_GYROMAGNETIC_RATIO: Constant = Constant {
    name: "neutron gyromagnetic ratio",
    value: 1.832_471_74e8,
    si_base_units: Some("s^-1 T^-1"),
    uncertainty: Some(0.000_000_43e8),
};
//230
pub static NEUTRON_GYROMAGNETIC_RATIO_IN_MHZ_T: Constant = Constant {
    name: "neutron gyromagnetic ratio in MHz/T",
    value: 29.164_693_5,
    si_base_units: Some("MHz T^-1"),
    uncertainty: Some(0.000_006_9),
};
//231
pub static NEUTRON_MAGNETIC_MOMENT: Constant = Constant {
    name: "neutron magnetic moment",
    value: -9.662_365_3e-27,
    si_base_units: Some("J T^-1"),
    uncertainty: Some(0.000_002_3e-27),
};
//232
pub static NEUTRON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "neutron magnetic moment to Bohr magneton ratio",
    value: -1.041_875_65e-3,
    si_base_units: None,
    uncertainty: Some(0.000_000_25e-3),
};
//233
pub static NEUTRON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "neutron magnetic moment to nuclear magneton ratio",
    value: -1.913_042_76,
    si_base_units: None,
    uncertainty: Some(0.000_000_45),
};
//234
pub static NEUTRON_MASS: Constant = Constant {
    name: "neutron mass",
    value: 1.674_927_500_56e-27,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_000_000_85e-27),
};
//235
pub static NEUTRON_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "neutron mass energy equivalent",
    value: 1.505_349_765_14e-10,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_000_76e-10),
};
//236
pub static NEUTRON_MASS_ENERGY_EQUIVALENT_IN_MEV: Constant = Constant {
    name: "neutron mass energy equivalent in MeV",
    value: 939.565_421_94,
    si_base_units: Some("MeV"),
    uncertainty: Some(0.000_000_48),
};
//237
pub static NEUTRON_MASS_IN_U: Constant = Constant {
    name: "neutron mass in u",
    value: 1.008_664_916_06,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_000_4),
};
//238
pub static NEUTRON_MOLAR_MASS: Constant = Constant {
    name: "neutron molar mass",
    value: 1.008_664_917_12e-3,
    si_base_units: Some("kg mol^-1"),
    uncertainty: Some(0.000_000_000_51e-3),
};
//239
pub static NEUTRON_MUON_MASS_RATIO: Constant = Constant {
    name: "neutron-muon mass ratio",
    value: 8.892_484_08,
    si_base_units: None,
    uncertainty: Some(0.000_000_2),
};
//240
pub static NEUTRON_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "neutron-proton magnetic moment ratio",
    value: -0.684_979_35,
    si_base_units: None,
    uncertainty: Some(0.000_000_16),
};
//241
pub static NEUTRON_PROTON_MASS_DIFFERENCE: Constant = Constant {
    name: "neutron-proton mass difference",
    value: 2.305_574_61e-30,
    si_base_units: None,
    uncertainty: Some(0.000_000_67e-30),
};
//242
pub static NEUTRON_PROTON_MASS_DIFFERENCE_ENERGY_EQUIVALENT: Constant = Constant {
    name: "neutron-proton mass difference energy equivalent",
    value: 2.072_147_12e-13,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_6e-13),
};
//243
pub static NEUTRON_PROTON_MASS_DIFFERENCE_ENERGY_EQUIVALENT_IN_MEV: Constant = Constant {
    name: "neutron-proton mass difference energy equivalent in MeV",
    value: 1.293_332_51,
    si_base_units: Some("MeV"),
    uncertainty: Some(0.000_000_38),
};
//244
pub static NEUTRON_PROTON_MASS_DIFFERENCE_IN_U: Constant = Constant {
    name: "neutron-proton mass difference in u",
    value: 1.388_449_48e-3,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_4e-3),
};
//245
pub static NEUTRON_PROTON_MASS_RATIO: Constant = Constant {
    name: "neutron-proton mass ratio",
    value: 1.001_378_419_46,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_4),
};
//246
pub static NEUTRON_RELATIVE_ATOMIC_MASS: Constant = Constant {
    name: "neutron relative atomic mass",
    value: 1.008_664_916_06,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_4),
};
//247
pub static NEUTRON_TAU_MASS_RATIO: Constant = Constant {
    name: "neutron-tau mass ratio",
    value: 0.528_799,
    si_base_units: None,
    uncertainty: Some(0.000_036),
};
//248
pub static NEUTRON_TO_SHIELDED_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "neutron to shielded proton magnetic moment ratio",
    value: -0.684_996_94,
    si_base_units: None,
    uncertainty: Some(0.000_000_16),
};
//249
pub static NEWTONIAN_CONSTANT_OF_GRAVITATION: Constant = Constant {
    name: "Newtonian constant of gravitation",
    value: 6.674_3e-11,
    si_base_units: Some("m^3 kg^-1 s^-2"),
    uncertainty: Some(0.000_15e-11),
};
//250
pub static NEWTONIAN_CONSTANT_OF_GRAVITATION_OVER_H_BAR_C: Constant = Constant {
    name: "Newtonian constant of gravitation over h-bar c",
    value: 6.708_83e-39,
    si_base_units: Some("(GeV/c^2)^2"),
    uncertainty: Some(0.000_15e-39),
};
//251
pub static NUCLEAR_MAGNETON: Constant = Constant {
    name: "nuclear magneton",
    value: 5.050_783_739_3e-27,
    si_base_units: Some("J T^-1"),
    uncertainty: Some(0.000_000_001_6e-27),
};
//252
pub static NUCLEAR_MAGNETON_IN_ELECTRON_VOLT_PER_TESLA: Constant = Constant {
    name: "nuclear magneton in eV/T",
    value: 3.152_451_254_17e-8,
    si_base_units: Some("eV T^-1"),
    uncertainty: Some(0.000_000_000_98e-8),
};
//253
pub static NUCLEAR_MAGNETON_IN_INVERSE_METER_PER_TESLA: Constant = Constant {
    name: "nuclear magneton in inverse meter per tesla",
    value: 2.542_623_410_09e-2,
    si_base_units: Some("m^-1 T^-1"),
    uncertainty: Some(0.000_000_000_79e-2),
};
//254
pub static NUCLEAR_MAGNETON_IN_KELVIN_PER_TESLA: Constant = Constant {
    name: "nuclear magneton in K/T",
    value: 3.658_267_770_6e-4,
    si_base_units: Some("K T^-1"),
    uncertainty: Some(0.000_000_001_1e-4),
};
//255
pub static NUCLEAR_MAGNETON_IN_MEGAHERTZ_PER_TESLA: Constant = Constant {
    name: "nuclear magneton in MHz/T",
    value: 7.622_593_218_8,
    si_base_units: Some("MHz T^-1"),
    uncertainty: Some(0.000_000_002_4),
};
//256
pub static PLANCK_CONSTANT: Constant = Constant {
    name: "Planck constant",
    value: 6.626_070_15e-34,
    si_base_units: Some("J Hz^-1"),
    uncertainty: None,
};
//257
pub static PLANCK_CONSTANT_IN_EV_PER_HZ: Constant = Constant {
    name: "Planck constant in eV/Hz",
    value: 4.135_667_696e-15, //...
    si_base_units: Some("eV Hz^-1"),
    uncertainty: None,
};
//258
pub static PLANCK_LENGTH: Constant = Constant {
    name: "Planck length",
    value: 1.616_255e-35,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_018e-35),
};
//259
pub static PLANCK_MASS: Constant = Constant {
    name: "Planck mass",
    value: 2.176_434e-8,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_024e-8),
};
//260
pub static PLANCK_MASS_ENERGY_EQUIVALENT_IN_GEV: Constant = Constant {
    name: "Planck mass energy equivalent in GeV",
    value: 1.220_890e19,
    si_base_units: Some("GeV"),
    uncertainty: Some(0.000_014e19),
};
//261
pub static PLANCK_TEMPERATURE: Constant = Constant {
    name: "Planck temperature",
    value: 1.416_784e32,
    si_base_units: Some("K"),
    uncertainty: Some(0.000_016e32),
};
//262
pub static PLANCK_TIME: Constant = Constant {
    name: "Planck time",
    value: 5.391_247e-44,
    si_base_units: Some("s"),
    uncertainty: Some(0.000_06e-44),
};
//263
pub static PROTON_CHARGE_TO_MASS_QUOTIENT: Constant = Constant {
    name: "proton charge to mass quotient",
    value: 9.578_833_143e7,
    si_base_units: Some("C kg^-1"),
    uncertainty: Some(0.000_000_003e7),
};
//264
pub static PROTON_COMPTON_WAVELENGTH: Constant = Constant {
    name: "proton Compton wavelength",
    value: 1.321_409_853_6e-15,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_000_41e-15),
};
//265
pub static PROTON_ELECTRON_MASS_RATIO: Constant = Constant {
    name: "proton-electron mass ratio",
    value: 1_836.152_673_426,
    si_base_units: None,
    uncertainty: Some(0.000_000_032),
};
//266
pub static PROTON_G_FACTOR: Constant = Constant {
    name: "proton g factor",
    value: 5.585_694_689_3,
    si_base_units: None,
    uncertainty: Some(0.000_000_001_6),
};
//267
pub static PROTON_GYROMAGNETIC_RATIO: Constant = Constant {
    name: "proton gyromagnetic ratio",
    value: 2.675_221_870_8e8,
    si_base_units: Some("s^-1 T^-1"),
    uncertainty: Some(0.000_000_001_1e8),
};
//268
pub static PROTON_GYROMAGNETIC_RATIO_IN_MHZ_PER_T: Constant = Constant {
    name: "proton gyromagnetic ratio in MHz/T",
    value: 42.577_478_461,
    si_base_units: Some("MHz T^-1"),
    uncertainty: Some(0.000_000_018),
};
//269
pub static PROTON_MAGNETIC_MOMENT: Constant = Constant {
    name: "proton magnetic moment",
    value: 1.410_606_795_45e-26,
    si_base_units: Some("J T^-1"),
    uncertainty: Some(0.000_000_000_6e-26),
};
//270
pub static PROTON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "proton magnetic moment to Bohr magneton ratio",
    value: 1.521_032_202_3e-3,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_45e-3),
};
//271
pub static PROTON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "proton magnetic moment to nuclear magneton ratio",
    value: 2.792_847_344_63,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_82),
};
//272
pub static PROTON_MAGNETIC_SHIELDING_CORRECTION: Constant = Constant {
    name: "proton magnetic shielding correction",
    value: 2.567_15e-5,
    si_base_units: None,
    uncertainty: Some(0.000_41e-5),
};
//273
pub static PROTON_MASS: Constant = Constant {
    name: "proton mass",
    value: 1.672_621_925_95e-27,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_000_000_52e-27),
};
//274
pub static PROTON_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "proton mass energy equivalent",
    value: 1.503_277_618_02e-10,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_000_47),
};
//275
pub static PROTON_MASS_ENERGY_EQUIVALENT_IN_MEV: Constant = Constant {
    name: "proton mass energy equivalent in MeV",
    value: 938.272_089_43,
    si_base_units: Some("MeV"),
    uncertainty: Some(0.000_000_29),
};
//276
pub static PROTON_MASS_IN_U: Constant = Constant {
    name: "proton mass in u",
    value: 1.007_276_466_578_9,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_000_008_3),
};
//277
pub static PROTON_MOLAR_MASS: Constant = Constant {
    name: "proton molar mass",
    value: 1.007_276_467_64e-3,
    si_base_units: Some("kg mol^-1"),
    uncertainty: Some(0.000_000_000_31e-3),
};
//278
pub static PROTON_MUON_MASS_RATIO: Constant = Constant {
    name: "proton-muon mass ratio",
    value: 8.880_243_38,
    si_base_units: None,
    uncertainty: Some(0.000_000_2),
};
//279
pub static PROTON_NEUTRON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "proton-neutron magnetic moment ratio",
    value: -1.459_898_02,
    si_base_units: None,
    uncertainty: Some(0.000_000_34),
};
//280
pub static PROTON_NEUTRON_MASS_RATIO: Constant = Constant {
    name: "proton-neutron mass ratio",
    value: 0.998_623_477_97,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_4),
};
//proton neutron mass difference
//281
pub static PROTON_RELATIVE_ATOMIC_MASS: Constant = Constant {
    name: "proton relative atomic mass",
    value: 1.007_276_466_578_9,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_008_3),
};
//282
pub static PROTON_RMS_CHARGE_RADIUS: Constant = Constant {
    name: "proton rms charge radius",
    value: 8.407_5e-16,
    si_base_units: Some("m"),
    uncertainty: Some(0.006_4e-16),
};
//283
pub static PROTON_TAU_MASS_RATIO: Constant = Constant {
    name: "proton-tau mass ratio",
    value: 0.528_051,
    si_base_units: None,
    uncertainty: Some(0.000_036),
};
//284
pub static QUANTUM_OF_CIRCULATION: Constant = Constant {
    name: "quantum of circulation",
    value: 3.636_947_546_7e-4,
    si_base_units: Some("m^2 s^-1"),
    uncertainty: Some(0.000_000_001_1e-4),
};
//285
pub static QUANTUM_OF_CIRCULATION_TIMES_2: Constant = Constant {
    name: "quantum of circluation times 2",
    value: 7.273_895_093_4e-4,
    si_base_units: Some("m^2 s^-1"),
    uncertainty: Some(0.000_000_002_3e-4),
};
//286
pub static REDUCED_COMPTON_WAVELENGTH: Constant = Constant {
    name: "reduced Compton wavelength",
    value: 3.861_592_674_4e-13,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_001_2e-13),
};
//287
pub static REDUCED_ELECTRON_COMPTON_WAVELENGTH: Constant = Constant {
    name: "reduced muon Compton wavelength",
    value: 1.867_594_306e-15,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_042e-15),
};
//288
pub static REDUCED_NEUTRON_COMPTON_WAVELENGTH: Constant = Constant {
    name: "reduced neutron Compton wavelength",
    value: 2.100_194_152e-16,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_001_1e-16),
};
//289
pub static REDUCED_PLANCK_CONSTANT: Constant = Constant {
    name: "reduced Planck constant",
    value: 1.054_571_817e-34, //...
    si_base_units: Some("J s"),
    uncertainty: None,
};
//290
pub static REDUCED_PLANCK_CONSTANT_IN_EV_S: Constant = Constant {
    name: "reduced Planck constant in eV s",
    value: 6.582_119_569e-16, //...
    si_base_units: Some("eV s"),
    uncertainty: None,
};
//291
pub static REDUCED_PLANCK_CONSTANT_TIMES_C_IN_MEV_FM: Constant = Constant {
    name: "reduced Planck constant times c in MeV fm",
    value: 197.326_980_4,
    si_base_units: Some("MeV fm"),
    uncertainty: None,
};
//292
pub static REDUCED_PROTON_COMPTON_WAVELENGTH: Constant = Constant {
    name: "reduced proton Compton wavelength",
    value: 2.103_089_100_51e-16,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_000_000_66e-16),
};
//293
pub static REDUCED_TAU_COMPTON_WAVELENGTH: Constant = Constant {
    name: "reduced tau Compton wavelength",
    value: 1.110_538e-16,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_075e-16),
};
//294
pub static RYDBERG_CONSTANT: Constant = Constant {
    name: "Rydberg constant",
    value: 10_973_731.568_157,
    si_base_units: Some("m^-1"),
    uncertainty: Some(0.000_012),
};
//295
pub static RYDBERG_CONSTANT_TIMES_C_IN_HZ: Constant = Constant {
    name: "Rydberg constant times c in Hz",
    value: 3.289_841_960_25e15,
    si_base_units: Some("Hz"),
    uncertainty: Some(0.000_000_000_003_6e15),
};
//296
pub static RYDBERG_CONSTANT_TIMES_HC_IN_EV: Constant = Constant {
    name: "Rydberg constant times hc in eV",
    value: 13.605_693_122_99,
    si_base_units: Some("eV"),
    uncertainty: Some(0.000_000_000_015),
};
//297
pub static RYDBERG_CONSTANT_TIMES_HC_IN_J: Constant = Constant {
    name: "Rydberg constant times hc in J",
    value: 2.179_872_361_103e-18,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_000_002_4e-18),
};
//298
pub static SACKUR_TETRODE_1K_100KPA: Constant = Constant {
    name: "Sackur-Tetrode (1 K, 100kPa)",
    value: -1.151_707_534_96,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_47),
};
//299
pub static SACKUR_TETRODE_1K_101P325KPA: Constant = Constant {
    name: "Sackur-Tetrode (1 K, 101.325 kPa)",
    value: -1.164_870_521_49,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_47),
};
//300
pub static SECOND_RADATION_CONSTANT: Constant = Constant {
    name: "second radiation constant",
    value: 1.438_776_877e-2, //...
    si_base_units: Some("m K"),
    uncertainty: None,
};
//301
pub static SHIELDED_HELION_GYROMAGNETIC_RATIO: Constant = Constant {
    name: "shielded helion gyromagnetic ratio",
    value: 2.037_894_607_8e8,
    si_base_units: Some("s^-1 T^-1"),
    uncertainty: Some(0.000_000_001_8e8),
};
//302
pub static SHIELDED_HELION_GYROMAGNETIC_RATIO_IN_MHZ_T: Constant = Constant {
    name: "shielded helion gyromagnetic ratio in MHz/T",
    value: 32.434_100_033,
    si_base_units: Some("MHz T^-1"),
    uncertainty: Some(0.000_000_028),
};
//303
pub static SHIELDED_HELION_MAGNETIC_MOMENT: Constant = Constant {
    name: "shielded helion magnetic moment",
    value: -1.074_533_110_35e-26,
    si_base_units: Some("J T^-1"),
    uncertainty: Some(0.000_000_000_93e-26),
};
//304
pub static SHIELDED_HELION_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATIO: Constant = Constant {
    name: "shielded helion mag. mom to Bohr magneton ratio",
    value: -1.158_671_494_57e-3,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_94e-3),
};
//305
pub static SHIELDED_HELION_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "shielded helion magnetic moment to nuclear magneton ratio",
    value: -2.117_497_762_4,
    si_base_units: None,
    uncertainty: Some(0.000_000_001_7),
};
//306
pub static SHIELDED_HELION_TO_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "shielded helion to proton magnetic moment ratio",
    value: -0.761_766_577_21,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_66),
};
//307
pub static SHIELDED_HELION_TO_SHIELDED_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "shielded helion to shielded proton magnetic moment ratio",
    value: -0.761_786_133_4,
    si_base_units: None,
    uncertainty: Some(0.000_000_003_1),
};
//308
pub static SHIELDED_PROTON_GYROMAGNETIC_RATIO: Constant = Constant {
    name: "shielded proton gyromagnetic ratio",
    value: 2.675_153_194e8,
    si_base_units: Some("s^-1 T^-1"),
    uncertainty: Some(0.000_000_011e8),
};
//309
pub static SHIELDED_PROTON_GYROMAGNETIC_RATIO_IN_MHZ_T: Constant = Constant {
    name: "shielded proton gyromagnetic ratio in MHz/T",
    value: 42.576_385_43,
    si_base_units: Some("MHz T^-1"),
    uncertainty: Some(0.000_000_17),
};
//310
pub static SHIELDED_PROTON_MAGNETIC_MOMENT: Constant = Constant {
    name: "shielded proton magnetic moment",
    value: 1.410_570_583e-26,
    si_base_units: Some("J T^-1"),
    uncertainty: Some(0.000_000_005_8e-26),
};
//311
pub static SHIELDED_PROTON_MAGNETIC_MOMENT_TO_BOHR_MAGNETON_RATION: Constant = Constant {
    name: "shielded proton magnetic moment to Bohr magneton ratio",
    value: 1.520_993_155_1e-3,
    si_base_units: None,
    uncertainty: Some(0.000_000_006_2e-3),
};
//312
pub static SHIELDED_PROTON_MAGNETIC_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "shielded proton mag. mom. to nuclear magneton ratio",
    value: 2.792_755_648,
    si_base_units: None,
    uncertainty: Some(0.000_000_011),
};
//313
pub static SHIELDING_DIFFERENCE_TO_D_AND_P_IN_HD: Constant = Constant {
    name: "shielding difference to d and p in HD",
    value: 1.987_70e-8,
    si_base_units: None,
    uncertainty: Some(0.000_1e-8),
};
//314
pub static SHIELDING_DIFFERENCE_OF_T_AND_P_IN_HT: Constant = Constant {
    name: "shielding difference of t and p in HT",
    value: 2.394_5e-8,
    si_base_units: None,
    uncertainty: Some(0.000_2e-8),
};
//315
pub static SPEED_OF_LIGHT_IN_VACUUM: Constant = Constant {
    name: "speed of light in vacuum",
    value: 299_792_458.0,
    si_base_units: Some("m s^-1"),
    uncertainty: None,
};
//316
pub static STANDARD_ACCELERATION_OF_GRAVITY: Constant = Constant {
    name: "standard acceleration of gravity",
    value: 9.806_65,
    si_base_units: Some("m s^-2"),
    uncertainty: None,
};
//317
pub static STANDARD_ATMOSPHERE: Constant = Constant {
    name: "standard atmosphere",
    value: 101_325.0,
    si_base_units: Some("Pa"),
    uncertainty: None,
};
//318
pub static STANDARD_STATE_PRESSURE: Constant = Constant {
    name: "standard-state pressure",
    value: 100_000.0,
    si_base_units: Some("Pa"),
    uncertainty: None,
};
//319
pub static STEFAN_BOLTZMANN_CONSTANT: Constant = Constant {
    name: "Stefan-Boltzmann constant",
    value: 5.670_374_419e-8,
    si_base_units: Some("W m^-2 K^-4"),
    uncertainty: None,
};
//320
pub static TAU_COMPTON_WAVELENGTH: Constant = Constant {
    name: "tau Compton wavelength",
    value: 6.977_71e-16,
    si_base_units: Some("m"),
    uncertainty: Some(0.000_47e-16),
};
//321
pub static TAU_ELECTRON_MASS_RATION: Constant = Constant {
    name: "tau-electron mass ratio",
    value: 3_477.23,
    si_base_units: None,
    uncertainty: Some(0.23),
};
//322
pub static TAU_ENERGY_EQUIVALENT: Constant = Constant {
    name: "tau energy equivalent",
    value: 1_776.86,
    si_base_units: Some("MeV"),
    uncertainty: Some(0.12),
};
//323
pub static TAU_MASS: Constant = Constant {
    name: "tau mass",
    value: 3.167_54e-27,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_21e-27),
};
//324
pub static TAU_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "tau mass energy equivalent",
    value: 2.846_84e-10,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_19e-10),
};
//325
pub static TAU_MASS_IN_U: Constant = Constant {
    name: "tau mass in u",
    value: 1.907_54,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_13),
};
//326
pub static TAU_MOLAR_MASS: Constant = Constant {
    name: "tau molar mass",
    value: 1.907_54e-3,
    si_base_units: Some("kg mol^-1"),
    uncertainty: Some(0.000_13e-3),
};
//327
pub static TAU_MUON_MASS_RATIO: Constant = Constant {
    name: "tau-muon mass ratio",
    value: 16.817,
    si_base_units: None,
    uncertainty: Some(0.001_1),
};
//328
pub static TAU_NEUTRON_MASS_RATIO: Constant = Constant {
    name: "tau-neutron mass ratio",
    value: 1.891_15,
    si_base_units: None,
    uncertainty: Some(0.000_13),
};
//329
pub static TAU_PROTON_MASS_RATIO: Constant = Constant {
    name: "tau-proton mass ratio",
    value: 1.893_76,
    si_base_units: None,
    uncertainty: Some(0.000_13),
};
//330
pub static THOMAS_CROSS_SECTION: Constant = Constant {
    name: "Thomas cross section",
    value: 6.652_458_705_1e-29,
    si_base_units: Some("m^2"),
    uncertainty: Some(0.000_000_006_2e-29),
};
//331
pub static TRITON_ELECTRON_MASS_RATIO: Constant = Constant {
    name: "triton-electron mass ratio",
    value: 5_469.921_535_51,
    si_base_units: None,
    uncertainty: Some(0.000_000_21),
};
//332
pub static TRITON_G_FACTOR: Constant = Constant {
    name: "triton g factor",
    value: 5.957_924_93,
    si_base_units: None,
    uncertainty: Some(0.000_000_012),
};
//333
pub static TRITON_MAGNETIC_MOMENT: Constant = Constant {
    name: "triton magnetic moment",
    value: 1.504_609_517_8e-26,
    si_base_units: Some("J T^-1"),
    uncertainty: Some(0.000_000_003e-26),
};
//334
pub static TRITON_MAGNETIC_MOMENT_TO_BOHR_RATIO: Constant = Constant {
    name: "triton magnetic moment to Bohr magneton ratio",
    value: 1.622_393_664_8e-3,
    si_base_units: None,
    uncertainty: Some(0.000_000_003_2e-3),
};
//335
pub static TRITON_MOMENT_TO_NUCLEAR_MAGNETON_RATIO: Constant = Constant {
    name: "triton mag. mom. to nuclear magneton ratio",
    value: 2.978_962_465,
    si_base_units: None,
    uncertainty: Some(0.000_000_005_9),
};
//336
pub static TRITON_MASS: Constant = Constant {
    name: "triton mass",
    value: 5.007_356_751_2e-27,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_000_001_6e-27),
};
//337
pub static TRITON_MASS_ENERGY_EQUIVALENT: Constant = Constant {
    name: "triton mass energy equivalent",
    value: 4.500_387_811_9e-10,
    si_base_units: Some("J"),
    uncertainty: Some(0.000_000_001_4e-10),
};
//338
pub static TRITION_MASS_ENERGY_EQUIVALENT_IN_MEV: Constant = Constant {
    name: "triton mass energy equivalent in MeV",
    value: 2_808.921_136_68,
    si_base_units: Some("MeV"),
    uncertainty: Some(0.000_000_88),
};
//339
pub static TRITON_MASS_IN_U: Constant = Constant {
    name: "triton mass in u",
    value: 3.015_500_715_97,
    si_base_units: Some("u"),
    uncertainty: Some(0.000_000_000_1),
};
//340
pub static TRITON_MOLAR_MASS: Constant = Constant {
    name: "triton molar mass",
    value: 3.015_500_719_13e-3,
    si_base_units: Some("kg mol^-1"),
    uncertainty: Some(0.000_000_000_94e-3),
};
//341
pub static TRITON_PROTON_MASS_RATIO: Constant = Constant {
    name: "triton-proton mass ratio",
    value: 2.993_717_034_03,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_1),
};
//342
pub static TRITON_RELATIVE_ATOMIC_MASS: Constant = Constant {
    name: "triton relative atomic mass",
    value: 3.015_500_715_97,
    si_base_units: None,
    uncertainty: Some(0.000_000_000_1),
};
//343
pub static TRITON_TO_PROTON_MAGNETIC_MOMENT_RATIO: Constant = Constant {
    name: "triton to proton magnetic moment ratio",
    value: 1.066_639_918_9,
    si_base_units: None,
    uncertainty: Some(0.000_000_002_1),
};
//344
pub static UNIFIED_ATOMIC_MASS_UNIT: Constant = Constant {
    name: "unified atomic mass unit",
    value: 1.660_539_068_92e-27,
    si_base_units: Some("kg"),
    uncertainty: Some(0.000_000_000_52e-27),
};
//345
pub static VACUUM_ELECTRICITY_PERMITTIVITY: Constant = Constant {
    name: "vacuum electricy permittivity",
    value: 8.854_187_818_8e-12,
    si_base_units: Some("F m^-1"),
    uncertainty: Some(0.000_000_001_4e-12),
};
//346
pub static VACUUM_MAGNETIC_PERMEABILITY: Constant = Constant {
    name: "vacuum magnetic permeability",
    value: 1.256_637_061_27e-6,
    si_base_units: Some("N A^-2"),
    uncertainty: Some(0.000_000_000_2e-6),
};
//347
pub static VON_KLITIZING_CONSTANT: Constant = Constant {
    name: "von Klitzing constant",
    value: 25_812.807_45,
    si_base_units: Some("ohm"),
    uncertainty: None,
};
//348
pub static WEAK_MIXING_ANGLE: Constant = Constant {
    name: "weak mixing angle",
    value: 0.223_05,
    si_base_units: None,
    uncertainty: Some(0.000_23),
};
//349
pub static WIEN_FREQUENCY_DISPLACEMENT_LAW_CONSTANT: Constant = Constant {
    name: "Wien frequency displacement law constant",
    value: 5.878_925_757e10,
    si_base_units: Some("Hz K^-1"),
    uncertainty: None,
};
//350
pub static WIEN_WAVELENGTH_DISPLACEMENT_LAW: Constant = Constant {
    name: "Wien wavelength displacement law",
    value: 2.897_771_955e-3,
    si_base_units: Some("m K"),
    uncertainty: None,
};
//351
pub static W_TO_Z_MASS_RATIO: Constant = Constant {
    name: "W to Z mass ratio",
    value: 0.881_45,
    si_base_units: None,
    uncertainty: Some(0.000_13),
};
