import clip from "liang-barsky";

export interface IRectangle {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface IPoint {
  x: number;
  y: number;
}

export interface ILine {
  start: IPoint;
  end: IPoint;
}

export function floatEq(
  first: number,
  second: number,
  tolerance: number = 1e-5,
): boolean {
  return Math.abs(first - second) < Math.abs(tolerance);
}

/**
 * Finds the Euclidean distance between two points.
 * @param start The starting point.
 * @param end The ending point.
 * @returns The distance.
 */
export function distance(start: IPoint, end: IPoint): number {
  return Math.sqrt(
    (end.x - start.x) * (end.x - start.x) +
      (end.y - start.y) * (end.y - start.y),
  );
}

/**
 * Finds the angle between a line defined by two points and the X axis.
 * @param start The first point.
 * @param end The second point.
 * @returns The angle, in degrees.
 */
export function angle(start: IPoint, end: IPoint): number {
  let result = Math.atan2(end.y - start.y, end.x - start.x) * (180 / Math.PI);

  if (result < 0) result += 360;
  if (result > 360) result %= 360;

  return result;
}



/**
 * Checks if a given point is contained within a given axis-aligned rectangle.
 * @param point The point.
 * @param rect The rectangle.
 * @returns True if the point is contained, false otherwise.
 */
export function isPointInRect(point: IPoint, rect: IRectangle): boolean {
  const xLeft = rect.width > 0 ? rect.x : rect.x + rect.width;
  const xRight = rect.width > 0 ? rect.x + rect.width : rect.x;
  const yTop = rect.height > 0 ? rect.y : rect.y + rect.height;
  const yBottom = rect.height > 0 ? rect.y + rect.height : rect.y;

  return (
    point.x >= xLeft &&
    point.x <= xRight &&
    point.y >= yTop &&
    point.y <= yBottom
  );
}

/**
 * Checks whether a line segment intersects or is contained by a given axis-aligned rectangle.
 * Liang-Barsky has been the bane of my existence for years now, so I've decided to just use
 * a Node module (credits to https://github.com/w8r/liang-barsky).
 * @param line The line.
 * @param rect The rectangle.
 * @returns True if the line is fully or partially within the rectangle, false otherwise.
 */
export function doesLineIntersectRect(line: ILine, rect: IRectangle): boolean {
  const xMin: number = Math.min(rect.x, rect.x + rect.width);
  const xMax: number = Math.max(rect.x, rect.x + rect.width);
  const yMin: number = Math.min(rect.y, rect.y + rect.height);
  const yMax: number = Math.max(rect.y, rect.y + rect.height);

  const x0: number = line.start.x >= line.end.x ? line.end.x : line.start.x;
  const y0: number = line.start.x >= line.end.x ? line.end.y : line.start.y;
  const x1: number = line.start.x >= line.end.x ? line.start.x : line.end.x;
  const y1: number = line.start.x >= line.end.x ? line.start.y : line.end.y;

  const result = clip([x0, y0], [x1, y1], [xMin, yMin, xMax, yMax]);

  return result == 1;
}

/**
 * Checks whether a line segment is completely contained within a given axis-aligned rectangle.
 * @param line The line segment.
 * @param rect The rectangle.
 * @returns True if the segment is completely within, false otherwise.
 */
export function isLineInRect(line: ILine, rect: IRectangle): boolean {
  const xLeft = rect.width > 0 ? rect.x : rect.x + rect.width;
  const xRight = rect.width > 0 ? rect.x + rect.width : rect.x;
  const yTop = rect.height > 0 ? rect.y : rect.y + rect.height;
  const yBottom = rect.height > 0 ? rect.y + rect.height : rect.y;

  const xMin = Math.min(line.start.x, line.end.x);
  const yMin = Math.min(line.start.y, line.end.y);
  const xMax = Math.max(line.start.x, line.end.x);
  const yMax = Math.max(line.start.y, line.end.y);

  return xMin >= xLeft && xMax <= xRight && yMin >= yTop && yMax <= yBottom;
}

/**
 * Finds the projection of a given vector onto another.
 * @param base The vector to be projected onto (with tail at the origin).
 * @param projected The vector to project (with tail at the origin).
 * @returns The projected vector, with tail at the origin, if the base vector is valid. Null, otherwise.
 */
export function projectOntoVector(
  base: IPoint,
  projected: IPoint,
): IPoint | null {
  const lengthSquared = base.x * base.x + base.y * base.y;
  if (lengthSquared == 0) return null;

  const dotProduct = base.x * projected.x + base.y * projected.y;
  const ratio = dotProduct / lengthSquared;

  return { x: ratio * base.x, y: ratio * base.y };
}

export function lerp(start: IPoint, end: IPoint, x: number): number {
  if (floatEq(start.x, end.x)) return end.y;
  const slope = (end.y - start.y) / (end.x - start.x);
  return start.y + slope * (x - start.x);
}
