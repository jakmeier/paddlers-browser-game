CREATE TABLE players (
    id BIGSERIAL PRIMARY KEY,
    karma BIGINT NOT NULL,
    display_name VARCHAR(128)
);

ALTER TABLE villages
-- NULL means the village is owned by the AI
ADD COLUMN player_id BIGINT REFERENCES players (id);