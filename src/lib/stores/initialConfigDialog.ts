import { writable } from "svelte/store";

export const isInitialConfigDialogVisible = writable<boolean>(false);
export const hasConfiguredInitialSettings = writable<boolean>(false);
