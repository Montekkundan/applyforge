.PHONY: db-users db-jobs

# View all users in the local Postgres database
db-users:
	psql -h localhost -U postgres -d actix-api-db -c "SELECT * FROM users;"

# View all jobs in the local Postgres database
db-jobs:
	psql -h localhost -U postgres -d actix-api-db -c "SELECT * FROM jobs;"
.PHONY: frontend backend db-init db-clean

# Start the Yew frontend locally
frontend:
	cd frontend && trunk serve --open --port 3000

# Start the Actix backend locally (requires DATABASE_URL in backend/.env or set in shell)
backend:
	cd backend && cargo run

# Initialize the local Postgres database using schema.sql
db-init:
	psql -h localhost -U postgres -d actix-api-db -f backend/schema.sql

# Clean (truncate) the local database tables
db-clean:
	psql -h localhost -U postgres -d actix-api-db -c "TRUNCATE users, jobs RESTART IDENTITY CASCADE;"