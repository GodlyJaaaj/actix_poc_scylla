CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_accounts_user_id ON accounts(user_id);
CREATE INDEX idx_accounts_provider_id ON accounts(provider, provider_account_id);
CREATE INDEX idx_teams_organization_id ON teams(organization_id);
CREATE INDEX idx_repositories_organization_id ON repositories(organization_id);