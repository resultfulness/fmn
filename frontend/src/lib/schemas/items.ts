import * as z from "zod";

export const UNITS = ["tbsp", "tsp", "g", "ml", "pcs", "pkgs"] as const;
export const Unit = z.enum(UNITS);

export function unitHumanReadable(u: Unit): string {
    switch (u) {
        case "tbsp": return "tablespoons";
        case "tsp": return "teaspoons";
        case "g": return "grams";
        case "ml": return "mililiters";
        case "pcs": return "pieces";
        case "pkgs": return "packages";
    }
}

export function unitPretty(u: Unit): string {
    return `${unitHumanReadable(u)} (${u})`;
}

export const ItemShort = z.object({
    item_id: z.int(),
    name: z.string(),
    icon: z.string(),
    unit: Unit,
});

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
})

export type Unit = z.infer<typeof Unit>;
export type Item = z.infer<typeof Item>;
export type ItemShort = z.infer<typeof ItemShort>;
export type ItemCreate = z.infer<typeof ItemCreate>;
export type ItemUpdate = z.infer<typeof ItemUpdate>;
