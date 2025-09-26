import type { Item, ItemAdd, Items } from "$lib/types";
import { get, writable } from "svelte/store";

const itemStore = (() => {
    const store = writable<Items>();
    const { subscribe, set, update } = store;

    return {
        subscribe,
        set,
        update,
        pushNewItem(newItem: ItemAdd) {
            this.pushItem({
                item_id: get(store).count,
                ...newItem
            })
        },
        pushItem(item: Item) {
            store.update(prev => {
                const final = prev;
                final.items.push(item);
                final.count++;
                return final;
            });
        },
        deleteItem(id: number) {
            store.update(prev => {
                const final = prev;
                final.items = prev.items.filter(item => item.item_id !== id);
                final.count--;
                return final;
            })
        }
    }
})();

export default itemStore;
