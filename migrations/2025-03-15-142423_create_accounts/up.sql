CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    account_type VARCHAR(50) NOT NULL DEFAULT 'credentials',
    password VARCHAR(255),
    provider VARCHAR(50),
    provider_account_id VARCHAR(255),
    refresh_token VARCHAR(255),
    access_token VARCHAR(255),
    expires_at TIMESTAMP WITH TIME ZONE,
    token_type VARCHAR(50),
    scope VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);
