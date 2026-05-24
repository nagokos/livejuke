compose_file := "infra/docker-compose.yml"
compose := "docker compose -f " + compose_file

up *services:
  {{compose}} up -d {{services}}

stop *services:
  {{compose}} stop {{services}}

down:
  {{compose}} down

reset:
  {{compose}} down -v

ps:
  {{compose}} ps

logs *services:
  {{compose}} logs -f {{services}}

psql:
  docker exec -it livejuke-postgres psql -U livejuke -d livejuke

redis:
  docker exec -it redis redis-cli

wait-db:
  until {{compose}} exec -T postgres pg_isready -U livejuke; do sleep 1; done

fresh-db: reset up
  just wait-db
  just api/migrate

restart: down up

dev: up
  just wait-db
  just api/migrate
  just api/dev

gen-schema:
  just api/gen-schema
  just app/gen-schema
