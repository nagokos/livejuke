CREATE TABLE authentications(
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	provider VARCHAR NOT NULL CHECK (provider IN ('email', 'google')), 
	uid VARCHAR NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	UNIQUE(provider, uid)
);
CREATE TRIGGER set_updated_at
    BEFORE UPDATE ON authentications
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
