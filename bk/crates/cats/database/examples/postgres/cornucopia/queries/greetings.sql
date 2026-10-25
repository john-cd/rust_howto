--! add_greeting
INSERT INTO cornucopia_greetings (message)
VALUES (:message)
ON CONFLICT (message) DO NOTHING;

--! list_greetings
SELECT message
FROM cornucopia_greetings
ORDER BY message;
