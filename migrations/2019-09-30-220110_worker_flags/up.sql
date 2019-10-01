CREATE TYPE WORKER_FLAG_TYPE AS ENUM ('mana_regeneration', 'work');

-- This table might look like it should be merged with workers to better follow normilzation practices.
-- However, these fields are used mostly independent of the other workers data and they are written much more often.
-- The empirical evidence is still outstanding at the time of writing, though.
CREATE TABLE worker_flags (
	worker_id BIGINT NOT NULL REFERENCES workers(id) ON DELETE CASCADE,
	flag_type WORKER_FLAG_TYPE NOT NULL,
    last_update TIMESTAMP NOT NULL,
    CONSTRAINT worker_flags_pk PRIMARY KEY (worker_id, flag_type)
);
