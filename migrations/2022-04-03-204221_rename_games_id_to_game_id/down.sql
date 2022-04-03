-- This file should undo anything in `up.sql`
ALTER TABLE users RENAME COLUMN game_id TO games_id;