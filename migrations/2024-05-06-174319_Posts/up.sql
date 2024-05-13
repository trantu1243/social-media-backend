-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    userId INTEGER REFERENCES Users(id) ON DELETE CASCADE,
    name VARCHAR(100),
    avatar_user TEXT,
    content TEXT,
    post_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    interact_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    image TEXT,
    likeId INTEGER[], 
    commentId INTEGER[],
    shareId INTEGER[]
);

ALTER SEQUENCE posts_id_seq RESTART WITH 1000000;