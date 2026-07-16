import { PUBLIC_API_URL } from "$env/static/public";
import type { CartItem, CartItemUpdate } from "$lib/domain/cart/cart";
import { pushToast } from "$lib/ui/toast";
import request from "./request";

export default {
    async readAll(): Promise<CartItem[]> {
        return await request.get("/cart");
    },
    async addItem(item_id: number): Promise<CartItem[]> {
        return await request.post(`/cart/item/${item_id}`, {});
    },
    async removeItem(item_id: number): Promise<CartItem[]> {
        return await request.delete(`/cart/item/${item_id}`);
    },
    async updateItem(
        item_id: number,
        item: CartItemUpdate
    ): Promise<CartItem[]> {
        return await request.put(`/cart/item/${item_id}`, item);
    },
    async addRecipe(recipe_id: number): Promise<CartItem[]> {
        return await request.post(`/cart/recipe/${recipe_id}`, {});
    },
    async undo(): Promise<CartItem[]> {
        return await request.post("/cart/undo", {});
    },
    async redo(): Promise<CartItem[]> {
        return await request.post("/cart/redo", {});
    },
    async clear() {
        return await request.delete("/cart/events");
    },
    async getStream() {
        const eventSource = new EventSource(PUBLIC_API_URL + "/cart/stream");
        eventSource.onerror = console.error;
        return eventSource;
    },
};
