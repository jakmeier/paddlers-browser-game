DELETE FROM attacks
WHERE origin_village_id IS NULL;
ALTER TABLE attacks
ALTER COLUMN origin_village_id
SET NOT NULL;