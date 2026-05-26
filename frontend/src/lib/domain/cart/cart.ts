import * as z from "zod";

export const CartItem = z.object({
    item_id: z.int(),
    description: z.string().optional(),
    quantity: z.number().positive().optional(),
});

export const CartItemUpdate = z.object({
    description: z.string().optional(),
    quantity: z.number().positive().optional(),
});

export type CartItem = z.infer<typeof CartItem>;
export type CartItemUpdate = z.infer<typeof CartItemUpdate>;
