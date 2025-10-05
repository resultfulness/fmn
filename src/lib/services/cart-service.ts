import cartApi from "$lib/api/cart-api";
import cartStore from "$lib/stores/cart-store";
import type { Item } from "$lib/types";
import type { CartItem } from "$lib/types/cart";
import { get, type Subscriber } from "svelte/store";

cartStore.set(await cartApi.getAll());
cartApi.stream(d => cartStore.set(d));

const cartService = {
    async addItem(item: Item) {
        if (cartStore.containsItem(item)) {
            return;
        }
        const prev = structuredClone(get(cartStore));
        cartStore.pushItem(item);

        try {
            await cartApi.addItem(item.item_id);
        } catch (e) {
            cartStore.set(prev);
            throw e;
        }
    },
    async deleteItem(item: CartItem) {
        const prev = structuredClone(get(cartStore));
        cartStore.deleteItem(item);

        try {
            await cartApi.deleteItem(item.item_id, item.origin);
        } catch (e) {
            cartStore.set(prev);
            throw e;
        }
    },
    subscribe(callback: Subscriber<CartItem[]>) {
        return cartStore.subscribe(callback);
    }
};

export default cartService;
