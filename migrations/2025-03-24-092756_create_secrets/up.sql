CREATE TABLE secrets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    key VARCHAR(255) NOT NULL,
    value TEXT NOT NULL, -- Encrypted value
    nonce BYTEA NOT NULL, -- Nonce used for encryption
    owner_id UUID NOT NULL,
    owner_type VARCHAR(20) NOT NULL CHECK (owner_type IN ('organization', 'team', 'user')),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_secrets_owner ON secrets(owner_id, owner_type);
CREATE INDEX idx_secrets_key ON secrets(key) WHERE deleted_at IS NULL;

SELECT diesel_manage_updated_at('secrets');