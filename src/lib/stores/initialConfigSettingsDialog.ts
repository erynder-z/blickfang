import { writable } from "svelte/store";

export const isInitialConfigSettingsDialogVisible = writable<boolean>(false);