ALTER TABLE attacks
ADD COLUMN entered_destination TIMESTAMP DEFAULT NULL;

COMMENT ON COLUMN attacks.entered_destination IS 'Attackers wait in a queue and must be manually let in before they enter the destination village.';