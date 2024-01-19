/**
 * Parses a string into a number, taking into account locales where commas
 * are used for decimal separator.
 * @param value The string to be parsed.
 * @returns The parsed number (valid or NaN).
 */

export function parseNumber(value: string) : number {
  const corrected = value.replace(",", ".");
  return Number(corrected);
}

/**
 * Checks if a given string can be parsed into a valid number.
 * @param value The string to be parsed.
 * @param canBeNegative Whether the result can be negative.
 * @param canBeZero Whether the result can be zero.
 * @returns True if the string is a valid number and fulfills the value restrictions, false otherwise.
 */
export function isValidNumber(
  value: string,
  canBeNegative: boolean = true,
  canBeZero: boolean = true,
): boolean {
  const parsed = parseNumber(value);
  if (Number.isNaN(parsed)) return false;
  if (!canBeZero && parsed == 0) return false;
  if (!canBeNegative && parsed < 0) return false;
  return true;
}