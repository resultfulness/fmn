SELECT
    "event_id",
    "payload",
    "is_future",
    "executed_at"
FROM "event"
ORDER BY executed_at
