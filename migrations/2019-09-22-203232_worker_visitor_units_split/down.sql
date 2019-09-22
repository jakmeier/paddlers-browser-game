-- Data could be migrated but there is no point for it atm.
DELETE FROM workers;
DELETE FROM hobos;

-- Recreate units table
CREATE TABLE units (
	id bigserial NOT NULL,
	home int8 NOT NULL,
	x int4 NOT NULL,
	y int4 NOT NULL,
	unit_type unit_type NOT NULL,
	color unit_color NULL,
	hp int8 NOT NULL,
	speed float4 NOT NULL,
	mana int4 NULL,
	CONSTRAINT units_pkey PRIMARY KEY (id)
);

ALTER TABLE public.units ADD CONSTRAINT units_village_fk FOREIGN KEY (home) REFERENCES villages(id) ON DELETE CASCADE;


-- hobo to attacks
ALTER TABLE attacks_to_hobos
DROP CONSTRAINT attacks_to_hobos_hobo_id_fkey;
ALTER TABLE attacks_to_hobos
    RENAME COLUMN hobo_id TO unit_id;
ALTER TABLE attacks_to_hobos
    RENAME TO attacks_to_units;
ALTER TABLE attacks_to_units
ADD CONSTRAINT attacks_to_units_unit_id_fkey FOREIGN KEY (unit_id) REFERENCES units(id) ON DELETE CASCADE;

-- Tasks
ALTER TABLE tasks
DROP CONSTRAINT tasks_worker_id_fkey;
ALTER TABLE tasks
    RENAME COLUMN worker_id to unit_id;
ALTER TABLE tasks
ADD CONSTRAINT tasks_unit_id_fkey FOREIGN KEY (unit_id) REFERENCES units(id) ON DELETE CASCADE;
ALTER TABLE tasks
DROP COLUMN target_hobo_id;

-- Abilities
ALTER TABLE abilities
DROP CONSTRAINT abilities_worker_id_fkey;
ALTER TABLE abilities
    RENAME COLUMN worker_id to unit_id;
ALTER TABLE abilities
ADD CONSTRAINT abilities_unit_id_fkey FOREIGN KEY (unit_id) REFERENCES units(id) ON DELETE CASCADE;


DROP TABLE hobos;
DROP TABLE workers;