CREATE TABLE attacks_to_units (
  attack_id BIGINT NOT NULL REFERENCES attacks (id) ON DELETE CASCADE,
  unit_id BIGINT NOT NULL REFERENCES units (id) ON DELETE CASCADE,
  CONSTRAINT attacks_to_units_pk PRIMARY KEY (attack_id, unit_id)
)