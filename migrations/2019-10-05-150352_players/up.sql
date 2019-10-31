CREATE TABLE players (
    id BIGSERIAL PRIMARY KEY,
    uuid UUID NOT NULL,
    karma BIGINT NOT NULL,
    display_name VARCHAR(128)
);

ALTER TABLE villages
-- NULL means the village is owned by the AI
ADD COLUMN player_id BIGINT REFERENCES players (id);

CREATE UNIQUE INDEX players_uuid_idx ON players (uuid);