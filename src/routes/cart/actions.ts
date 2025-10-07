import cartService from "$lib/services/cart-service";
import itemService from "$lib/services/item-service";
import type { CartItem, Item } from "$lib/types";
import type { Unsubscriber } from "svelte/store";
import data from "./data.svelte";

let unsubscribers: Unsubscriber[] = [];
const actions = {
    async init() {
        unsubscribers.push(cartService.subscribe(
            cartItems => data.cartItems = cartItems
        ));
        try {
            unsubscribers.push(await itemService.subscribe(
                items => data.items = items
            ));
        } catch (e: any) {
            return;
        }
    },
    deinit() {
        for (const unsub of unsubscribers) {
            unsub();
        }
    },
    async handleAddItem(item: Item) {
        try {
            await cartService.addItem(item);
        } catch (e) {
            console.log(e);
        }
    },
    async handleDeleteItem(item: CartItem) {
        try {
            await cartService.deleteItem(item);
        } catch (e) {
            console.log(e);
        }
    }
};

export default actions;
