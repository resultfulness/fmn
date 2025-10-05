import cartService from "$lib/services/cart-service";
import itemService from "$lib/services/item-service";
import type { CartItem, Item } from "$lib/types";
import data from "./data.svelte";

const actions = {
    init() {
        this.loadItems();
        this.deinit = cartService.subscribe(
            cartItems => data.cartItems = cartItems
        );
    },
    deinit() { },
    async loadItems() {
        const items = itemService.getAll();
        if (items) {
            data.items = items;
        }
        await itemService.fetchAll();
        data.items = itemService.getAll();
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
