import { Errors } from "$lib/types/error";
import itemStore from "$lib/stores/item-store";
import type { Item, ItemRequest } from "$lib/types";
import { get, type Subscriber } from "svelte/store";
import itemApi from "$lib/api/item-api";

const itemService = {
    getAll() {
        return get(itemStore);
    },
    async fetchAll() {
        itemStore.set(await itemApi.getAll());
    },
    async add(newItem: ItemRequest) {
        if (itemStore.containsItemName(newItem.name)) {
            throw Errors.Items.AlreadyExistsError;
        }
        const prev = structuredClone(get(itemStore));
        itemStore.pushNewItem(newItem);
        try {
            const addedItem = await itemApi.add(newItem);
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
            await itemApi.delete(id);
        } catch (e) {
            itemStore.set(prev);
            throw e;
        }
    },
    subscribe(callback: Subscriber<Item[]>) {
        return itemStore.subscribe(callback);
    },
};

export default itemService;
