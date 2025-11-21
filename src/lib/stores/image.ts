// src/lib/stores/image.ts
import type { AiDetectionResult } from "$lib/types/image";
import { writable } from "svelte/store";

export const imageUrl = writable<string | null>(null);
export const imagePath = writable<string | null>(null);
export const imageExif = writable<string | null>(null);
export const imageFormat = writable<string | null>(null);

export const imageResolution = writable<{ width: number; height: number } | null>(null);
export const imageAspectRatio = writable<string | null>(null);
export const imageColorDepth = writable<number | null>(null);
export const imageFileSize = writable<number | null>(null);
export const aiDetectionResult = writable<AiDetectionResult | null>(null);
