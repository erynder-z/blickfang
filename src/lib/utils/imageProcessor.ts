import { imageFormat, imageResolution, imageAspectRatio, imageExif } from "$lib/stores/appState";

const greatestCommonDivisor = (a: number, b: number): number => {
  return b === 0 ? a : greatestCommonDivisor(b, a % b);
};

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
