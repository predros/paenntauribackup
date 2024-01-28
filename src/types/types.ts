/* eslint-disable no-unused-vars */
export enum Direction {
  None = 0,
  X,
  Y,
  Z,
}

export enum SupportType {
  None = 0,
  Rx,
  Ry,
  RxRy,
  Rz,
  RxRz,
  RyRz,
  RxRyRz,
}

export enum SectionType {
  Generic,
  Circle,
  Rectangle,
  SymmetricW,
  AsymmetricW,
}

export enum SideBarType {
  Select,
  NewNode,
  MatSec,
  NewMember,
  Supports,
  Hinges,
  NodalForces,
  MemberLoads,
  ThermalLoads,
  Result,
}

export enum ClickType {
  Select,
  NewNode,
  NewMemberStart,
  NewMemberEnd,
  Result,
}

export enum ResultType {
  Displacement,
  Normal,
  Shear,
  Moment,
}

export interface INode {
  id: number;
  x: number;
  y: number;
  hinged: boolean;
  supports: boolean[];
  supportAngle: number;
  springs: number[];
  prescribedDisplacements: number[];
  fx: number;
  fy: number;
  mz: number;
  forceAngle: number;
}

export interface IMember {
  id: number;
  x0: number;
  y0: number;
  x1: number;
  y1: number;
  length: number;
  angle: number;
  hinges: {
    start: boolean;
    end: boolean;
  };

  material: number;
  section: number;

  qx0: number;
  qy0: number;
  qx1: number;
  qy1: number;
  isGlobal: boolean;

  tSup: number;
  tInf: number;
}

export interface IMaterial {
  id: number;
  name: string;
  elasticity: number;
  thermal: number;
}

export interface ISection {
  id: number;
  name: string;
  inertia: number;
  area: number;
  ySup: number;
  yInf: number;
  sectionType: SectionType;
  params: number[];
}

export interface ILoadcase {
  id: number;
  name: string;
}

export interface ICombination {
  id: number;
  name: string;
  loadFactors: Record<number, number>;
}

export interface IMemberResult {
  id: number;
  dx: number[];
  dy: number[];
  rz: number[];

  normal: number[];
  shear: number[];
  moment: number[];

  maxMoment: number[];
  minMoment: number[];
  vertShear: number[];
  vertNormal: number[];
}

export interface INodeReaction {
  id: number;
  rx: number;
  ry: number;
  mz: number;
}

export type ResultsDict = Record<number, IMemberResult[]>;
export type ReactionsDict = Record<number, INodeReaction[]>;

export interface KonvaWheelEvent {
  evt: WheelEvent;
  cancelBubble: boolean;
  pointerId: number;
  type: string;
}

export interface KonvaMouseEvent {
  evt: MouseEvent;
  cancelBubble: boolean;
  pointerId: number;
  type: string;
}
