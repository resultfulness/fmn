INSERT INTO "item" ("name", "icon", "unit")
VALUES ($1, $2, $3)
RETURNING "item_id"
