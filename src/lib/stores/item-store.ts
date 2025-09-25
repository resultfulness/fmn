import type { Item } from "$lib/types";
import { writable } from "svelte/store";

const itemStore = writable<Item[]>([]);

export default itemStore;
