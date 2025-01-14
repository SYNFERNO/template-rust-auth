-- Down Migration: 20240229103000_create_auth_schema.sql

-- Drop the trigger first
DROP TRIGGER IF EXISTS update_last_used_at_trigger ON sessions;

-- Drop the update function
DROP FUNCTION IF EXISTS update_last_used_at();

-- Drop the indexes
DROP INDEX IF EXISTS idx_sessions_token_id;
DROP INDEX IF EXISTS idx_sessions_user_id;
DROP INDEX IF EXISTS idx_tokens_user_id;
DROP INDEX IF EXISTS idx_tokens_token_value;
DROP INDEX IF EXISTS idx_users_email;

-- Drop tables in reverse order of creation due to foreign key constraints
DROP TABLE IF EXISTS sessions;
DROP TABLE IF EXISTS tokens;
DROP TABLE IF EXISTS users;