SELECT
    "item_id",
    "name",
    "icon",
    "unit",
    "created_at",
    "updated_at"
FROM "item"
WHERE "item_id" = $1
