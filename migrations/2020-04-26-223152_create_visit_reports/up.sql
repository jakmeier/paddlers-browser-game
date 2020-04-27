-- Create visit reports and feature for waiting attackers

ALTER TABLE hobos 
ADD COLUMN hurried BOOLEAN NOT NULL DEFAULT TRUE;

ALTER TABLE attacks_to_hobos 
ADD COLUMN satisfied BOOLEAN;

ALTER TABLE attacks_to_hobos 
ADD COLUMN released TIMESTAMP;

CREATE TABLE visit_reports (
    id BIGSERIAL PRIMARY KEY,
    village_id BIGSERIAL NOT NULL REFERENCES villages(id),
    reported TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    karma BIGINT NOT NULL
);

CREATE TABLE rewards (
    id BIGSERIAL PRIMARY KEY,
    visit_report_id BIGSERIAL NOT NULL REFERENCES visit_reports(id),
    resource_type RESOURCE_TYPE NOT NULL,
    amount BIGINT NOT NULL
);
