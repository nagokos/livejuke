CREATE TABLE release_group_secondary_types (
  id                BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  release_group_id  BIGINT NOT NULL REFERENCES release_groups(id) ON DELETE CASCADE,
 	secondary_type   TEXT NOT NULL CHECK (
		secondary_type IN (
			'compilation', 
			'soundtrack', 
			'spokenword', 
			'interview', 
			'audiobook', 
			'audio_drama', 
			'live', 
			'remix', 
			'dj_mix', 
			'mixtape_street', 
			'demo', 
			'field_recording'
		)
	),
  created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  UNIQUE (release_group_id, secondary_type)
);

CREATE TRIGGER set_updated_at
    BEFORE UPDATE ON release_group_secondary_types
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
