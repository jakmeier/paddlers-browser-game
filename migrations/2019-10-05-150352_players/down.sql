DROP INDEX players_uuid_idx;

ALTER TABLE villages
DROP COLUMN player_id;

DROP TABLE players;