UPDATE players
SET display_name = 'PLAYER#' || id
WHERE display_name IS NULL;

ALTER TABLE players
ALTER COLUMN display_name 
SET NOT NULL;