import type { Item, ItemAdd } from "$lib/types";
import { apiFetch } from "./fetch";

const itemsApi = {
    async getAll() {
        return await apiFetch("/items/search");
    },
    async add(newItem: ItemAdd) {
        return await apiFetch(
            "/items/new",
            "POST",
            { ...newItem }
        );
    },
    async delete(id: number) {
        return await apiFetch(`/items/${id}`, "DELETE");
    }
};

export default itemsApi;
