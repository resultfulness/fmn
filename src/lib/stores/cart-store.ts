import type { Item } from "$lib/types";
import type { CartItem } from "$lib/types/cart";
import { get } from "svelte/store";
import { optimisticStore } from "./optimistic-store";

const cartStore = (() => {
    const store = optimisticStore<CartItem[]>();
    const { subscribe, set, update, updateOptimistic, undo } = store;

    return {
        subscribe,
        set,
        update,
        updateOptimistic,
        undo,
        containsItem(item: Item, origin: string = "") {
            return get(store).some(storeItem =>
                storeItem.item_id === item.item_id &&
                storeItem.origin === origin);
        },
    };
})();

export default cartStore;
