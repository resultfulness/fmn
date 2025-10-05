import type { Item } from "$lib/types";

const data: {
    items: Item[]
} = $state({
    items: [],
});

export default data;
