UPDATE "recipe"
SET
    "name" = COALESCE($2, "name"),
    "icon" = COALESCE($3, "icon"),
    "description" = COALESCE($4, "description"),
    "servings" = COALESCE($5, "servings")
WHERE "recipe_id" = $1
