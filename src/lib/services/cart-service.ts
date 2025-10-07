import cartApi from "$lib/api/cart-api";
import cartStore from "$lib/stores/cart-store";
import type { Item } from "$lib/types";
import type { CartItem } from "$lib/types/cart";
import { type Subscriber } from "svelte/store";

cartStore.set(await cartApi.getAll());
cartApi.stream(d => cartStore.set(d));

const cartService = {
    async addItem(item: Item) {
        if (cartStore.containsItem(item)) {
            return;
        }

        cartStore.updateOptimistic(
            prev => {
                const final = prev;
                final.push({
                    origin: "",
                    ...item,
                });
                return final;
            },
            async () => {
                return await cartApi.addItem(item.item_id);
            }
        );
    },
    async deleteItem(item: CartItem) {
        cartStore.updateOptimistic(
            prev => {
                return prev.filter(storeItem =>
                    storeItem.item_id !== item.item_id ||
                    storeItem.origin !== item.origin
                )
            },
            async () => await cartApi.deleteItem(item.item_id, item.origin)
        );
    },
    subscribe(callback: Subscriber<CartItem[]>) {
        return cartStore.subscribe(callback);
    }
};

export default cartService;
