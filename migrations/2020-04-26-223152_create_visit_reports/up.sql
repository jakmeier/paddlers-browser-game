-- Create visit reports and feature for waiting attackers

ALTER TABLE hobos 
ADD COLUMN hurried BOOLEAN NOT NULL DEFAULT TRUE;

CREATE TYPE JOURNEY_POSITION AS ENUM (
    'travelling',
    'visiting',
    'waiting',
    'gone'
);

ALTER TABLE attacks_to_hobos 
ADD COLUMN position JOURNEY_POSITION NOT NULL DEFAULT 'travelling';

CREATE TABLE visit_reports (
    id BIGSERIAL PRIMARY KEY,
    village_id BIGSERIAL NOT NULL REFERENCES villages(id),
    reported TIMESTAMP NOT NULL,
    karma BIGINT NOT NULL
);

CREATE TABLE rewards (
    id BIGSERIAL PRIMARY KEY,
    visit_report_id BIGSERIAL NOT NULL REFERENCES visit_reports(id),
    resource_type RESOURCE_TYPE NOT NULL,
    amount BIGINT NOT NULL
);
