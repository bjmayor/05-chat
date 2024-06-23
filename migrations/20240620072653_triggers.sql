-- Add migration script here
-- if chat changed, notify with chat data
-- define add_to_chat function
CREATE OR REPLACE FUNCTION add_to_chat() RETURNS TRIGGER AS $$ BEGIN -- notice log
	RAISE NOTICE 'add_to_chat: %',
	NEW;
PERFORM pg_notify(
	'chat_updated',
	json_build_object('op', TG_OP, 'old', OLD, 'new', NEW)::text
);
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
-- define trigger to call add_to_chat function
CREATE TRIGGER add_to_chat_trigger
AFTER
INSERT
	OR
UPDATE
	OR DELETE ON chats FOR EACH ROW EXECUTE FUNCTION add_to_chat();
-- if new message added, notify with message data
-- define add_to_message function
CREATE OR REPLACE FUNCTION add_to_message() RETURNS TRIGGER AS $$
DECLARE USERS bigint [];
BEGIN IF TG_OP = 'INSERT' THEN RAISE NOTICE 'add_to_message: %',
NEW;
-- select chat with chat_id in NEW
SELECT members INTO USERS
FROM chats
WHERE id = NEW.chat_id;
-- pg_notify(channel, payload) 可以当成简单的消息队列使用
PERFORM pg_notify(
	'chat_message_created',
	json_build_object('message', NEW, 'members', USERS)::text
);
END IF;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
-- define trigger to call add_to_message function
CREATE TRIGGER add_to_message_trigger
AFTER
INSERT ON messages FOR EACH ROW EXECUTE FUNCTION add_to_message();
