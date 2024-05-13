CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    userId INTEGER REFERENCES users(id) ON DELETE CASCADE,
    postId INTEGER REFERENCES posts(id) ON DELETE CASCADE,
    name VARCHAR(100),
    avatar_user TEXT,
    content TEXT,
    likeId INTEGER[], 
    commentId INTEGER[],
    comment_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

ALTER SEQUENCE comments_id_seq RESTART WITH 1000000;