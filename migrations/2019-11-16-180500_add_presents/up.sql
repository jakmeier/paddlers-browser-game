ALTER TYPE building_type ADD VALUE 'present_a';
ALTER TYPE building_type ADD VALUE 'present_b';
ALTER TYPE task_type ADD VALUE 'collect_reward';

-- If you must use another version of pg than paddlers main, then it might not be possible to run above code in a transaction.
-- This is an example for the be a workaround that has been used before updating pg.
-- INSERT INTO pg_enum (enumtypid, enumlabel, enumsortorder)
--     SELECT 'building_type'::regtype::oid, 'present_a', 
--     ( SELECT MAX(enumsortorder) + 1 FROM pg_enum WHERE enumtypid = 'building_type'::regtype );