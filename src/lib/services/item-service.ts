import itemRepository from "$lib/repositories/item-repository";
import itemStore from "$lib/stores/item-store";
import type { ItemAdd, Items } from "$lib/types";
import { get, type Subscriber } from "svelte/store";

const itemService = {
    async getAll() {
        const prev = get(itemStore);
        if (prev) {
            this.fetchAll();
            return prev;
        } else {
            await this.fetchAll();
            return get(itemStore);
        }
    },
    async fetchAll() {
        itemStore.set(await itemRepository.getAll());
    },
    async add(newItem: ItemAdd) {
        const prev = structuredClone(get(itemStore));
        itemStore.pushNewItem(newItem);
        try {
            const addedItem = await itemRepository.add(newItem);
            itemStore.set(prev);
            itemStore.pushItem(addedItem);
        } catch (e) {
            itemStore.set(prev);
            throw e;
        }
    },
    async delete(id: number) {
        const prev = structuredClone(get(itemStore));
        itemStore.deleteItem(id);
        try {
            const _deletedItem = await itemRepository.delete(id);
        } catch (e) {
            itemStore.set(prev);
            throw e;
        }
    },
    subscribe(callback: Subscriber<Items>) {
        return itemStore.subscribe(callback);
    },
};

export default itemService;
