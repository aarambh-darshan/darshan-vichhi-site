# Content reference

Every user-facing word and link is managed in `content/**/*.md`. Files use YAML
frontmatter between `---` lines followed by a Markdown body.

Raw HTML and Markdown headings are intentionally rejected. Section and project
headings belong in frontmatter so the generated page always retains one `h1`
and a logical heading hierarchy. Paragraphs, lists, emphasis, code, blockquotes,
tables, and safe links can be used in Markdown bodies.

## Site settings and introduction

Use this shape for `content/site.md`:

```markdown
---
name: Darshan Vichhi
role: Rust Systems & AI Engineer
organization: AarambhDevHub
organizationRole: Founder of

seoTitle: Darshan Vichhi — Rust Systems & AI Engineer
seoDescription: A concise description for search and social previews.
canonicalUrl: https://username.github.io/repository/
socialImage: assets/social-card.svg
workHeading: Selected Work
skipLinkLabel: Skip to content

navigation:
  - label: About
    target: about
  - label: Work
    target: work
  - label: Writing
    target: writing
  - label: Contact
    target: contact

links:
  github:
    label: Darshan Vichhi on GitHub
    url: https://github.com/aarambh-darshan
  organization:
    label: AarambhDevHub on GitHub
    url: https://github.com/AarambhDevHub
  linkedin:
    label: Darshan Vichhi on LinkedIn
    url: https://www.linkedin.com/in/darshan-vichhi-rust-developer
  youtube:
    label: AarambhDevHub on YouTube
    url: https://www.youtube.com/@AarambhDevHub
  medium:
    label: AarambhDevHub on Medium
    url: https://medium.com/@aarambhdevhub
  email:
    label: Email Darshan Vichhi
    url: ""
---

Write the introduction here.
```

Keep `canonicalUrl` empty until the final public URL is known. Any link with an
empty `url` is hidden automatically. Regular configured links must use HTTP or
HTTPS; the email entry may use a valid `mailto:` URL.

Navigation targets are the stable one-page section IDs: `about`, `work`,
`writing`, and `contact`.

## About, writing, and contact sections

`content/about.md`, `content/writing.md`, and `content/contact.md` share this
shape:

```markdown
---
heading: About
---

Write the section body here. Use normal Markdown for paragraphs, lists,
emphasis, and safe links.
```

Change the `heading` to the visible section heading required by that file.

## Project

Create one file per project in `content/projects/`:

```markdown
---
id: example-project
slug: example-project
name: Example Project
order: 9
featured: true

summary: One factual sentence describing the project.

technologies:
  - Rust
  - Systems programming
  - Developer tooling

links:
  github:
    label: View Example Project on GitHub
    url: https://github.com/owner/example-project
  documentation:
    label: Read the Example Project documentation
    url: ""
---

Add a restrained explanation of the project, its purpose, and the architecture
that is visible in its repository.
```

Rules enforced by the validator:

- `id` and `slug` are required, unique, lowercase, and may contain digits or
  hyphens.
- `order` is a unique positive integer.
- `name`, `summary`, Markdown body, and at least one technology are required.
- The GitHub link is required and must use HTTP or HTTPS.
- The documentation link is displayed only when its URL is non-empty.
- Every `.md` file in `content/projects/` is discovered automatically.

To remove a project, delete its file. To reorder projects, edit their `order`
values. Run `cargo run --bin validate_content` before committing.
