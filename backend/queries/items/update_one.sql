UPDATE "item"
SET
    "name" = COALESCE($2, "name"),
    "icon" = COALESCE($3, "icon"),
    "unit" = COALESCE($4, "unit")
WHERE "item_id" = $1
