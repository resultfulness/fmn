import itemsApi from "$lib/api/items";
import { handleGenericErrors } from "$lib/error";
import type { Item, ItemAdd } from "$lib/types";

const itemRepository = {
    async getAll() {
        let res = await itemsApi.getAll();
        res = await handleGenericErrors(res);
        return await res.json();
    },
    async add(newItem: ItemAdd): Promise<Item> {
        let res = await itemsApi.add(newItem);
        res = await handleGenericErrors(res);
        return await res.json();
    },
    async delete(id: number): Promise<Item> {
        let res = await itemsApi.delete(id);
        res = await handleGenericErrors(res);
        return await res.json();
    }
}

export default itemRepository;
