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