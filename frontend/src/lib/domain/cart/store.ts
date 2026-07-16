import api from "$lib/api";
import { writable } from "svelte/store";
import type { CartItem, CartItemUpdate } from "./cart";

const cartItems = writable<CartItem[]>([]);

async function execute(
    fn: (...args: any[]) => Promise<CartItem[]>,
    ...args: any[]
) {
    cartItems.set(await fn(...args));
}

const cartStore = {
    subscribe: cartItems.subscribe,
    async load() {
        await execute(api.cart.readAll);
    },
    async undo() {
        await execute(api.cart.undo);
    },
    async redo() {
        await execute(api.cart.redo);
    },
    async removeItem(id: number) {
        await execute(api.cart.removeItem, id);
    },
    async addItem(id: number) {
        await execute(api.cart.addItem, id);
    },
    async updateItem(id: number, newItem: CartItemUpdate) {
        await execute(api.cart.updateItem, id, newItem);
    },
    async addRecipe(id: number) {
        await execute(api.cart.addRecipe, id);
    },
    async stream() {
        const eventSource = await api.cart.getStream();
        eventSource.onmessage = e => cartItems.set(JSON.parse(e.data));
    },
};

export default cartStore;
