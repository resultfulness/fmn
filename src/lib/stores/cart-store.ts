import type { Item } from "$lib/types";
import type { CartItem } from "$lib/types/cart";
import { get, writable } from "svelte/store";

const cartStore = (() => {
    const store = writable<CartItem[]>();
    const { subscribe, set, update } = store;

    return {
        subscribe,
        set,
        update,
        containsItem(item: Item, origin: string = "") {
            return get(store).some(storeItem =>
                storeItem.item_id === item.item_id &&
                storeItem.origin === origin);
        },
        pushItem(item: Item) {
            this.update(prev => {
                prev.push({
                    ...item,
                    origin: "",
                })
                return prev;
            });
        },
        deleteItem(item: CartItem) {
            this.update(prev => {
                const final = prev.filter(storeItem =>
                    storeItem.item_id !== item.item_id ||
                    storeItem.origin !== item.origin
                );
                return final;
            });
        }
    };
})();

export default cartStore;
