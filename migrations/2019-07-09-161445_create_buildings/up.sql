CREATE TYPE BUILDING_TYPE AS ENUM ('blue_flowers', 'red_flowers', 'tree', 'bundling_station');
CREATE TABLE buildings (
    id BIGSERIAL PRIMARY KEY,
    x INT NOT NULL,
    y INT NOT NULL,
    building_type BUILDING_TYPE NOT NULL,
    building_range REAL,
    attack_power REAL,
    attacks_per_cycle INT,
    creation TIMESTAMP NOT NULL
)