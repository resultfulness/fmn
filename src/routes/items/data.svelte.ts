import type { Items } from "$lib/types";

const data: {
    items: Items
} = $state({
    items: {
        count: 0,
        items: [],
    }
});

export default data;
