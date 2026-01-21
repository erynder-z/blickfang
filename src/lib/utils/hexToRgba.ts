/**
 * Converts a hex color to an RGBA string with optional transparency.
 * @param hex - The hex color (e.g., "#FF5733" or "FF5733").
 * @param alpha - The transparency (0 to 1). Defaults to 0.5.
 * @returns The RGBA string (e.g., "rgba(255, 87, 51, 0.5)").
 */
export const hexToRgba = (hex: string, alpha: number = 0.5): string => {
  hex = hex.replace("#", "");

  const r = parseInt(hex.length === 3 ? hex[0] + hex[0] : hex.substring(0, 2), 16);
  const g = parseInt(hex.length === 3 ? hex[1] + hex[1] : hex.substring(2, 4), 16);
  const b = parseInt(hex.length === 3 ? hex[2] + hex[2] : hex.substring(4, 6), 16);

  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
};
