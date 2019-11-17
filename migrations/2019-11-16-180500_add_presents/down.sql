DELETE FROM pg_enum
    WHERE pg_enum.enumtypid = 'building_type'::regtype::oid
    AND pg_enum.enumlabel = 'pressent_b';
DELETE FROM pg_enum
    WHERE pg_enum.enumtypid = 'building_type'::regtype::oid
    AND pg_enum.enumlabel = 'pressent_a';