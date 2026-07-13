import type { Item, ItemUpdate, ItemCreate } from "$lib/domain/items/item";
import request from "./request";

export default {
    async create(item: ItemCreate): Promise<Item> {
        return await request.post("/items", item);
    },
    async read(id: number): Promise<Item> {
        return await request.get(`/items/${id}`);
    },
    async readAll(): Promise<Item[]> {
        return await request.get("/items");
    },
    async update(id: number, item: ItemUpdate): Promise<Item> {
        return await request.patch(`/items/${id}`, item);
    },
    async delete(id: number): Promise<Item> {
        return await request.delete(`/items/${id}`);
    },
};
