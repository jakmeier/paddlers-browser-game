DELETE FROM pg_enum WHERE pg_enum.enumtypid = 'story_state_type'::regtype::oid AND pg_enum.enumlabel = 'welcome_visitor_quest_started';
DELETE FROM pg_enum WHERE pg_enum.enumtypid = 'story_state_type'::regtype::oid AND pg_enum.enumlabel = 'visitor_queued';
