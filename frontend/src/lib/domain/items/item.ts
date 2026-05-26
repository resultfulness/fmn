import * as z from "zod";
import { Unit } from "./units";

export const Item = z.object({
    item_id: z.int(),
    name: z.string(),
    icon: z.string(),
    unit: Unit,
    created_at: z.coerce.date(),
    updated_at: z.coerce.date(),
});

export const ItemCreate = z.object({
    name: z.string().nonempty(),
    icon: z.string(),
    unit: Unit,
});

export const ItemUpdate = z.object({
    name: z.string().nonempty().optional(),
    icon: z.string().optional(),
    unit: Unit.optional(),
});

export type Item = z.infer<typeof Item>;
export type ItemCreate = z.infer<typeof ItemCreate>;
export type ItemUpdate = z.infer<typeof ItemUpdate>;
