ALTER TYPE building_type ADD VALUE 'temple';
ALTER TYPE unit_color ADD VALUE 'prophet';
ALTER TABLE villages
ADD COLUMN faith SMALLINT NOT NULL DEFAULT 100;