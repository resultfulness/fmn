import type { Item, ItemRequest } from "$lib/types";
import { Errors } from "$lib/types/error";
import { apiFetch } from "./fetch";

const itemApi = {
    async getAll(): Promise<Item[]> {
        return await (await apiFetch("/items")).json();
    },
    async add(newItem: ItemRequest): Promise<Item> {
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
    async delete(id: number): Promise<Item> {
        const res = await apiFetch(`/items/${id}`, "DELETE");
        if (!res.ok && res.status === 404) {
            throw Errors.Items.NotFoundError;
        }
        return await res.json();
    },
};

export default itemApi;
