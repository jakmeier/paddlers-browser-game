INSERT INTO pg_enum (enumtypid, enumlabel, enumsortorder)
    SELECT 'building_type'::regtype::oid, 'temple', 
    ( SELECT MAX(enumsortorder) + 1 FROM pg_enum WHERE enumtypid = 'building_type'::regtype );
INSERT INTO pg_enum (enumtypid, enumlabel, enumsortorder)
    SELECT 'unit_color'::regtype::oid, 'prophet', 
    ( SELECT MAX(enumsortorder) + 1 FROM pg_enum WHERE enumtypid = 'unit_color'::regtype );