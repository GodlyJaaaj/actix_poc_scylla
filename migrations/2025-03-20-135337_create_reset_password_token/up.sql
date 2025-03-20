CREATE TABLE reset_password_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    token VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    used_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX reset_password_tokens_token_idx ON reset_password_tokens (token);
CREATE INDEX reset_password_tokens_user_id_idx ON reset_password_tokens (user_id);