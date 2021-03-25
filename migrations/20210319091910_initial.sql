-- Add migration script here
SET TIMEZONE='UTC';

CREATE TABLE IF NOT EXISTS public.posts (
    "id" SERIAL PRIMARY KEY,
    "author" VARCHAR(64) NOT NULL,
    "author_id" BIGINT NOT NULL,
    "content" VARCHAR(512) NOT NULL,
    "hearts" BIGINT NOT NULL DEFAULT 0,
    "hearted_users" BIGINT[] NOT NULL DEFAULT '{}'::BIGINT[],
    "timestamp" TIMESTAMP WITH TIME ZONE DEFAULT TIMEZONE('UTC', NOW())
);