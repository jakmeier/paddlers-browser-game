DELETE FROM pg_enum WHERE pg_enum.enumtypid = 'story_state_type'::regtype::oid AND pg_enum.enumlabel = 'unlocking_invitation_path_b';
DELETE FROM pg_enum WHERE pg_enum.enumtypid = 'story_state_type'::regtype::oid AND pg_enum.enumlabel = 'unlocking_invitation_path_a';
DELETE FROM pg_enum WHERE pg_enum.enumtypid = 'story_state_type'::regtype::oid AND pg_enum.enumlabel = 'welcome_visitor_quest_started';
DELETE FROM pg_enum WHERE pg_enum.enumtypid = 'story_state_type'::regtype::oid AND pg_enum.enumlabel = 'visitor_queued';
