INSERT INTO "recipe" ("name", "icon", "description", "servings")
VALUES ($1, $2, $3, $4)
RETURNING "recipe_id"
