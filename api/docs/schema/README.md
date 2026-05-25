# livejuke

## Tables

| Name | Columns | Comment | Type |
| ---- | ------- | ------- | ---- |
| [public.users](public.users.md) | 7 |  | BASE TABLE |
| [public.authentications](public.authentications.md) | 6 |  | BASE TABLE |
| [public.sessions](public.sessions.md) | 10 |  | BASE TABLE |
| [public.artists](public.artists.md) | 16 |  | BASE TABLE |
| [public.canonical_releases](public.canonical_releases.md) | 3 |  | BASE TABLE |
| [public.release_groups](public.release_groups.md) | 16 |  | BASE TABLE |
| [public.recordings](public.recordings.md) | 14 |  | BASE TABLE |
| [public.release_group_secondary_types](public.release_group_secondary_types.md) | 5 |  | BASE TABLE |
| [public.release_group_recordings](public.release_group_recordings.md) | 8 |  | BASE TABLE |

## Stored procedures and functions

| Name | ReturnType | Arguments | Type |
| ---- | ------- | ------- | ---- |
| public.update_updated_at | trigger |  | FUNCTION |

## Relations

```mermaid
erDiagram

"public.authentications" }o--|| "public.users" : "FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE"
"public.sessions" }o--|| "public.users" : "FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE"
"public.artists" }o--o| "public.users" : "FOREIGN KEY (approved_by_user_id) REFERENCES users(id)"
"public.artists" }o--o| "public.users" : "FOREIGN KEY (created_by_user_id) REFERENCES users(id)"
"public.artists" }o--o| "public.users" : "FOREIGN KEY (deleted_by_user_id) REFERENCES users(id)"
"public.release_groups" }o--o| "public.users" : "FOREIGN KEY (approved_by_user_id) REFERENCES users(id)"
"public.release_groups" }o--o| "public.users" : "FOREIGN KEY (created_by_user_id) REFERENCES users(id)"
"public.release_groups" }o--o| "public.users" : "FOREIGN KEY (deleted_by_user_id) REFERENCES users(id)"
"public.release_groups" }o--|| "public.artists" : "FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE RESTRICT"
"public.recordings" }o--o| "public.users" : "FOREIGN KEY (approved_by_user_id) REFERENCES users(id)"
"public.recordings" }o--o| "public.users" : "FOREIGN KEY (created_by_user_id) REFERENCES users(id)"
"public.recordings" }o--o| "public.users" : "FOREIGN KEY (deleted_by_user_id) REFERENCES users(id)"
"public.release_group_secondary_types" }o--|| "public.release_groups" : "FOREIGN KEY (release_group_id) REFERENCES release_groups(id) ON DELETE CASCADE"
"public.release_group_recordings" }o--|| "public.release_groups" : "FOREIGN KEY (release_group_id) REFERENCES release_groups(id) ON DELETE CASCADE"
"public.release_group_recordings" }o--|| "public.recordings" : "FOREIGN KEY (recording_id) REFERENCES recordings(id) ON DELETE CASCADE"

"public.users" {
  bigint id
  varchar_20_ display_name
  varchar_254_ email
  varchar_512_ avatar_key
  varchar role
  timestamp_with_time_zone created_at
  timestamp_with_time_zone updated_at
}
"public.authentications" {
  bigint id
  bigint user_id FK
  text provider
  text uid
  timestamp_with_time_zone created_at
  timestamp_with_time_zone updated_at
}
"public.sessions" {
  bigint id
  bigint user_id FK
  text token_hash
  text device_name
  text model_name
  text os
  boolean is_revoked
  timestamp_with_time_zone expires_at
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
"public.canonical_releases" {
  uuid release_mbid
  uuid canonical_release_mbid
  uuid release_group_mbid
}
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
"public.recordings" {
  bigint id
  uuid mbid
  text title
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
```

---

> Generated by [tbls](https://github.com/k1LoW/tbls)
