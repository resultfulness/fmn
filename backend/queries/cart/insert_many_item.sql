INSERT INTO "cart_item" ("item_id", "description", "quantity", "order")
SELECT * FROM UNNEST($1::INT[], $2::TEXT[], $3::INT[], $4::INT[])
ON CONFLICT DO NOTHING
