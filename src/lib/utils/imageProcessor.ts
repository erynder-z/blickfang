import { imageFormat, imageResolution, imageAspectRatio, imageExif } from "$lib/stores/appState";

/**
 * Calculate the greatest common divisor of two numbers.
 * This function uses the Euclidean algorithm to find the GCD.
 * @param {number} a - The first number.
 * @param {number} b - The second number.
 * @returns {number} The greatest common divisor of a and b.
 */
const greatestCommonDivisor = (a: number, b: number): number => {
  return b === 0 ? a : greatestCommonDivisor(b, a % b);
};

/**
 * Processes an image by setting the image format, resolution, and aspect ratio in the app state.
 * The image is decoded and its width and height are used to calculate the aspect ratio.
 * If the image cannot be decoded, an error is logged to the console and the image resolution and aspect ratio are set to null.
 * @param {string} imageData - The data URL of the image.
 * @param {string} imagePath - The path to the image file.
 * @param {string | null} exifData - The EXIF data of the image, if available.
 */
export const processImage = async (
  imageData: string,
  imagePath: string,
  exifData: string | null
) => {
  imageExif.set(exifData);

  const format = imagePath.split(".").pop()?.toUpperCase() ?? null;
  imageFormat.set(format);

  const img = new Image();
  img.src = imageData;
  try {
    await img.decode();
    const width = img.naturalWidth;
    const height = img.naturalHeight;
    const divisor = greatestCommonDivisor(width, height);
    const aspectRatio = `${width / divisor}:${height / divisor}`;

    imageResolution.set({ width, height });
    imageAspectRatio.set(aspectRatio);
  } catch (err) {
    console.error("Failed to decode image:", err);
    imageResolution.set(null);
    imageAspectRatio.set(null);
  }
};
