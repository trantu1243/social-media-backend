-- Your SQL goes here
CREATE TABLE notifications (
    id SERIAL PRIMARY KEY,
    userId INTEGER REFERENCES users(id) ON DELETE CASCADE,
    postId INTEGER REFERENCES posts(id) ON DELETE CASCADE,
    name VARCHAR(100),
    content TEXT,
    avatar_image_url VARCHAR(255),
    notification_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

ALTER SEQUENCE notifications_id_seq RESTART WITH 1000000;