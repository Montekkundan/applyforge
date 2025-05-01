# Yew Trunk Template

This is a fairly minimal template for a Yew app that's built with [Trunk].

## Usage

For a more thorough explanation of Trunk and its features, please head over to the [repository][trunk].

### Installation

If you don't already have it installed, it's time to install Rust: <https://www.rust-lang.org/tools/install>.
The rest of this guide assumes a typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Now that we have our basics covered, it's time to install the star of the show: [Trunk].
Simply run the following command to install it:

```bash
cargo install trunk wasm-bindgen-cli
```

That's it, we're done!

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

## Using this template

There are a few things you have to adjust when adopting this template.

### Remove example code

The code in [src/main.rs](src/main.rs) specific to the example is limited to only the `view` method.
There is, however, a fair bit of Sass in [index.scss](index.scss) you can remove.

### Update metadata

Update the `version`, `description` and `repository` fields in the [Cargo.toml](Cargo.toml) file.
The [index.html](index.html) file also contains a `<title>` tag that needs updating.


Finally, you should update this very `README` file to be about your app.

### License

The template ships with both the Apache and MIT license.
If you don't want to have your app dual licensed, just remove one (or both) of the files and update the `license` field in `Cargo.toml`.

There are two empty spaces in the MIT license you need to fill out: `` and `Montek <87750128+Montekkundan@users.noreply.github.com>`.

[trunk]: https://github.com/thedodd/trunk

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

---