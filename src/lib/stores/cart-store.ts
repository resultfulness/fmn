import type { CartItem } from "$lib/types/cart";
import { writable } from "svelte/store";

const cartStore = writable<CartItem[]>([]);

export default cartStore;
