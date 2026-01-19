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

# データ含めて全初期化（ボリューム削除）
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

# postgresの起動を待つ
wait-db:
  until {{compose}} exec -T postgres pg_isready -U livejuke; do sleep 1; done

# マイグレーション実行
migrate:
  cd api && sqlx migrate run

# マイグレーションファイル作成
migrate-add name:
  cd api && sqlx migrate add -r {{name}}

migrate-rev:
	cd api && sqlx migrate revert

# DB完全リセット + 起動 + マイグレーション
fresh-db: reset up
  just wait-db
  just migrate

# 全部リセットして再起動
restart: down up

# API開発サーバー起動
dev-api:
  cd api && cargo run --quiet

# フロント開発サーバー起動
dev-app:
  cd app && npx expo start

# 全部起動（DB + API）
dev: up
  just wait-db
  just migrate
  just dev-api
