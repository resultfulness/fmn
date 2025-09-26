import data from "./data.svelte";
import itemService from "$lib/services/item-service";
import type { ItemAdd } from "$lib/types";

const actions = {
    async init() {
        await this.loadItems();
        this.deinit = itemService.subscribe(items => data.items = items);
    },
    deinit() { },
    async loadItems() {
        data.items = await itemService.getAll();
    },
    addItem(newItem: ItemAdd) {
        itemService.add(newItem);
    },
    deleteItem(id: number) {
        itemService.delete(id);
    }
};

export default actions;
