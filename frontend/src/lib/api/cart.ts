import type { CartItem, CartItemUpdate } from "$lib/domain/cart/cart";
import request from "./request";

export default {
    readAll(): Promise<CartItem[]> {
        return request.get("/cart");
    },
    addItem(item_id: number): Promise<CartItem[]> {
        return request.post(`/cart/item/${item_id}`, {});
    },
    removeItem(item_id: number): Promise<CartItem[]> {
        return request.delete(`/cart/item/${item_id}`);
    },
    updateItem(item_id: number, item: CartItemUpdate): Promise<CartItem[]> {
        return request.put(`/cart/item/${item_id}`, item);
    },
    addRecipe(recipe_id: number): Promise<CartItem[]> {
        return request.post(`/cart/recipe/${recipe_id}`, {});
    },
    undo(): Promise<CartItem[]> {
        return request.post("/cart/undo", {});
    },
    redo(): Promise<CartItem[]> {
        return request.post("/cart/redo", {});
    },
    clear() {
        return request.delete("/cart/events");
    },
};
