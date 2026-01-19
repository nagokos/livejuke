# Justfile
set dotenv-load := true

compose_file := "infra/docker-compose.yml"
compose := "docker compose -f " + compose_file

# コンテナ起動（引数なしで全部、指定でそのサービスだけ）
up *services:
  {{compose}} up -d {{services}}

# コンテナ停止
stop *services:
  {{compose}} stop {{services}}

# コンテナ削除
down:
  {{compose}} down

# データ含めて初期化（ボリューム削除）
reset:
  {{compose}} down -v

# 状態確認
ps:
  {{compose}} ps

# ログ表示
logs *services:
  {{compose}} logs -f {{services}}

# psqlに接続
psql:
  docker exec -it livejuke-postgres psql -U livejuke -d livejuke

# postgresの起動を待つ（CI/スクリプト用）
wait-db:
  until {{compose}} exec -T postgres pg_isready -U livejuke; do sleep 1; done

# 全部リセットして再起動
restart: down up

# DBだけリセット
reset-db:
  {{compose}} rm -sf postgres
  docker volume rm livejuke_postgres-data || true

# DBリセット→再起動
re-db: reset-db
  {{compose}} up -d postgres
  just wait-db
