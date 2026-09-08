/**
 * Tests whether the provided value is a {@link Date}.
 * @param value - The value to test.
 */
function isDate(value: unknown): value is Date {
  return value instanceof Date && !isNaN(value.getTime())
}

/**
 * Parses the provided date string to a date.
 * @param dateString - The date string to parse.
 * @throws {Error} - When the provided date string doesn't represent a valid date.
 */
function parse(dateString: string): Date {
  const parsedDate = new Date(Date.parse(dateString))

  if (!isDate(parsedDate)) {
    throw new Error('The provided date string does not represent a valid date!')
  }

  return parsedDate
}

/**
 * Parses a date string and formats it to something more readable.
 * If the date string is invalid, the default value will be returned.
 * @param dateString - The date string to parse and format.
 * @param [fallbackValue='?'] - The fallback value to return when the provided date string is invalid.
 */
export function format(dateString: string, fallbackValue: string = '?'): string {
  try {
    return parse(dateString).toLocaleDateString()
  } catch (error) {
    console.warn(
      `Failed to format the provided date string, returning the fallback value '${fallbackValue}': ${(error as Error).message}`,
      { cause: error }
    )
    return fallbackValue
  }
}
