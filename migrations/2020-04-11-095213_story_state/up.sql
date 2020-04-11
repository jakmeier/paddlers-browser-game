CREATE TYPE STORY_STATE_TYPE AS ENUM (
  'initialized',
  'servant_accepted',
  'temple_built',
  'visitor_arrived',
  'first_visitor_welcomed',
  'flower_planted',
  'more_happy_visitors',
  'tree_planted',
  'stick_gathering_station_build',
  'gathering_sticks'
);

ALTER TABLE players
ADD COLUMN story_state STORY_STATE_TYPE NOT NULL DEFAULT 'initialized';
