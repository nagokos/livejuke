# public.release_groups

## Columns

| Name | Type | Default | Nullable | Children | Parents | Comment |
| ---- | ---- | ------- | -------- | -------- | ------- | ------- |
| id | bigint |  | false | [public.release_group_secondary_types](public.release_group_secondary_types.md) [public.release_group_recordings](public.release_group_recordings.md) |  |  |
| artist_id | bigint |  | false |  | [public.artists](public.artists.md) |  |
| mbid | uuid |  | true |  |  |  |
| title | text |  | false |  |  |  |
| primary_type | text |  | false |  |  |  |
| first_release_date | date |  | true |  |  |  |
| status | text | 'pending'::text | false |  |  |  |
| source | text |  | false |  |  |  |
| approved_at | timestamp with time zone |  | true |  |  |  |
| approved_by_user_id | bigint |  | true |  | [public.users](public.users.md) |  |
| created_by_user_id | bigint |  | true |  | [public.users](public.users.md) |  |
| created_at | timestamp with time zone | now() | false |  |  |  |
| updated_at | timestamp with time zone | now() | false |  |  |  |
| deleted_at | timestamp with time zone |  | true |  |  |  |
| deleted_by_user_id | bigint |  | true |  | [public.users](public.users.md) |  |
| deletion_reason | text |  | true |  |  |  |

## Constraints

| Name | Type | Definition |
| ---- | ---- | ---------- |
| deleted_consistency | CHECK | CHECK ((((deleted_at IS NULL) AND (deleted_by_user_id IS NULL)) OR ((deleted_at IS NOT NULL) AND (deleted_by_user_id IS NOT NULL)))) |
| mbid_source_consistency | CHECK | CHECK ((((source = 'musicbrainz'::text) AND (mbid IS NOT NULL)) OR ((source = 'manual'::text) AND (mbid IS NULL)))) |
| release_groups_primary_type_check | CHECK | CHECK ((primary_type = ANY (ARRAY['album'::text, 'single'::text, 'ep'::text]))) |
| release_groups_source_check | CHECK | CHECK ((source = ANY (ARRAY['musicbrainz'::text, 'manual'::text]))) |
| release_groups_status_check | CHECK | CHECK ((status = ANY (ARRAY['pending'::text, 'approved'::text]))) |
| status_approved_consistency | CHECK | CHECK ((((status = 'pending'::text) AND (approved_at IS NULL) AND (approved_by_user_id IS NULL)) OR ((status = 'approved'::text) AND (approved_at IS NOT NULL) AND (approved_by_user_id IS NOT NULL)))) |
| release_groups_approved_by_user_id_fkey | FOREIGN KEY | FOREIGN KEY (approved_by_user_id) REFERENCES users(id) |
| release_groups_created_by_user_id_fkey | FOREIGN KEY | FOREIGN KEY (created_by_user_id) REFERENCES users(id) |
| release_groups_deleted_by_user_id_fkey | FOREIGN KEY | FOREIGN KEY (deleted_by_user_id) REFERENCES users(id) |
| release_groups_artist_id_fkey | FOREIGN KEY | FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE RESTRICT |
| release_groups_pkey | PRIMARY KEY | PRIMARY KEY (id) |
| release_groups_mbid_key | UNIQUE | UNIQUE (mbid) |

## Indexes

| Name | Definition |
| ---- | ---------- |
| release_groups_pkey | CREATE UNIQUE INDEX release_groups_pkey ON public.release_groups USING btree (id) |
| release_groups_mbid_key | CREATE UNIQUE INDEX release_groups_mbid_key ON public.release_groups USING btree (mbid) |

## Triggers

| Name | Definition |
| ---- | ---------- |
| set_updated_at | CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.release_groups FOR EACH ROW EXECUTE FUNCTION update_updated_at() |

## Relations

```mermaid
erDiagram

"public.release_group_secondary_types" }o--|| "public.release_groups" : "FOREIGN KEY (release_group_id) REFERENCES release_groups(id) ON DELETE CASCADE"
"public.release_group_recordings" }o--|| "public.release_groups" : "FOREIGN KEY (release_group_id) REFERENCES release_groups(id) ON DELETE CASCADE"
"public.release_groups" }o--|| "public.artists" : "FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE RESTRICT"
"public.release_groups" }o--o| "public.users" : "FOREIGN KEY (approved_by_user_id) REFERENCES users(id)"
"public.release_groups" }o--o| "public.users" : "FOREIGN KEY (created_by_user_id) REFERENCES users(id)"
"public.release_groups" }o--o| "public.users" : "FOREIGN KEY (deleted_by_user_id) REFERENCES users(id)"

"public.release_groups" {
  bigint id
  bigint artist_id FK
  uuid mbid
  text title
  text primary_type
  date first_release_date
  text status
  text source
  timestamp_with_time_zone approved_at
  bigint approved_by_user_id FK
  bigint created_by_user_id FK
  timestamp_with_time_zone created_at
  timestamp_with_time_zone updated_at
  timestamp_with_time_zone deleted_at
  bigint deleted_by_user_id FK
  text deletion_reason
}
"public.release_group_secondary_types" {
  bigint id
  bigint release_group_id FK
  text secondary_type
  timestamp_with_time_zone created_at
  timestamp_with_time_zone updated_at
}
"public.release_group_recordings" {
  bigint id
  bigint release_group_id FK
  bigint recording_id FK
  uuid track_mbid
  smallint disc_number
  smallint track_number
  timestamp_with_time_zone created_at
  timestamp_with_time_zone updated_at
}
"public.artists" {
  bigint id
  uuid mbid
  text title
  date first_release_date
  boolean tracks_fetched
  text spotify_id
  text status
  text source
  timestamp_with_time_zone approved_at
  bigint approved_by_user_id FK
  bigint created_by_user_id FK
  timestamp_with_time_zone created_at
  timestamp_with_time_zone updated_at
  timestamp_with_time_zone deleted_at
  bigint deleted_by_user_id FK
  text deletion_reason
}
"public.users" {
  bigint id
  varchar_20_ display_name
  varchar_254_ email
  varchar_512_ avatar_key
  varchar role
  timestamp_with_time_zone created_at
  timestamp_with_time_zone updated_at
}
```

---

> Generated by [tbls](https://github.com/k1LoW/tbls)
