CREATE TABLE recordings(
	id                  BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	mbid                UUID UNIQUE,
	title               TEXT NOT NULL,
	first_release_date  DATE,
	source              TEXT NOT NULL CHECK (source IN ('musicbrainz', 'manual')),
	approved_at         TIMESTAMPTZ,
	approved_by_user_id BIGINT REFERENCES users(id) ON DELETE RESTRICT,
	created_by_user_id  BIGINT REFERENCES users(id) ON DELETE RESTRICT,
	created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	deleted_at          TIMESTAMPTZ,
	deleted_by_user_id  BIGINT REFERENCES users(id) ON DELETE RESTRICT,
	deletion_reason     TEXT,

	CONSTRAINT mbid_source_consistency CHECK (
			(source = 'musicbrainz' AND mbid IS NOT NULL) OR
			(source = 'manual'      AND mbid IS NULL)
	),
	CONSTRAINT approval_consistency CHECK (
			(approved_at IS NULL 
					AND approved_by_user_id IS NULL) OR
			(source = 'manual' 
					AND approved_at IS NOT NULL 
					AND approved_by_user_id IS NOT NULL) OR
			(source = 'musicbrainz' 
					AND approved_at IS NOT NULL 
					AND approved_by_user_id IS NULL)
	),
	CONSTRAINT creator_consistency CHECK (
    (source = 'musicbrainz' AND created_by_user_id IS NULL) OR
    (source = 'manual'      AND created_by_user_id IS NOT NULL)
  ),
	CONSTRAINT deletion_consistency CHECK (
			(deleted_at IS NULL     
					AND deleted_by_user_id IS NULL     
					AND deletion_reason IS NULL) OR
			(deleted_at IS NOT NULL 
					AND deleted_by_user_id IS NOT NULL 
					AND deletion_reason IS NOT NULL)
	)
);

CREATE INDEX idx_recordings_approved  ON recordings(approved_at)  WHERE deleted_at IS NULL;
CREATE INDEX idx_recordings_title     ON recordings(title)        WHERE deleted_at IS NULL;


CREATE TRIGGER set_updated_at
    BEFORE UPDATE ON recordings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
