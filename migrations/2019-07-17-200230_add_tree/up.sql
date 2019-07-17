-- ALTER TYPE building_type ADD VALUE 'tree';
ALTER TABLE buildings
ADD COLUMN creation TIMESTAMP NOT NULL;