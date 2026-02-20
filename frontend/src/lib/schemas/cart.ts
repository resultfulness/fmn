import * as z from "zod";

export const CartItem = z.object({
    icon: z.string(),
    name: z.string(),
    unit: z.string(),
    quantity: z.number().positive().optional()
});

export type CartItem = z.infer<typeof CartItem>;
