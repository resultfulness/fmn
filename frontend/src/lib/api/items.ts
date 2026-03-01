import type { Item, ItemUpdate, ItemCreate } from "$lib/schemas/items";
import request from "./request";

const items = {
    create(item: ItemCreate): Promise<Item> {
        return request.post("/items", item);
    },
    read(id: number): Promise<Item> {
        return request.get(`/items/${id}`);
    },
    readAll(): Promise<Item[]> {
        return request.get("/items");
    },
    update(id: number, item: ItemUpdate): Promise<Item> {
        return request.patch(`/items/${id}`, item);
    },
    delete(id: number): Promise<Item> {
        return request.delete(`/items/${id}`);
    },
};

export default items;
