DROP TABLE rewards;
DROP TABLE visit_reports;
ALTER TABLE attacks_to_hobos DROP COLUMN satisfied;
ALTER TABLE attacks_to_hobos DROP COLUMN released;
ALTER TABLE hobos DROP COLUMN hurried;