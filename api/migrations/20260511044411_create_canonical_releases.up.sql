CREATE TABLE canonical_releases (
    release_mbid           UUID NOT NULL,
    canonical_release_mbid UUID NOT NULL,
    release_group_mbid     UUID NOT NULL
);

CREATE INDEX idx_canonical_releases_release_group_mbid 
ON canonical_releases (release_group_mbid);
