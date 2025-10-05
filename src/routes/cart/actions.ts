import cartService from "$lib/services/cart-service";
import itemService from "$lib/services/item-service";
import data from "./data.svelte";

const actions = {
    init() {
        this.loadItems();
        cartService.init();
        this.deinit = cartService.subscribe(cartItems => data.cartItems = cartItems);
    },
    deinit() { },
    async loadItems() {
        const items = itemService.getAll();
        if (!items) {
            await itemService.fetchAll();
            data.items = itemService.getAll();
        } else {
            data.items = items;
            await itemService.fetchAll();
        }
    },
};

export default actions;
