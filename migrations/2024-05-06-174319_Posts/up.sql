-- Your SQL goes here
CREATE TABLE Posts (
    id INTEGER PRIMARY KEY,
    userId INTEGER REFERENCES Users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    imageUrl VARCHAR(255),
    likeId INTEGER[],
    commentId INTEGER[],
    shareId INTEGER[]
);

ALTER SEQUENCE users_id_seq RESTART WITH 1000000;