CREATE TYPE TASK_TYPE AS ENUM ('idle', 'walk', 'defend', 'gather_sticks', 'chop_tree', 'welcome_ability');
CREATE TABLE tasks (
    id BIGSERIAL PRIMARY KEY,
    unit_id BIGINT NOT NULL REFERENCES units (id) ON DELETE CASCADE,
    task_type TASK_TYPE NOT NULL,
    x INT NOT NULL,
    y INT NOT NULL,
    start_time TIMESTAMP NOT NULL DEFAULT NOW()
);