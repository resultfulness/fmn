import * as z from "zod";

export const ALL_UNITS = ["tbsp", "tsp", "g", "ml", "pcs", "pkgs"] as const;
export const Unit = z.enum(ALL_UNITS);
export type Unit = z.infer<typeof Unit>;

const UNIT_LONG: Record<Unit, string> = {
    "tbsp": "tablespoons",
    "tsp": "teaspoons",
    "g": "grams",
    "ml": "mililiters",
    "pcs": "pieces",
    "pkgs": "packages",
}

export function unitLong(u: Unit): string {
    return UNIT_LONG[u];
}

export function unitPretty(u: Unit): string {
    return `${unitLong(u)} (${u})`;
}
