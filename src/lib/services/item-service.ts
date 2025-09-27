import { Errors } from "$lib/types/error";
import itemRepository from "$lib/repositories/item-repository";
import itemStore from "$lib/stores/item-store";
import type { ItemAdd, Items } from "$lib/types";
import { get, type Subscriber } from "svelte/store";

const itemService = {
    getAll() {
        return get(itemStore);
    },
    async fetchAll() {
        itemStore.set(await itemRepository.getAll());
    },
    async add(newItem: ItemAdd) {
        if (itemStore.containsItemName(newItem.name)) {
            throw Errors.Items.AlreadyExistsError;
        }
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
        if (!itemStore.containsItemId(id)) {
            throw Errors.Items.NotFoundError;
        }
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
