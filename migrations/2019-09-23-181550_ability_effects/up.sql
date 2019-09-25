-- Attributes that a hobo has. Used to describe effects .
CREATE TYPE HOBO_ATTRIBUTE_TYPE AS ENUM ('health', 'speed');
-- Effects from abilities onto hobos
CREATE TABLE effects (
    id BIGSERIAL PRIMARY KEY,
    hobo_id BIGINT NOT NULL REFERENCES hobos (id) ON DELETE CASCADE,
    attribute HOBO_ATTRIBUTE_TYPE NOT NULL,
    strength INT,
    start_time TIMESTAMP NOT NULL DEFAULT NOW() -- When the effect has started to be in effect
);