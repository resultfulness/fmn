INSERT INTO "recipe_item" ("recipe_id", "item_id", "quantity", "order")
SELECT $1, * FROM UNNEST($2::INT[], $3::INT[], $4::INT[])
