CREATE TABLE "cart_item" (
    "item_id" INTEGER NOT NULL REFERENCES "item" ON DELETE CASCADE,
    "description" TEXT,
    "quantity" INTEGER,
    "order" INTEGER NOT NULL
);

CREATE TABLE "event" (
    "event_id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "payload" JSONB NOT NULL,
    "is_future" BOOL NOT NULL DEFAULT FALSE,
    "executed_at" TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);
