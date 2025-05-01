.PHONY: all backend frontend docker-up docker-down db-init run
all: backend frontend
prepare:
	cd backend && DATABASE_URL=postgres://postgres:docker@localhost:5432/actix-api-db cargo sqlx prepare
backend:
	cd backend && export DATABASE_URL=postgres://postgres:docker@localhost:5432/actix-api-db && cargo build && cargo run
frontend:
	cd frontend && trunk serve --open --port 3000
run:
	zsh -c 'cd backend && DATABASE_URL=postgres://postgres:docker@localhost:5432/actix-api-db cargo run' & \
	zsh -c 'cd frontend && trunk serve --open --port 3000' & \
	wait
docker-up:
	docker compose -f docker-compose.yml up --build
docker-down:
	docker compose -f docker-compose.yml down
db-init:
	psql -h localhost -U postgres -d actix-api-db -f backend/schema.sql && \
	echo "Database initialized successfully."
db-clean:
	psql -h localhost -U postgres -d actix-api-db -c "TRUNCATE users, jobs RESTART IDENTITY CASCADE;"
