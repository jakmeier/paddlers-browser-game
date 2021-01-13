DELETE FROM pg_enum
    WHERE pg_enum.enumtypid = 'building_type'::regtype::oid
    AND pg_enum.enumlabel = 'watergate';