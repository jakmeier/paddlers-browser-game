ALTER TABLE quests DROP COLUMN follow_up_quest;
ALTER TABLE quests DROP COLUMN pop_condition;
ALTER TABLE quests ADD COLUMN next_story_state STORY_STATE_TYPE;

DELETE FROM pg_enum
    WHERE pg_enum.enumtypid = 'story_state_type'::regtype::oid
    AND (
        pg_enum.enumlabel = 'dialogue_balance_a'
     OR pg_enum.enumlabel = 'dialogue_balance_b'
    );

ALTER TYPE STORY_STATE_TYPE RENAME VALUE 'solving_secondary_quest_a' TO 'solving_secondary_quest_part_a';
ALTER TYPE STORY_STATE_TYPE RENAME VALUE 'solving_secondary_quest_b' TO 'solving_secondary_quest_part_b';

ALTER TYPE STORY_STATE_TYPE ADD VALUE 'tree_planted';
ALTER TYPE STORY_STATE_TYPE ADD VALUE 'gathering_sticks';
ALTER TYPE STORY_STATE_TYPE ADD VALUE 'stick_gathering_station_build';
ALTER TYPE STORY_STATE_TYPE ADD VALUE 'flower_planted';
ALTER TYPE STORY_STATE_TYPE ADD VALUE 'more_happy_visitors';
ALTER TYPE STORY_STATE_TYPE ADD VALUE 'picking_secondary_civ_bonus';