#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub dark_theme: bool,
    pub locale: String,
    pub grid_spacing: [i32; 2],
    pub units: UnitSettings,
    pub unit_precision: UnitPrecision,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct UnitSettings {
    pub length: UnitLength,
    pub force: UnitForce,
    pub angle: UnitAngle,
    pub temperature: UnitTemperature,
    pub moment: (UnitForce, UnitLength),
    pub load: (UnitForce, UnitLength),
    pub displacement: UnitLength,
    pub rotation: UnitAngle,
    pub spring: (UnitForce, UnitLength),
    pub torsion_spring: (UnitForce, UnitLength, UnitAngle),
    pub elasticity: UnitStress,
    pub thermal: UnitTemperature,
    pub inertia: UnitLength,
    pub area: UnitLength,
    pub dimension: UnitLength,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct UnitPrecision {
    pub length: (usize, bool),
    pub force: (usize, bool),
    pub angle: (usize, bool),
    pub temperature: (usize, bool),
    pub moment: (usize, bool),
    pub load: (usize, bool),
    pub displacement: (usize, bool),
    pub rotation: (usize, bool),
    pub spring: (usize, bool),
    pub torsion_spring: (usize, bool),
    pub elasticity: (usize, bool),
    pub thermal: (usize, bool),
    pub inertia: (usize, bool),
    pub area: (usize, bool),
    pub dimension: (usize, bool),
}

impl UnitSettings {
    const LENGTH_FACTORS: [f64; 5] = [1.0, 10.0, 0.01, 0.393701, 0.0328084];
    const FORCE_FACTORS: [f64; 5] = [1.0, 1000.0, 101.972, 0.101972, 224.809];
    const ANGLE_FACTORS: [f64; 2] = [1.0, 0.0174533];
    const TEMPERATURE_FACTORS: [f64; 3] = [1.0, 1.8, 1.0];
    const STRESS_FACTORS: [f64; 6] = [1.0, 1000.0, 1000000.0, 0.1, 145.038, 9.86923];

    fn get_factor(&self, unit_type: UnitType) -> f64 {
        let factor: f64 = match unit_type {
            UnitType::Length => 1.0,
            UnitType::Force => UnitSettings::FORCE_FACTORS[self.force as usize],
            UnitType::Angle => UnitSettings::ANGLE_FACTORS[self.angle as usize],
            UnitType::Temperature => UnitSettings::TEMPERATURE_FACTORS[self.temperature as usize],
            UnitType::Moment => {
                UnitSettings::FORCE_FACTORS[self.moment.0 as usize]
                    * UnitSettings::LENGTH_FACTORS[self.moment.1 as usize]
            }
            UnitType::Load => {
                UnitSettings::FORCE_FACTORS[self.load.0 as usize]
                    / UnitSettings::LENGTH_FACTORS[self.load.1 as usize]
            }
            UnitType::Displacement => 1.0,
            UnitType::Rotation => UnitSettings::ANGLE_FACTORS[self.rotation as usize],
            UnitType::Spring => {
                UnitSettings::FORCE_FACTORS[self.spring.0 as usize]
                    / UnitSettings::LENGTH_FACTORS[self.spring.1 as usize]
            }
            UnitType::TorsionSpring => {
                UnitSettings::FORCE_FACTORS[self.torsion_spring.0 as usize]
                    * UnitSettings::LENGTH_FACTORS[self.torsion_spring.1 as usize]
                    / UnitSettings::ANGLE_FACTORS[self.torsion_spring.2 as usize]
            }
            UnitType::Elasticity => UnitSettings::STRESS_FACTORS[self.elasticity as usize],
            UnitType::Thermal => 1.0 / UnitSettings::TEMPERATURE_FACTORS[self.thermal as usize],
            UnitType::Inertia => UnitSettings::LENGTH_FACTORS[self.inertia as usize].powi(4),
            UnitType::Area => UnitSettings::LENGTH_FACTORS[self.area as usize].powi(2),
            UnitType::Dimension => UnitSettings::LENGTH_FACTORS[self.dimension as usize],
        };
        factor
    }

    pub fn convert_to(&self, value: f64, unit_type: UnitType) -> f64 {
        let factor = self.get_factor(unit_type);
        factor * value
    }

    pub fn convert_from(&self, value: f64, unit_type: UnitType) -> f64 {
        let factor = self.get_factor(unit_type);
        value / factor
    }
}

#[derive(Clone, Copy, serde_repr::Deserialize_repr, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum UnitType {
    Length,
    Force,
    Angle,
    Temperature,
    Moment,
    Load,
    Displacement,
    Rotation,
    Spring,
    TorsionSpring,
    Elasticity,
    Thermal,
    Inertia,
    Area,
    Dimension,
}

#[derive(Clone, Copy, serde_repr::Deserialize_repr, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum UnitLength {
    Centimeter,
    Milimeter,
    Meter,
    Inch,
    Foot,
}

#[derive(Clone, Copy, serde_repr::Deserialize_repr, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum UnitForce {
    KiloNewton,
    Newton,
    KilogramForce,
    TonForce,
    PoundForce,
}

#[derive(Clone, Copy, serde_repr::Deserialize_repr, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum UnitAngle {
    Degree,
    Radian,
}

#[derive(Clone, Copy, serde_repr::Deserialize_repr, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum UnitTemperature {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Clone, Copy, serde_repr::Deserialize_repr, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum UnitStress {
    MegaPascal,
    KiloPascal,
    Pascal,
    KNcm2,
    Psi,
    Atmosphere,
}
