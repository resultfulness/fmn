import state from "./state.svelte";
import itemService from "$lib/services/item-service";

const actions = {
    init() {
        this.loadItems();
        this.deinit = itemService.subscribe(items => state.items = items);
    },
    deinit() { },
    loadItems() {
        state.items = itemService.getAll();
    }
};

export default actions;
