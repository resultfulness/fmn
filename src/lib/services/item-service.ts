import itemRepository from "$lib/repositories/item-repository";
import itemStore from "$lib/stores/item-store";
import type { Item } from "$lib/types";
import { get, type Subscriber } from "svelte/store";

const itemService = {
    getAll() {
        this.fetchAll();
        return get(itemStore);
    },
    async fetchAll() {
        itemStore.set(await itemRepository.getAll());
    },
    subscribe(callback: Subscriber<Item[]>) {
        return itemStore.subscribe(callback);
    },
};

export default itemService;
