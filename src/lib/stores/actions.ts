import { writable } from "svelte/store";

export const isRemapping = writable(false);
export const activeActions = writable<string[]>([]);
