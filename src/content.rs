use crate::models::{
    ContentLink, MarkdownSection, Project, ProjectFrontmatter, SectionFrontmatter, SiteContent,
    SiteFrontmatter,
};
use gray_matter::{Matter, engine::YAML};
use pulldown_cmark::{Event, Options, Parser, Tag};
use serde::de::DeserializeOwned;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};
use url::Url;

#[derive(Debug)]
pub struct ContentError {
    messages: Vec<String>,
}

impl ContentError {
    fn single(message: impl Into<String>) -> Self {
        Self {
            messages: vec![message.into()],
        }
    }

    fn validation(messages: Vec<String>) -> Self {
        Self { messages }
    }
}

impl fmt::Display for ContentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "content validation failed:")?;
        for message in &self.messages {
            writeln!(formatter, "  - {message}")?;
        }
        Ok(())
    }
}

impl Error for ContentError {}

pub fn load_site(content_dir: &Path) -> Result<SiteContent, ContentError> {
    let site_path = content_dir.join("site.md");
    let about_path = content_dir.join("about.md");
    let writing_path = content_dir.join("writing.md");
    let contact_path = content_dir.join("contact.md");

    let (site, introduction_html) = load_document::<SiteFrontmatter>(&site_path)?;
    let about = load_section(&about_path)?;
    let writing = load_section(&writing_path)?;
    let contact = load_section(&contact_path)?;
    let mut projects = load_projects(&content_dir.join("projects"))?;
    projects.sort_by_key(|project| project.metadata.order);

    let content = SiteContent {
        site,
        introduction_html,
        about,
        writing,
        contact,
        projects,
    };
    validate(&content)?;
    Ok(content)
}

fn load_section(path: &Path) -> Result<MarkdownSection, ContentError> {
    let (metadata, html) = load_document::<SectionFrontmatter>(path)?;
    Ok(MarkdownSection {
        heading: metadata.heading,
        html,
    })
}

fn load_projects(project_dir: &Path) -> Result<Vec<Project>, ContentError> {
    let entries = fs::read_dir(project_dir).map_err(|error| {
        ContentError::single(format!("cannot read {}: {error}", project_dir.display()))
    })?;
    let mut paths = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "md"))
        .collect::<Vec<PathBuf>>();
    paths.sort();

    paths
        .into_iter()
        .map(|path| {
            let (metadata, html) = load_document::<ProjectFrontmatter>(&path)?;
            Ok(Project { metadata, html })
        })
        .collect()
}

fn load_document<T: DeserializeOwned>(path: &Path) -> Result<(T, String), ContentError> {
    let source = fs::read_to_string(path).map_err(|error| {
        ContentError::single(format!("cannot read {}: {error}", path.display()))
    })?;
    reject_placeholders_and_scripts(path, &source)?;

    let parsed = Matter::<YAML>::new().parse::<T>(&source).map_err(|error| {
        ContentError::single(format!(
            "invalid YAML frontmatter in {}: {error}",
            path.display()
        ))
    })?;
    let metadata = parsed.data.ok_or_else(|| {
        ContentError::single(format!("{} must contain YAML frontmatter", path.display()))
    })?;
    let html = markdown_to_html(path, &parsed.content)?;
    Ok((metadata, html))
}

fn reject_placeholders_and_scripts(path: &Path, source: &str) -> Result<(), ContentError> {
    let lowercase = source.to_ascii_lowercase();
    let mut errors = Vec::new();
    if lowercase.contains("todo") {
        errors.push(format!("{} contains a TODO placeholder", path.display()));
    }
    if lowercase.contains("<script") || lowercase.contains("</script") {
        errors.push(format!("{} contains a script tag", path.display()));
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(ContentError::validation(errors))
    }
}

fn markdown_to_html(path: &Path, markdown: &str) -> Result<String, ContentError> {
    if markdown.trim().is_empty() {
        return Err(ContentError::single(format!(
            "{} must contain a Markdown body",
            path.display()
        )));
    }

    let options = Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TABLES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_SMART_PUNCTUATION;
    let events = Parser::new_ext(markdown, options).collect::<Vec<_>>();
    let mut errors = Vec::new();

    for event in &events {
        match event {
            Event::Html(_) | Event::InlineHtml(_) => errors.push(format!(
                "{} contains raw HTML; use Markdown syntax instead",
                path.display()
            )),
            Event::Start(Tag::Heading { .. }) => errors.push(format!(
                "{} contains a Markdown heading; set the section or project name in frontmatter",
                path.display()
            )),
            Event::Start(Tag::Link { dest_url, .. })
            | Event::Start(Tag::Image { dest_url, .. })
                if !is_safe_markdown_url(dest_url) =>
            {
                errors.push(format!(
                    "{} contains an unsafe Markdown URL: {dest_url}",
                    path.display()
                ));
            }
            _ => {}
        }
    }

    if !errors.is_empty() {
        errors.sort();
        errors.dedup();
        return Err(ContentError::validation(errors));
    }

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, events.into_iter());
    let html = html
        .replace(
            "<a href=\"https://",
            "<a rel=\"noopener noreferrer\" href=\"https://",
        )
        .replace(
            "<a href=\"http://",
            "<a rel=\"noopener noreferrer\" href=\"http://",
        );
    Ok(html)
}

fn is_safe_markdown_url(value: &str) -> bool {
    let value = value.trim();
    if value.is_empty()
        || value.starts_with('#')
        || (value.starts_with('/') && !value.starts_with("//"))
        || value.starts_with("./")
        || value.starts_with("../")
    {
        return true;
    }
    Url::parse(value).is_ok_and(|url| matches!(url.scheme(), "http" | "https" | "mailto"))
}

fn validate(content: &SiteContent) -> Result<(), ContentError> {
    let mut errors = Vec::new();
    validate_site(content, &mut errors);
    validate_sections(content, &mut errors);
    validate_projects(&content.projects, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(ContentError::validation(errors))
    }
}

fn validate_site(content: &SiteContent, errors: &mut Vec<String>) {
    let site = &content.site;
    for (field, value) in [
        ("name", site.name.as_str()),
        ("role", site.role.as_str()),
        ("organization", site.organization.as_str()),
        ("organizationRole", site.organization_role.as_str()),
        ("seoTitle", site.seo_title.as_str()),
        ("seoDescription", site.seo_description.as_str()),
        ("socialImage", site.social_image.as_str()),
        ("workHeading", site.work_heading.as_str()),
        ("skipLinkLabel", site.skip_link_label.as_str()),
    ] {
        if value.trim().is_empty() {
            errors.push(format!("content/site.md field `{field}` cannot be empty"));
        }
    }

    if !site.canonical_url.trim().is_empty() && !is_http_url(&site.canonical_url) {
        errors.push("content/site.md `canonicalUrl` must be an HTTP/HTTPS URL".to_owned());
    }
    if !is_safe_asset_url(&site.social_image) {
        errors.push(
            "content/site.md `socialImage` must be a relative path or HTTP/HTTPS URL".to_owned(),
        );
    }
    if site.navigation.is_empty() {
        errors.push("content/site.md must contain at least one navigation item".to_owned());
    }

    let allowed_targets = ["about", "work", "writing", "contact"];
    let mut navigation_targets = HashSet::new();
    for item in &site.navigation {
        if item.label.trim().is_empty() {
            errors.push("content/site.md contains an empty navigation label".to_owned());
        }
        if !allowed_targets.contains(&item.target.as_str()) {
            errors.push(format!(
                "content/site.md navigation target `{}` does not match a page section",
                item.target
            ));
        }
        if !navigation_targets.insert(&item.target) {
            errors.push(format!(
                "content/site.md contains duplicate navigation target `{}`",
                item.target
            ));
        }
    }

    for link in site.links.all() {
        validate_optional_link("content/site.md", link, errors, true);
    }
}

fn validate_sections(content: &SiteContent, errors: &mut Vec<String>) {
    for (path, section) in [
        ("content/about.md", &content.about),
        ("content/writing.md", &content.writing),
        ("content/contact.md", &content.contact),
    ] {
        if section.heading.trim().is_empty() {
            errors.push(format!("{path} heading cannot be empty"));
        }
        if section.html.trim().is_empty() {
            errors.push(format!("{path} Markdown body cannot be empty"));
        }
    }
    if content.introduction_html.trim().is_empty() {
        errors.push("content/site.md introduction cannot be empty".to_owned());
    }
}

fn validate_projects(projects: &[Project], errors: &mut Vec<String>) {
    if projects.is_empty() {
        errors.push("content/projects must contain at least one Markdown file".to_owned());
        return;
    }

    let mut ids = HashSet::new();
    let mut slugs = HashSet::new();
    let mut orders = HashMap::new();
    for project in projects {
        let metadata = &project.metadata;
        let path = format!("content/projects/{}.md", metadata.slug);

        if metadata.name.trim().is_empty() {
            errors.push(format!("{path} project name cannot be empty"));
        }
        if metadata.summary.trim().is_empty() {
            errors.push(format!("{path} project summary cannot be empty"));
        }
        if metadata.id.trim().is_empty() || !is_identifier(&metadata.id) {
            errors.push(format!(
                "{path} has an invalid project ID `{}`",
                metadata.id
            ));
        }
        if metadata.slug.trim().is_empty() || !is_identifier(&metadata.slug) {
            errors.push(format!(
                "{path} has an invalid project slug `{}`",
                metadata.slug
            ));
        }
        if metadata.order == 0 {
            errors.push(format!("{path} project order must be greater than zero"));
        }
        if !ids.insert(&metadata.id) {
            errors.push(format!("duplicate project ID `{}`", metadata.id));
        }
        if !slugs.insert(&metadata.slug) {
            errors.push(format!("duplicate project slug `{}`", metadata.slug));
        }
        if let Some(previous) = orders.insert(metadata.order, metadata.slug.as_str()) {
            errors.push(format!(
                "duplicate project order {} used by `{previous}` and `{}`",
                metadata.order, metadata.slug
            ));
        }
        if metadata.technologies.is_empty() {
            errors.push(format!("{path} must contain at least one technology"));
        }
        if metadata
            .technologies
            .iter()
            .any(|technology| technology.trim().is_empty())
        {
            errors.push(format!("{path} contains an empty technology name"));
        }
        if project.html.trim().is_empty() {
            errors.push(format!("{path} Markdown body cannot be empty"));
        }
        if metadata.links.github.url.trim().is_empty() {
            errors.push(format!("{path} must configure a GitHub link"));
        }
        validate_optional_link(&path, &metadata.links.github, errors, false);
        validate_optional_link(&path, &metadata.links.documentation, errors, false);

        let _is_featured = metadata.featured;
    }
}

fn validate_optional_link(
    path: &str,
    link: &ContentLink,
    errors: &mut Vec<String>,
    allow_mailto: bool,
) {
    if link.url.trim().is_empty() {
        return;
    }
    if link.label.trim().is_empty() {
        errors.push(format!(
            "{path} contains a configured link with an empty label"
        ));
    }
    let valid = if allow_mailto && link.url.starts_with("mailto:") {
        Url::parse(&link.url).is_ok()
    } else {
        is_http_url(&link.url)
    };
    if !valid {
        errors.push(format!(
            "{path} link `{}` must use a valid HTTP/HTTPS URL",
            link.label
        ));
    }
}

fn is_http_url(value: &str) -> bool {
    Url::parse(value)
        .is_ok_and(|url| matches!(url.scheme(), "http" | "https") && url.host_str().is_some())
}

fn is_safe_asset_url(value: &str) -> bool {
    let value = value.trim();
    !value.is_empty()
        && (is_http_url(value)
            || (!value.starts_with('/')
                && !value.contains(':')
                && !value.split('/').any(|part| part == "..")))
}

fn is_identifier(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_and_sorts_repository_content() {
        let content_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("content");
        let content = load_site(&content_dir).expect("repository content should be valid");
        assert_eq!(content.projects.len(), 8);
        assert!(
            content
                .projects
                .windows(2)
                .all(|pair| pair[0].metadata.order < pair[1].metadata.order)
        );
    }

    #[test]
    fn rejects_executable_markdown_links() {
        assert!(!is_safe_markdown_url("javascript:alert(1)"));
        assert!(!is_safe_markdown_url("data:text/html,unsafe"));
        assert!(!is_safe_markdown_url("//example.com/path"));
        assert!(is_safe_markdown_url("https://example.com"));
        assert!(is_safe_markdown_url("#work"));
    }
}
