ALTER TABLE villages
DROP COLUMN faith;
DELETE FROM pg_enum
    WHERE pg_enum.enumtypid = 'unit_color'::regtype::oid
    AND pg_enum.enumlabel = 'prophet';
DELETE FROM pg_enum
    WHERE pg_enum.enumtypid = 'building_type'::regtype::oid
    AND pg_enum.enumlabel = 'temple';