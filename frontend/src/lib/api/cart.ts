import type { CartItem } from "$lib/schemas/cart";

let lsCart = localStorage.getItem("cart");
let items = JSON.parse(lsCart ?? "[]") as CartItem[];
let maxId = items.length;

const cart = {
    readAll(): Promise<CartItem[]> {
        return new Promise(res => setTimeout(() => res(items), 100));
    },
};

export default cart;
