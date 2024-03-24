-- Users
CREATE TABLE users (
  id UUID PRIMARY KEY,
  username TEXT NOT NULL,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL
);

CREATE TABLE conversations (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE messages (
  id UUID PRIMARY KEY,
  conversation_id UUID NOT NULL,
  user_id UUID NOT NULL,
  text TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);
