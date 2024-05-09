-- Your SQL goes here
CREATE TABLE Users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL, 
    name VARCHAR(100) NOT NULL,
    avatar VARCHAR(255), 
    background VARCHAR(255), 
    postId INTEGER[], 
    followerId INTEGER[], 
    followingId INTEGER[], 
    likeId INTEGER[], 
    commentId INTEGER[],
    shareId INTEGER[]
);

ALTER SEQUENCE users_id_seq RESTART WITH 1000000;