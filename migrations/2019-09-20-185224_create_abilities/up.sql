CREATE TYPE ABILITY_TYPE AS ENUM ('welcome', 'work');
CREATE TABLE abilities (
    ability_type ABILITY_TYPE NOT NULL,
    unit_id BIGINT NOT NULL REFERENCES units (id) ON DELETE CASCADE,
    last_used TIMESTAMP DEFAULT NULL
);
ALTER TABLE abilities
ADD CONSTRAINT abilities_pk
PRIMARY KEY (ability_type, unit_id);

ALTER TABLE units
ADD COLUMN mana INT;