-- Your SQL goes here
CREATE TABLE sessions (
    session_id VARCHAR PRIMARY KEY,
    user_id SERIAL references users(id),
    expire TIMESTAMP NOT NULL
)

