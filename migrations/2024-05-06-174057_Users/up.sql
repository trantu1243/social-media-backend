-- Your SQL goes here
CREATE TABLE Users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    name VARCHAR(100) NOT NULL,
    avatar TEXT DEFAULT 'https://trantu-secret.s3.ap-southeast-2.amazonaws.com/0ce956b2-9787-4756-a580-299568810730.png', 
    background TEXT DEFAULT 'https://trantu-secret.s3.ap-southeast-2.amazonaws.com/defaultBackground.png', 
    postId INTEGER[], 
    followerId INTEGER[], 
    followingId INTEGER[], 
    likeId INTEGER[], 
    commentId INTEGER[],
    shareId INTEGER[],
    notifications INTEGER[],
    checkNotification BOOLEAN DEFAULT false
);

ALTER SEQUENCE users_id_seq RESTART WITH 1000000;