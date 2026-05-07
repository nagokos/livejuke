CREATE TABLE release_group_recordings (
  id                BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  release_group_id  BIGINT NOT NULL REFERENCES release_groups(id) ON DELETE CASCADE,
  recording_id      BIGINT NOT NULL REFERENCES recordings(id) ON DELETE CASCADE,
  track_mbid        UUID UNIQUE,
  disc_number       SMALLINT NOT NULL CHECK (disc_number > 0),
  track_number      SMALLINT NOT NULL CHECK (track_number > 0),
  created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE (release_group_id, recording_id),
  UNIQUE (release_group_id, disc_number, track_number)
);

CREATE TRIGGER set_updated_at
    BEFORE UPDATE ON release_group_recordings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
