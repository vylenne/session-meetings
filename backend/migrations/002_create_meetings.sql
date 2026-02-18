CREATE TABLE meetings (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_name   VARCHAR(255) UNIQUE NOT NULL,
    title       VARCHAR(500),
    creator_id  UUID NOT NULL REFERENCES users(id),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
