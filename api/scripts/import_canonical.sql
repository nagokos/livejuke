\copy canonical_releases (release_mbid, canonical_release_mbid, release_group_mbid) FROM 'csv/canonical_release_redirect.csv' WITH (FORMAT csv, HEADER true)
