import data from "./data.svelte";
import itemService from "$lib/services/item-service";
import type { ItemRequest } from "$lib/types";
import { toastStore } from "$lib/stores/toast-store.svelte";
import { Errors } from "$lib/types/error";

const actions = {
    async init() {
        try {
            await this.loadItems();
        } catch (e: any) {
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
    async handleAddItem(e: SubmitEvent) {
        e.preventDefault();
        const data = new FormData(e.target as HTMLFormElement);
        const newItem = {
            name: data.get("item-name")!.toString(),
            icon: data.get("item-icon")!.toString(),
        };

        try {
            await itemService.add(newItem);
        } catch (e: any) {
            if (e === Errors.Items.AlreadyExistsError) {
                toastStore.show(e.message, "uhoh");
            }
        }
    },
    async deleteItem(id: number) {
        try {
            await itemService.delete(id);
        } catch (e: any) {
            if (e === Errors.Items.NotFoundError) {
                toastStore.show(e.message, "uhoh");
            }
        }
    }
};

export default actions;
