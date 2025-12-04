import { writable } from "svelte/store";

export const isInitialDialogVisible = writable<boolean>(false);
export const hasConfiguredInitialSettings = writable<boolean>(false);
export const isInitialConfigSettingsDialogVisible = writable<boolean>(false);
