import type { Item, ItemRequest } from "$lib/types";
import { get, writable } from "svelte/store";

const itemStore = (() => {
    const store = writable<Item[]>();
    const { subscribe, set, update } = store;

    return {
        subscribe,
        set,
        update,
        pushNewItem(newItem: ItemRequest) {
            this.pushItem({
                item_id: get(store).length,
                ...newItem
            })
        },
        pushItem(item: Item) {
            store.update(prev => {
                const final = prev;
                final.push(item);
                return final;
            });
        },
        deleteItem(id: number) {
            store.update(prev => {
                let final = prev;
                final = prev.filter(item => item.item_id !== id);
                return final;
            })
        },
        containsItemId(id: number) {
            return get(store).some(item => item.item_id === id);
        },
        containsItemName(itemName: string) {
            return get(store).some(item => item.name === itemName);
        }
    }
})();

export default itemStore;
