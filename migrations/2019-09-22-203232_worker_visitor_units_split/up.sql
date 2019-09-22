-- Data could be migrated but there is no point for it atm.
DELETE FROM units;

-- Units that work in town and may be used to attack other towns.
CREATE TABLE workers (
	id BIGSERIAL NOT NULL PRIMARY KEY,
	home BIGINT NOT NULL REFERENCES villages(id) ON DELETE CASCADE,
	x INT NOT NULL,
	y INT NOT NULL,
	unit_type unit_type NOT NULL,
	color unit_color NULL,
	speed REAL NOT NULL,
	mana INT NULL
);

-- Hobos live in shelters and don't work. They drop some loot to 
-- other player when being invited to a town they like.
-- Hobos are also called visitors when in a foreign town.
CREATE TABLE hobos (
	id BIGSERIAL NOT NULL PRIMARY KEY,
	home BIGINT NOT NULL REFERENCES villages(id) ON DELETE CASCADE,
	color unit_color NULL,
	speed REAL NOT NULL,
	hp INT8 NOT NULL
);

-- hobo to attacks
ALTER TABLE attacks_to_units
DROP CONSTRAINT attacks_to_units_unit_id_fkey;
ALTER TABLE attacks_to_units
    RENAME COLUMN unit_id TO hobo_id;
ALTER TABLE attacks_to_units
    RENAME TO attacks_to_hobos;
ALTER TABLE attacks_to_hobos
ADD CONSTRAINT attacks_to_hobos_hobo_id_fkey FOREIGN KEY (hobo_id) REFERENCES hobos(id) ON DELETE CASCADE;

-- Tasks
ALTER TABLE tasks
DROP CONSTRAINT tasks_unit_id_fkey;
ALTER TABLE tasks
    RENAME COLUMN unit_id TO worker_id;
ALTER TABLE tasks
ADD CONSTRAINT tasks_worker_id_fkey FOREIGN KEY (worker_id) REFERENCES workers(id) ON DELETE CASCADE;
-- Also add target to tasks while we are at it
ALTER TABLE tasks
ADD COLUMN target_hobo_id BIGINT REFERENCES hobos(id) ON DELETE CASCADE; -- may be null

-- Abilities
ALTER TABLE abilities
DROP CONSTRAINT abilities_unit_id_fkey;
ALTER TABLE abilities
    RENAME COLUMN unit_id TO worker_id;
ALTER TABLE abilities
ADD CONSTRAINT abilities_worker_id_fkey FOREIGN KEY (worker_id) REFERENCES workers(id) ON DELETE CASCADE;


DROP TABLE units;