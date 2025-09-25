import type { Item } from "$lib/types";

const state: {
    items: Item[]
} = $state({
    items: [],
});

export default state;
