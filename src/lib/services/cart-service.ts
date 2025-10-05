import cartApi from "$lib/api/cart-api";
import cartStore from "$lib/stores/cart-store";
import type { CartItem } from "$lib/types/cart";
import type { Subscriber } from "svelte/store";

const cartService = {
    init() {
        cartApi.openStream();
        cartApi.subscribe(cartItems => cartStore.set(cartItems));
    },
    subscribe(callback: Subscriber<CartItem[]>) {
        return cartStore.subscribe(callback);
    }
};

export default cartService;
