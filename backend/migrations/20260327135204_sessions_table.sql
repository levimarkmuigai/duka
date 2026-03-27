CREATE TABLE sessions (
  id VARCHAR(128) PRIMARY KEY,
  data JSONB NOT NULL,
  expires_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX id_sessions_expires_at ON sessions (expires_at);
