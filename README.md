# Darshan Vichhi — personal website

A small, Markdown-managed personal website for Darshan Vichhi, Rust Systems &
AI Engineer and founder of AarambhDevHub. A native Rust binary reads the content
and uses Leptos server-side rendering to produce a plain static GitHub Pages
artifact.

The page itself is plain HTML, CSS, SVG, XML, and text: no hydration code,
client-side state, server, database, or Node.js toolchain. The one exception is
the optional **intro video** (see below): a corner play button whose 20-line
loader script does nothing until the visitor asks for it.

## Intro video

A "Watch the intro" button sits in the bottom-right corner. Pressing it makes
the button press, spring, and grow into a full-screen, interactive,
audio-driven video drawn live on a canvas: typography, a Scenix 3D wireframe,
and all motion from Animato. It is part of this crate: the code is in
`src/video/`, and the same crate built for `wasm32` is the video (the site
generator is left out of that build).

- Nothing downloads until the visitor hovers, focuses, or presses the button.
- The narration is loaded whole and played from memory, so seeking works on any
  server, including ones without range-request support.
- The picture is a pure function of the narration's clock, so pause, scrub,
  replay, and keyboard control (`Space`, arrows, `M`, `C`, `1`-`6`, `Esc`) are exact.
- Words are timed from `src/video/data/words.json`, built by
  `scripts/build_words.py` from `src/video/data/script.md` and the ElevenLabs
  word timings. `cargo test` fails if a spoken cue goes missing.
- CI (`.github/workflows/pages.yml`) builds the WebAssembly module with
  `scripts/build-video.sh` before building and deploying the site. The built
  files in `assets/video/` are committed too, so a plain `cargo run --bin
  build_site` also works locally; rebuild them with `scripts/build-video.sh`
  (needs the `wasm32-unknown-unknown` target and a matching `wasm-bindgen`
  CLI; the script prints the install command).
- Without JavaScript or WebAssembly the button simply stays hidden.

## Technology

- Rust stable, edition 2024
- Leptos 0.8.20 with its `ssr` feature
- pulldown-cmark for Markdown
- Serde and gray_matter with yaml-rust2 for typed YAML frontmatter
- Handwritten CSS
- GitHub Actions and GitHub Pages

Leptos runs only inside the native build binary. Browsers receive the generated
files in `dist/`; they do not run Rust or Leptos.

## Repository structure

```text
.
├── .github/workflows/pages.yml
├── assets/
│   ├── favicon.svg
│   ├── social-card.svg
│   └── styles.css
├── content/
│   ├── projects/*.md
│   ├── about.md
│   ├── contact.md
│   ├── site.md
│   └── writing.md
├── src/
│   ├── bin/
│   │   ├── build_site.rs
│   │   └── validate_content.rs
│   ├── content.rs
│   ├── lib.rs
│   ├── models.rs
│   └── render.rs
├── CONTENT.md
├── Cargo.toml
└── rust-toolchain.toml
```

`dist/` is generated and gitignored. The Pages workflow uploads only this
directory, with `index.html` at the artifact root.

## Build locally

Install stable Rust with rustup, then run:

```sh
cargo run --bin validate_content
cargo run --release --bin build_site
```

The second command recreates `dist/` and writes `dist/index.html`, copied
assets, `sitemap.xml`, `robots.txt`, and `.nojekyll`.

Preview the generated files with a simple local file server:

```sh
python3 -m http.server 8000 --directory dist
```

Then open `http://localhost:8000`. Python is used only for local preview; it is
not part of the build or deployed website.

## Quality checks

The same checks used by continuous integration can be run locally:

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
cargo run --bin validate_content
cargo run --release --bin build_site
```

Validation rejects malformed frontmatter, missing required values, unsafe
Markdown links, raw HTML, script tags, placeholder markers, invalid configured
URLs, duplicate project IDs, slugs, or orders, and invalid order values. Error
messages include the affected content file.

## Edit the website

All visible website copy, headings, navigation labels, link labels and URLs,
project technology names, and SEO copy live under `content/`. Normal content
work never requires changes to Rust, HTML, or CSS.

The standard update workflow is:

1. Edit a Markdown file inside `content/`.
2. Commit the Markdown change.
3. Push it to `main`.
4. GitHub Actions validates the content.
5. Rust rebuilds the static site.
6. GitHub Pages deploys the new `dist/` artifact.

See [CONTENT.md](CONTENT.md) for copyable schemas.

### Add a project

Copy the project example from `CONTENT.md` into a new file under
`content/projects/`. Give it a unique lowercase `id`, unique lowercase `slug`,
and unique positive `order`. The loader scans every `.md` file in that
directory, so no Rust import or project registry is needed.

### Remove a project

Delete its Markdown file from `content/projects/` and rebuild. No source-code
change is required. At least one project must remain.

### Change project order

Edit the numeric `order` values in the project frontmatter. Values must be
positive and unique. The build sorts every discovered project by this field.

## Canonical URL and SEO

`content/site.md` intentionally starts with an empty `canonicalUrl` so the
repository does not publish a guessed address. An empty value produces a clear
build warning, omits the canonical and Open Graph URL tags, omits the sitemap
location from `robots.txt`, and leaves the sitemap URL set empty.

After GitHub Pages has a final address, set the field to the public site root,
including the repository path and a trailing slash when applicable:

```yaml
canonicalUrl: https://username.github.io/repository/
```

Rebuild after editing it. The generator will emit canonical metadata, absolute
social-image metadata, a sitemap location, and the `robots.txt` sitemap line.

## Deploy with GitHub Pages

1. Push the repository to GitHub with the default branch named `main`.
2. In the repository settings, open **Pages** and choose **GitHub Actions** as
   the source.
3. Set `canonicalUrl` in `content/site.md` to the final Pages URL and push the
   change.
4. Run the **Deploy GitHub Pages** workflow, or let a push to `main` trigger it.

The workflow installs stable Rust, restores the Cargo cache, validates content,
runs tests and Clippy with warnings denied, builds the release artifact, checks
for `dist/index.html` and `dist/.nojekyll`, uploads `dist/`, and deploys it with
GitHub Pages. Deployment concurrency prevents two Pages releases from running
at the same time.

## License

The repository source is available under the [MIT License](LICENSE).
