import * as z from "zod";

export const RecipeItem = z.object({
    item_id: z.int(),
    quantity: z.int().positive(),
});

export const Recipe = z.object({
    recipe_id: z.int(),
    name: z.string(),
    icon: z.string(),
    description: z.string(),
    servings: z.int(),
    items: z.array(RecipeItem),
    created_at: z.coerce.date(),
    updated_at: z.coerce.date(),
});

export const RecipeCreate = z.object({
    name: z.string().nonempty(),
    icon: z.string(),
    description: z.string(),
    servings: z.int().positive(),
});

export const RecipeUpdate = z.object({
    name: z.string().optional(),
    icon: z.string().optional(),
    description: z.string().optional(),
    servings: z.int().positive().optional(),
    items: z.array(RecipeItem).optional(),
});

export type RecipeItem = z.infer<typeof RecipeItem>;

export type Recipe = z.infer<typeof Recipe>;
export type RecipeCreate = z.infer<typeof RecipeCreate>;
export type RecipeUpdate = z.infer<typeof RecipeUpdate>;
