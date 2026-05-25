CREATE TABLE recordings(
	id                  BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	mbid                UUID UNIQUE NULL,
	title               TEXT NOT NULL,
	first_release_date  DATE NULL,
	status              TEXT NOT NULL CHECK (status IN ('pending', 'approved')) DEFAULT 'pending',
	source              TEXT NOT NULL CHECK (source IN ('musicbrainz', 'manual')),
	approved_at         TIMESTAMPTZ,
	approved_by_user_id BIGINT REFERENCES users(id),
	created_by_user_id  BIGINT REFERENCES users(id),
  created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	deleted_at          TIMESTAMPTZ,
	deleted_by_user_id  BIGINT REFERENCES users(id),
	deletion_reason     TEXT,

	CONSTRAINT mbid_source_consistency CHECK (
		(source = 'musicbrainz' AND mbid IS NOT NULL) OR
		(source = 'manual'			AND mbid IS NULL)
	),
	CONSTRAINT status_approved_consistency CHECK (
		(status = 'pending'  AND approved_at IS NULL     AND approved_by_user_id IS NULL) OR
		(status = 'approved' AND approved_at IS NOT NULL AND approved_by_user_id IS NOT NULL)
	),
	CONSTRAINT deleted_consistency CHECK (
		(deleted_at IS NULL     AND deleted_by_user_id IS NULL) OR
		(deleted_at IS NOT NULL AND deleted_by_user_id IS NOT NULL)
	)
);

CREATE TRIGGER set_updated_at
    BEFORE UPDATE ON recordings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
