DROP TABLE rewards;
DROP TABLE visit_reports;
ALTER TABLE attacks_to_hobos DROP COLUMN position;
DROP TYPE JOURNEY_POSITION;
ALTER TABLE hobos DROP COLUMN hurried;