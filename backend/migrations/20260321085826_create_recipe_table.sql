CREATE TABLE "recipe" (
    "recipe_id" SERIAL PRIMARY KEY,
    "name" TEXT NOT NULL UNIQUE,
    "icon" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "servings" INT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE "recipe_item" (
    "recipe_id" INTEGER NOT NULL REFERENCES "recipe" ON DELETE CASCADE,
    "item_id" INTEGER NOT NULL REFERENCES "item" ON DELETE CASCADE,
    "quantity" INTEGER NOT NULL,
    "order" INTEGER NOT NULL
)
