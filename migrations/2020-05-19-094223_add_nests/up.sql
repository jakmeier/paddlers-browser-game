ALTER TYPE building_type ADD VALUE 'single_nest';
ALTER TYPE building_type ADD VALUE 'triple_nest';

ALTER TABLE hobos
ADD COLUMN nest BIGINT NULL REFERENCES buildings (id) ON DELETE SET NULL DEFAULT NULL;
