/**
 * Returns a human-readable representation of the given number of bytes.
 * The returned string will be in the format of "X.XX YYY" where X.XX is the number of bytes
 * and YYY is the unit of measurement (B, KB, MB, GB, TB).
 * @param {number} bytes - The number of bytes to format.
 * @returns {string} - A human-readable representation of the given number of bytes.
 */
export const prettySize = (bytes: number) => {
  const units = ["B", "KB", "MB", "GB", "TB"];
  let size = bytes;
  let unit = 0;

  while (size >= 1024 && unit < units.length - 1) {
    size /= 1024;
    unit++;
  }

  return `${size.toFixed(2)} ${units[unit]}`;
};
