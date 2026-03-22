UPDATE "event"
SET "is_future" = $2
WHERE "event_id" = $1
