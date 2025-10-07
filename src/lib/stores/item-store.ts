import type { Item } from "$lib/types";
import { get } from "svelte/store";
import { optimisticStore } from "./optimistic-store";

const itemStore = (() => {
    const store = optimisticStore<Item[]>();
    const { subscribe, set, update, updateOptimistic, undo } = store;

    return {
        subscribe,
        set,
        update,
        updateOptimistic,
        undo,
        containsItemId(id: number) {
            return get(store).some(item => item.item_id === id);
        },
        containsItemName(itemName: string) {
            return get(store).some(item => item.name === itemName);
        }
    }
})();

export default itemStore;
