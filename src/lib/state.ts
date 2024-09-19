import { writable } from "svelte/store";

export const loading = writable<boolean>(true);
export const wallpapers = writable<string[]>([]);

export const appVersion = writable<string | undefined>(undefined);
export const tauriVersion = writable<string | undefined>(undefined);

export const modifiersHeld = writable<string[]>([]);
export const cropMode = writable<boolean>(false);

export const input = writable<string>("");
export const previousInput = writable<string>("");

export const debugScroll = writable<boolean>(false);
export const debugGeneral = writable<boolean>(false);
