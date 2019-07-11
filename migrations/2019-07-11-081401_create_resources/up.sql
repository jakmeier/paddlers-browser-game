CREATE TYPE RESOURCE_TYPE AS ENUM ('feathers', 'sticks', 'logs');
CREATE TABLE resources (
    resource_type RESOURCE_TYPE PRIMARY KEY,
    amount BIGINT NOT NULL
)