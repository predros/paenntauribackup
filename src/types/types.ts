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
  support_angle: number;
  springs: number[];
  prescribed_displacements: number[];
  fx: number;
  fy: number;
  mz: number;
  force_angle: number;
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
  is_global: boolean;

  t_sup: number;
  t_inf: number;
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
  y_sup: number;
  y_inf: number;
  section_type: SectionType;
  params: number[];
}

export interface ILoadcase {
  id: number;
  name: string;
}

export interface ICombination {
  id: number;
  name: string;
}

export interface IMemberResult {
  id: number;
  dx: number[];
  dy: number[];
  rz: number[];

  normal: number[];
  shear: number[];
  moment: number[];

  max_moment: number[];
  min_moment: number[];
  vert_shear: number[];
  vert_normal: number[];
}

export interface INodeReaction {
  id: number;
  rx: number;
  ry: number;
  mz: number;
}

export type ResultsDict = { [key: number]: IMemberResult[] };
export type ReactionsDict = { [key: number]: INodeReaction[] };

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
