CREATE TABLE streams (
    id BIGSERIAL PRIMARY KEY,
    start_x REAL NOT NULL,
    control_points REAL[] NOT NULL
);