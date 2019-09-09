CREATE TABLE villages (
    id BIGSERIAL PRIMARY KEY,
    x REAL NOT NULL,
    y REAL NOT NULL,
    stream_id BIGINT NOT NULL REFERENCES streams (id) ON DELETE NO ACTION
);

-- Add links to villages to various tables
-- First, remove all data from table which have new constraints
DELETE FROM attacks;
DELETE FROM buildings;
DELETE FROM resources;
DELETE FROM units;
-- Attacks origin and destination
ALTER TABLE attacks
ADD COLUMN origin_village_id BIGINT NOT NULL REFERENCES villages (id);
ALTER TABLE attacks
ADD COLUMN destination_village_id BIGINT NOT NULL REFERENCES villages (id);
-- Buildings
ALTER TABLE buildings
ADD COLUMN village_id BIGINT NOT NULL REFERENCES villages (id)
ON DELETE CASCADE;
-- Resources
ALTER TABLE resources
ADD COLUMN village_id BIGINT NOT NULL REFERENCES villages (id)
ON DELETE CASCADE;
ALTER TABLE resources
DROP CONSTRAINT resources_pkey;
ALTER TABLE resources
ADD CONSTRAINT resources_pk
PRIMARY KEY (resource_type, village_id);
-- Unit home
ALTER TABLE units
ADD CONSTRAINT units_village_fk
FOREIGN KEY (home)
REFERENCES villages(id) 
ON DELETE CASCADE;