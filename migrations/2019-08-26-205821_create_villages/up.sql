CREATE TABLE villages (
    id BIGSERIAL PRIMARY KEY,
    x REAL NOT NULL,
    y REAL NOT NULL,
    stream_id BIGINT NOT NULL REFERENCES streams (id) ON DELETE NO ACTION
);
-- TODO: If not exists, add stream id 1 / 3
-- Insert test village
INSERT INTO villages (x,y,stream_id)
VALUES (2, 5, 1);
INSERT INTO villages (x,y,stream_id)
VALUES (5, 5, 3);

-- Add links to villages to various tables
-- Attacks origin and destination
ALTER TABLE attacks
ADD COLUMN origin_village_id BIGINT NOT NULL REFERENCES villages (id)
-- TODO: Better handle default insert (Add constraint after update with default)
DEFAULT 1;
ALTER TABLE attacks
ADD COLUMN destination_village_id BIGINT NOT NULL REFERENCES villages (id)
DEFAULT 1;
-- Buildings
ALTER TABLE buildings
ADD COLUMN village_id BIGINT NOT NULL REFERENCES villages (id)
ON DELETE CASCADE
DEFAULT 1;
-- Resources
ALTER TABLE resources
ADD COLUMN village_id BIGINT NOT NULL REFERENCES villages (id)
ON DELETE CASCADE
DEFAULT 1;
-- Unit home
ALTER TABLE units
ADD CONSTRAINT units_village_fk
FOREIGN KEY (home)
REFERENCES villages(id) 
ON DELETE CASCADE;