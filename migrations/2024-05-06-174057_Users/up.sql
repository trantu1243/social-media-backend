-- Your SQL goes here
CREATE TABLE Users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    name VARCHAR(100) NOT NULL,
    about TEXT,
    avatar TEXT DEFAULT 'https://trantu-secret.s3.ap-southeast-2.amazonaws.com/0ce956b2-9787-4756-a580-299568810730.png', 
    background TEXT DEFAULT 'https://trantu-secret.s3.ap-southeast-2.amazonaws.com/defaultBackground.png', 
    photo TEXT[] DEFAULT ARRAY[]::TEXT[],
    postId INTEGER[] DEFAULT ARRAY[]::INTEGER[], 
    followerId INTEGER[] DEFAULT ARRAY[]::INTEGER[], 
    followingId INTEGER[] DEFAULT ARRAY[]::INTEGER[], 
    likeId INTEGER[] DEFAULT ARRAY[]::INTEGER[], 
    commentId INTEGER[] DEFAULT ARRAY[]::INTEGER[],
    shareId INTEGER[] DEFAULT ARRAY[]::INTEGER[],
    notifications INTEGER[] DEFAULT ARRAY[]::INTEGER[],
    checkNotification BOOLEAN DEFAULT false,
    blockList INTEGER[] DEFAULT ARRAY[]::INTEGER[]
);

ALTER SEQUENCE users_id_seq RESTART WITH 1000000;