ALTER TABLE attacks
DROP COLUMN origin_village_id;

ALTER TABLE attacks
DROP COLUMN destination_village_id;

ALTER TABLE buildings
DROP COLUMN village_id;

DELETE FROM resources;
ALTER TABLE resources
DROP CONSTRAINT resources_pk;
ALTER TABLE resources
ADD CONSTRAINT resources_pkey
PRIMARY KEY (resource_type);
ALTER TABLE resources
DROP COLUMN village_id;

ALTER TABLE units
DROP CONSTRAINT units_village_fk;

DROP TABLE villages;