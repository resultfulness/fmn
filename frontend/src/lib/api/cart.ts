import type { CartItem } from "$lib/schemas/cart";

let lsCart = localStorage.getItem("cart");
const cart = {
    items: JSON.parse(lsCart ?? "[]") as CartItem[],
    maxId: 0,
    readAll(): Promise<CartItem[]> {
        return new Promise(res => setTimeout(() => res(this.items), 100));
    },
};

export default cart;
