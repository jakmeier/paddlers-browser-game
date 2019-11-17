-- Forced to do something this to add new enum variant until Diesel supports
-- migration outside of transactions (or postgreSQL allows ALTER TYPE inside 
-- transactions)

-- Adding presents as building types
INSERT INTO pg_enum (enumtypid, enumlabel, enumsortorder)
    SELECT 'building_type'::regtype::oid, 'present_a', 
    ( SELECT MAX(enumsortorder) + 1 FROM pg_enum WHERE enumtypid = 'building_type'::regtype );
INSERT INTO pg_enum (enumtypid, enumlabel, enumsortorder)
    SELECT 'building_type'::regtype::oid, 'present_b', 
    ( SELECT MAX(enumsortorder) + 1 FROM pg_enum WHERE enumtypid = 'building_type'::regtype );

-- Add new task type
INSERT INTO pg_enum (enumtypid, enumlabel, enumsortorder)
    SELECT 'task_type'::regtype::oid, 'collect_reward', 
    ( SELECT MAX(enumsortorder) + 1 FROM pg_enum WHERE enumtypid = 'task_type'::regtype );