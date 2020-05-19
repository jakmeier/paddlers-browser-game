INSERT INTO pg_enum (enumtypid, enumlabel, enumsortorder)
    SELECT 'building_type'::regtype::oid, 'single_nest', 
    ( SELECT MAX(enumsortorder) + 1 FROM pg_enum WHERE enumtypid = 'building_type'::regtype );

INSERT INTO pg_enum (enumtypid, enumlabel, enumsortorder)
    SELECT 'building_type'::regtype::oid, 'triple_nest', 
    ( SELECT MAX(enumsortorder) + 1 FROM pg_enum WHERE enumtypid = 'building_type'::regtype );


ALTER TABLE hobos
ADD COLUMN nest BIGINT NULL REFERENCES buildings (id) ON DELETE SET NULL DEFAULT NULL;
