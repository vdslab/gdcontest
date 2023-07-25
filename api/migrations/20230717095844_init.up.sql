-- Add up migration script here
CREATE TABLE contests (
  contest_name TEXT PRIMARY KEY,
  is_public BOOLEAN NOT NULL
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
);

CREATE TABLE graphs (
  contest_name TEXT REFERENCES contests(contest_name),
  graph_name TEXT,
  content JSON NOT NULL,
  distance JSON NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (contest_name, graph_name)
);

CREATE TABLE submissions (
  id SERIAL PRIMARY KEY,
  contest_name TEXT NOT NULL,
  graph_name TEXT NOT NULL,
  user_id TEXT NOT NULL,
  content JSON NOT NULL,
  metrics JSON NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (contest_name, graph_name) REFERENCES graphs (contest_name, graph_name)
);

CREATE TABLE admin_users (
  user_id TEXT PRIMARY KEY
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
);

