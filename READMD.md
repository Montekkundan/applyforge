## Steps:


### Developer Log: Project Setup & Troubleshooting

1. **Rust & WASM Toolchain Issues**
   - Rust was originally installed via Homebrew, which can conflict with rustup-managed toolchains, especially for cross-compilation targets like `wasm32-unknown-unknown`.
   - I encountered errors like `can't find crate for core` and `can't find crate for std` when building for WASM.
   - **Fix:** Uninstalled Homebrew Rust, reinstalled Rust using rustup, and ran `rustup target add wasm32-unknown-unknown` to ensure the WASM target was available for the correct toolchain.

2. **Yew App Structure**
   - Created a `models.rs` for the job application data model.
   - Updated `app.rs` to use a stateful Yew function component for the main UI.
   - Added a form for users to add job applications (company, position, status, date).
   - Displayed analytics (counts for each status) and a table of all jobs.
   - Added buttons for CSV export and OAuth login (UI only, logic to be implemented).

3. **Styling**
   - Replaced the default Yew example styles with a custom dark theme using oklch colors.
   - Styled the form, table, and buttons for a modern, accessible look.
   - Attempted to use Tailwind CSS, but reverted to custom SCSS due to Trunk/Sass limitations with `@import "tailwindcss";`.

4. **Dependency Fixes**
   - Added `web-sys` to `Cargo.toml` for DOM event handling in Yew forms.

5. **Build/Cache Issues**
   - Used `trunk clean` and a hard browser refresh (`Cmd+Shift+R`) to resolve issues with old builds or cached WASM.

6. Added Actix Web api

7. **Database Integration & Secure Auth**
   - Switched backend from in-memory storage to PostgreSQL using `sqlx` for async database access.
   - Added a `backend/.env` file to store the `DATABASE_URL` so all tools and the app can find the database connection string automatically.
   - Updated the backend to use a connection pool (`PgPool`) and removed the old in-memory state.
   - Implemented secure password hashing with Argon2 (never store plain text passwords!). On registration, passwords are hashed before saving; on login, hashes are verified.
   - Added SQL table creation logic to `main.rs` and a `backend/schema.sql` for manual or scripted DB setup.

8. **Docker & Makefile Automation**
   - Fixed Docker Compose and Dockerfile issues so the backend and Postgres DB work together reliably.
   - Created a `Makefile` with targets for common tasks:
     - `make prepare` — prepares SQLx macros for offline builds.
     - `make backend` — builds and runs the backend.
     - `make frontend` — runs the Yew frontend.
     - `make run` — runs both backend and frontend together.
     - `make docker-up` / `make docker-down` — starts/stops all services.
     - `make db-init` — initializes the database schema.
     - `make db-clean` — truncates all users and jobs for a clean test DB.
   - Explained that `docker compose down -v` will delete all database data, while `make docker-down` only stops containers.

9. **Frontend & API Integration**
   - Added a `frontend/src/login_register.rs` component for user registration and login, posting to the Actix Web API.
   - Updated the frontend to use the new API endpoints for authentication and job management.
   - Ensured error messages from the backend are displayed in the UI for better user feedback.

10. **General Advice & Troubleshooting**
    - If you see errors like `set DATABASE_URL to use query macros online, or run cargo sqlx prepare`, make sure your `.env` file is present and up to date, or run `make prepare`.
    - Always use TABs (not spaces) for command indentation in your `Makefile` to avoid "missing separator" errors.
    - Use `make db-clean` to reset your database for fresh testing, or `docker compose down -v` to wipe all data.

---

**System:**
- PostgreSQL
- Server: postgres
- Username: postgres
- Password: docker
- Database: actix-api-db

**Common Commands:**
- Start containers: `make docker-up`
- Stop containers: `make docker-down`
- Wipe all DB data: `docker compose down -v`
- Clean (truncate) users and jobs: `make db-clean`
- Re-initialize tables: `make db-init`
- Prepare SQLx macros: `make prepare`
- Run backend: `make backend`
- Run frontend: `make frontend`
- Run both: `make run`