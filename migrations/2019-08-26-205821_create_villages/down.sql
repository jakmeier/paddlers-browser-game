ALTER TABLE attacks
DROP COLUMN origin_village_id;

ALTER TABLE attacks
DROP COLUMN destination_village_id;

ALTER TABLE buildings
DROP COLUMN village_id;

ALTER TABLE resources
DROP COLUMN village_id;

ALTER TABLE units
DROP CONSTRAINT units_village_fk;

DROP TABLE villages;