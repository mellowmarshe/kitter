-- Add migration script here
SET TIMEZONE='UTC';

CREATE TABLE IF NOT EXISTS public.posts (
    "id" SERIAL PRIMARY KEY,
    "author" VARCHAR(64) NOT NULL,
    "author_id" BIGINT NOT NULL,
    "content" VARCHAR(512) NOT NULL,
    "timestamp" TIMESTAMP WITH TIME ZONE DEFAULT TIMEZONE('UTC', NOW())
);