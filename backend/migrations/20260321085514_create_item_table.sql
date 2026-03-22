CREATE TABLE "item" (
    "item_id" SERIAL PRIMARY KEY,
    "name" TEXT NOT NULL UNIQUE,
    "icon" TEXT NOT NULL,
    "unit" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
)
