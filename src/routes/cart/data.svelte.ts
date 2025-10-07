import type { CartItem, Item } from "$lib/types";

const data: {
    cartItems: CartItem[],
    items: Item[],
} = $state({
    cartItems: [],
    items: [],
});

export default data;
