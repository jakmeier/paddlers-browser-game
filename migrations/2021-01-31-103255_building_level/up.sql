ALTER TABLE buildings
ADD COLUMN lv INT NOT NULL;

COMMENT ON COLUMN buildings.lv IS 'Building level';