# Darshan Vichhi — personal website

A small, Markdown-managed personal website for Darshan Vichhi, Rust Systems &
AI Engineer and founder of AarambhDevHub. A native Rust binary reads the content
and uses Leptos server-side rendering to produce a plain static GitHub Pages
artifact.

The deployed site contains HTML, CSS, SVG, XML, and text files only. It ships no
JavaScript, WebAssembly, hydration code, client-side state, server, database, or
Node.js toolchain.

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
