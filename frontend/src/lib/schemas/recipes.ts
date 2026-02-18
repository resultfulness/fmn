import * as z from "zod";

export const RecipeShort = z.object({
    recipe_id: z.int(),
    name: z.string(),
    servings: z.int(),
    icon: z.string(),
});
export const Recipe = z.object({
    recipe_id: z.int(),
    name: z.string(),
    servings: z.int(),
    icon: z.string(),
    created_at: z.coerce.date(),
    updated_at: z.coerce.date(),
});
export type RecipeShort = z.infer<typeof RecipeShort>;
export type Recipe = z.infer<typeof Recipe>;
