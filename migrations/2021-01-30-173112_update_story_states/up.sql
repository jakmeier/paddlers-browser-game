DELETE FROM pg_enum
    WHERE pg_enum.enumtypid = 'story_state_type'::regtype::oid
    AND (
       pg_enum.enumlabel = 'stick_gathering_station_build'
    OR pg_enum.enumlabel = 'gathering_sticks'
    OR pg_enum.enumlabel = 'tree_planted'
    OR pg_enum.enumlabel = 'flower_planted'
    OR pg_enum.enumlabel = 'more_happy_visitors'
    OR pg_enum.enumlabel = 'picking_secondary_civ_bonus'
    );
ALTER TYPE STORY_STATE_TYPE RENAME VALUE 'solving_secondary_quest_part_a' TO 'solving_secondary_quest_a';
ALTER TYPE STORY_STATE_TYPE RENAME VALUE 'solving_secondary_quest_part_b' TO 'solving_secondary_quest_b';
ALTER TYPE STORY_STATE_TYPE ADD VALUE 'dialogue_balance_a';
ALTER TYPE STORY_STATE_TYPE ADD VALUE 'dialogue_balance_b';

ALTER TABLE quests DROP COLUMN next_story_state;
ALTER TABLE quests ADD COLUMN pop_condition BIGINT DEFAULT NULL;
ALTER TABLE quests ADD COLUMN follow_up_quest VARCHAR(64) DEFAULT NULL;
