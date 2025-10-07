import { Errors } from "$lib/types/error";
import itemStore from "$lib/stores/item-store";
import type { Item, ItemRequest } from "$lib/types";
import { get, type Subscriber } from "svelte/store";
import itemApi from "$lib/api/item-api";

const itemService = {
    async add(newItem: ItemRequest) {
        if (itemStore.containsItemName(newItem.name)) {
            throw Errors.Items.AlreadyExistsError;
        }

        itemStore.updateOptimistic(
            prev => {
                const final = prev;
                final.push({ item_id: get(itemStore).length, ...newItem });
                return final;
            },
            async prev => {
                const final = prev;
                const addedItem = await itemApi.add(newItem);
                final.push(addedItem);
                return final;
            }
        );
    },
    async delete(id: number) {
        if (!itemStore.containsItemId(id)) {
            throw Errors.Items.NotFoundError;
        }

        itemStore.updateOptimistic(
            prev => prev.filter(item => item.item_id !== id),
            async prev => {
                await itemApi.delete(id);
                return prev.filter(item => item.item_id !== id);
            }
        );
    },
    async subscribe(callback: Subscriber<Item[]>) {
        if (!get(itemStore)) {
            itemStore.set(await itemApi.getAll());
            return itemStore.subscribe(callback);
        }
        const unsubscribe = itemStore.subscribe(callback);
        itemStore.set(await itemApi.getAll());
        return unsubscribe;
    },
};

export default itemService;
