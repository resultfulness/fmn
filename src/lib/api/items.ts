import type { Item, ItemAdd } from "$lib/types";
import { Errors } from "$lib/types/error";
import { apiFetch } from "./fetch";

const itemsApi = {
    async getAll(): Promise<Item[]> {
        return await (await apiFetch("/items")).json();
    },
    async add(newItem: ItemAdd): Promise<Item> {
        const res = await apiFetch(
            "/items",
            "POST",
            { ...newItem }
        );
        if (!res.ok && res.status === 409) {
            throw Errors.Items.AlreadyExistsError;
        }
        return await res.json();
    },
    async delete(id: number) {
        const res = await apiFetch(`/items/${id}`, "DELETE");
        if (!res.ok && res.status === 404) {
            throw Errors.Items.NotFoundError;
        }
        return await res.json();
    }
};

export default itemsApi;
