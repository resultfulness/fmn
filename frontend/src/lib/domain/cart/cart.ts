import * as z from "zod";
import { Unit } from "../items/units";
import type { Item } from "../items/item";

export const CartItem = z.object({
    item_id: z.int(),
    description: z.string().optional(),
    quantity: z.number().positive().optional(),
});

export const CartItemDisplay = z.object({
    item_id: z.int(),
    description: z.string().optional(),
    quantity: z.number().positive().optional(),
    name: z.string().nonempty(),
    icon: z.string(),
    unit: Unit,
});

export function cartItemDisplay(
    item: Item,
    cartItem: CartItem
): CartItemDisplay {
    return {
        item_id: item.item_id,
        description: cartItem.description,
        quantity: cartItem.quantity,
        name: item.name,
        icon: item.icon,
        unit: item.unit,
    };
}

export function cartItemDisplays(
    items: Item[],
    cartItems: CartItem[]
): CartItemDisplay[] {
    let cartItemDisplays = [];

    for (const cartItem of cartItems) {
        let fullItem = items.find(
            ({ item_id }) => item_id === cartItem.item_id
        );
        if (fullItem === undefined) return [];

        cartItemDisplays.push(cartItemDisplay(fullItem, cartItem));
    }

    return cartItemDisplays;
}

export const CartItemUpdate = z.object({
    description: z.string().optional(),
    quantity: z.number().positive().optional(),
});

export type CartItem = z.infer<typeof CartItem>;
export type CartItemUpdate = z.infer<typeof CartItemUpdate>;
export type CartItemDisplay = z.infer<typeof CartItemDisplay>;
