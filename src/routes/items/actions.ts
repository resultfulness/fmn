import data from "./data.svelte";
import itemService from "$lib/services/item-service";
import type { ItemAdd } from "$lib/types";
import { toastStore } from "$lib/stores/toast-store.svelte";

const actions = {
    async init() {
        try {
            await this.loadItems();
        } catch (e: any) {
            toastStore.show(e.message, "uhoh");
            return;
        }
        this.deinit = itemService.subscribe(items => data.items = items);
    },
    deinit() { },
    async loadItems() {
        const items = itemService.getAll();
        if (!items) {
            await itemService.fetchAll();
            data.items = itemService.getAll();
        } else {
            data.items = items;
            await itemService.fetchAll();
        }
    },
    addItem(newItem: ItemAdd) {
        try {
            itemService.add(newItem);
        } catch (e: any) {
            toastStore.show(e.message, "uhoh");
        }
    },
    deleteItem(id: number) {
        try {
            itemService.delete(id);
        } catch (e: any) {
            toastStore.show(e.message, "uhoh");
        }
    }
};

export default actions;
