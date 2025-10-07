import { PUBLIC_API_URL } from "$env/static/public";
import type { CartItem } from "$lib/types";
import { apiFetch } from "./fetch";

const eventSource = new EventSource(`${PUBLIC_API_URL}/cart/stream`);

const cartApi = {
    async getAll(): Promise<CartItem[]> {
        return await (await apiFetch("/cart/items")).json();
    },
    async addItem(id: number): Promise<CartItem[]> {
        return await (await apiFetch(`/cart/items/${id}`, "PUT")).json();
    },
    async deleteItem(id: number, origin: string): Promise<CartItem[]> {
        return await (
            await apiFetch(`/cart/items/${id}?origin=${origin}`, "DELETE")
        ).json();
    },
    stream(callback: (data: CartItem[]) => void) {
        eventSource.addEventListener("cart", e => {
            callback(JSON.parse(e.data));
        });
    },
};

window.addEventListener("beforeunload", () => {
    if (eventSource) {
        eventSource.close();
    }
})

export default cartApi;
