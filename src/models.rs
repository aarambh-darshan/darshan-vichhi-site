use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SiteFrontmatter {
    pub name: String,
    pub role: String,
    pub organization: String,
    pub organization_role: String,
    pub seo_title: String,
    pub seo_description: String,
    pub canonical_url: String,
    pub social_image: String,
    pub work_heading: String,
    pub skip_link_label: String,
    pub navigation: Vec<NavigationItem>,
    pub links: SiteLinks,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NavigationItem {
    pub label: String,
    pub target: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContentLink {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteLinks {
    pub github: ContentLink,
    pub organization: ContentLink,
    pub linkedin: ContentLink,
    pub youtube: ContentLink,
    pub medium: ContentLink,
    pub email: ContentLink,
}

impl SiteLinks {
    pub fn all(&self) -> [&ContentLink; 6] {
        [
            &self.github,
            &self.organization,
            &self.linkedin,
            &self.youtube,
            &self.medium,
            &self.email,
        ]
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SectionFrontmatter {
    pub heading: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectFrontmatter {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub order: u32,
    pub featured: bool,
    pub summary: String,
    pub technologies: Vec<String>,
    pub links: ProjectLinks,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectLinks {
    pub github: ContentLink,
    pub documentation: ContentLink,
}

#[derive(Debug, Clone)]
pub struct MarkdownSection {
    pub heading: String,
    pub html: String,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub metadata: ProjectFrontmatter,
    pub html: String,
}

#[derive(Debug, Clone)]
pub struct SiteContent {
    pub site: SiteFrontmatter,
    pub introduction_html: String,
    pub about: MarkdownSection,
    pub writing: MarkdownSection,
    pub contact: MarkdownSection,
    pub projects: Vec<Project>,
}
