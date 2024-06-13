-- Your SQL goes here
CREATE TABLE friend_requests (
    id SERIAL PRIMARY KEY,
    userId1 INTEGER REFERENCES users(id) ON DELETE CASCADE,
    userId2 INTEGER REFERENCES users(id) ON DELETE CASCADE,
    confirm BOOLEAN DEFAULT false
);

ALTER SEQUENCE notifications_id_seq RESTART WITH 1000000;