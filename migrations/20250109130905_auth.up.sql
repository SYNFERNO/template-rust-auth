-- Up Migration: 20240229103000_create_auth_schema.sql

-- 1. Users Table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE,
    CONSTRAINT username_length CHECK (LENGTH(username) >= 3),
    CONSTRAINT email_format CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$')
);

-- Index for email lookup
CREATE INDEX idx_users_email ON users (email);

-- 2. Tokens Table
CREATE TABLE tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_value TEXT UNIQUE NOT NULL,
    token_type VARCHAR(50) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT token_value_length CHECK (LENGTH(token_value) > 0),
    CONSTRAINT token_type_supported CHECK (token_type IN ('access_token', 'refresh_token'))
);

-- Index for token lookup
CREATE INDEX idx_tokens_token_value ON tokens (token_value);
CREATE INDEX idx_tokens_user_id ON tokens (user_id);


-- 3. Sessions Table
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_id UUID NOT NULL REFERENCES tokens(id) ON DELETE CASCADE,
    device_info TEXT,
    ip_address INET,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_used_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Index on user_id for session lookup
CREATE INDEX idx_sessions_user_id ON sessions (user_id);
CREATE INDEX idx_sessions_token_id ON sessions (token_id);

-- Trigger function to update last_used_at on session update
CREATE OR REPLACE FUNCTION update_last_used_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.last_used_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to update last_used_at on sessions table
CREATE TRIGGER update_last_used_at_trigger
BEFORE UPDATE ON sessions
FOR EACH ROW
EXECUTE FUNCTION update_last_used_at();