export enum UnitType {
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

export enum UnitLength {
  Centimeter,
  Milimeter,
  Meter,
  Inch,
  Foot,
}

export enum UnitForce {
  KiloNewton,
  Newton,
  KilogramForce,
  TonForce,
  PoundForce,
}

export enum UnitAngle {
  Degree,
  Radian,
}

export enum UnitTemperature {
  Celsius,
  Fahrenheit,
  Kelvin,
}

export enum UnitStress {
  MegaPascal,
  KiloPascal,
  Pascal,
  KNcm2,
  Psi,
  Atmosphere,
}

export interface ISettings {
  dark_theme: boolean;
  locale: string;
  grid_spacing: [number, number];
  units: IUnitSettings;
  unit_precision: IUnitPrecision;
}

export interface IUnitSettings {
  length: UnitLength;
  force: UnitForce;
  angle: UnitAngle;
  temperature: UnitTemperature;
  moment: [UnitForce, UnitLength];
  load: [UnitForce, UnitLength];
  displacement: UnitLength;
  rotation: UnitAngle;
  spring: [UnitForce, UnitLength];
  torsion_spring: [UnitForce, UnitLength, UnitAngle];
  elasticity: UnitStress;
  thermal: UnitTemperature;
  inertia: UnitLength;
  area: UnitLength;
  dimension: UnitLength;
}

export interface IUnitPrecision {
  length: [number, boolean];
  force: [number, boolean];
  angle: [number, boolean];
  temperature: [number, boolean];
  moment: [number, boolean];
  load: [number, boolean];
  displacement: [number, boolean];
  rotation: [number, boolean];
  spring: [number, boolean];
  torsion_spring: [number, boolean];
  elasticity: [number, boolean];
  thermal: [number, boolean];
  inertia: [number, boolean];
  area: [number, boolean];
  dimension: [number, boolean];
}
