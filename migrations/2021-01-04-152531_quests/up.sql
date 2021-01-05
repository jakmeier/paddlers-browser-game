CREATE TABLE quests (
    id BIGSERIAL PRIMARY KEY,
    quest_key VARCHAR(64) NOT NULL,
    next_story_state STORY_STATE_TYPE,
    karma_condition BIGINT
);
CREATE UNIQUE INDEX quest_key_idx ON quests (quest_key);

CREATE TABLE quest_res_rewards (
    id BIGSERIAL PRIMARY KEY,
    quest_id BIGSERIAL NOT NULL REFERENCES quests(id) ON DELETE CASCADE,
    resource_type RESOURCE_TYPE NOT NULL,
    amount BIGINT NOT NULL
);

CREATE TABLE quest_res_conditions (
    id BIGSERIAL PRIMARY KEY,
    quest_id BIGSERIAL NOT NULL REFERENCES quests(id) ON DELETE CASCADE,
    resource_type RESOURCE_TYPE NOT NULL,
    amount BIGINT NOT NULL
);

CREATE TABLE quest_building_conditions (
    id BIGSERIAL PRIMARY KEY,
    quest_id BIGSERIAL NOT NULL REFERENCES quests(id) ON DELETE CASCADE,
    building_type BUILDING_TYPE NOT NULL,
    amount BIGINT NOT NULL
);

CREATE TABLE quest_worker_conditions (
    id BIGSERIAL PRIMARY KEY,
    quest_id BIGSERIAL NOT NULL REFERENCES quests(id) ON DELETE CASCADE,
    task_type TASK_TYPE NOT NULL,
    amount BIGINT NOT NULL
);


CREATE TABLE quest_to_player (
  quest_id BIGINT NOT NULL REFERENCES quests (id) ON DELETE CASCADE,
  player_id BIGINT NOT NULL REFERENCES players (id) ON DELETE CASCADE,
  CONSTRAINT quest_to_player_pk PRIMARY KEY (quest_id, player_id)
);
