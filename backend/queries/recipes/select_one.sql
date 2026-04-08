SELECT
    "recipe_id",
    "name",
    "icon",
    "description",
    "servings",
    ARRAY_REMOVE(ARRAY_AGG(
        ("item_id", "quantity") ORDER BY "order"
    ), (NULL::INT, NULL::INT)) "items!: Vec<RecipeItem>",
    "created_at",
    "updated_at"
FROM "recipe"
LEFT JOIN "recipe_item" USING ("recipe_id")
WHERE "recipe_id" = $1
GROUP BY "recipe_id"
