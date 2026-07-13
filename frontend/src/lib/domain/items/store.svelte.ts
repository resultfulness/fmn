import api from "$lib/api";
import type { Item } from "./item";

const itemStore = $state({
    items: <Item[]>[],
    async load() {
        this.items = await api.items.readAll();
    },
});

export default itemStore;
