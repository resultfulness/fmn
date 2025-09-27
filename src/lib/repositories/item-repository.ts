import itemsApi from "$lib/api/items";
import { Errors } from "$lib/types/error";
import type { Item, ItemAdd, Items } from "$lib/types";

const itemRepository = {
    async getAll(): Promise<Items> {
        return await (await itemsApi.getAll()).json();
    },
    async add(newItem: ItemAdd): Promise<Item> {
        const res = await itemsApi.add(newItem);
        if (!res.ok && res.status === 409) {
            throw Errors.Items.AlreadyExistsError;
        }
        return await res.json();
    },
    async delete(id: number): Promise<Item> {
        const res = await itemsApi.delete(id);
        if (!res.ok && res.status === 404) {
            throw Errors.Items.NotFoundError;
        }
        return await res.json();
    }
}

export default itemRepository;
