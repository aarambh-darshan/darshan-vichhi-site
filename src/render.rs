use crate::models::{ContentLink, SiteContent};
use leptos::either::Either;
use leptos::prelude::*;
use serde_json::json;
use url::Url;

pub fn render_document(content: &SiteContent, year: i32) -> String {
    let site = &content.site;
    let canonical = site.canonical_url.trim().to_owned();
    let social_image = social_image_url(&canonical, &site.social_image);
    let person_json = json_ld(&person_schema(content));
    let website_json = json_ld(&website_schema(content));

    let canonical_link = (!canonical.is_empty()).then(|| {
        view! { <link rel="canonical" href=canonical.clone()/> }
    });
    let open_graph_url = (!canonical.is_empty()).then(|| {
        leptos::html::meta()
            .attr("property", "og:url")
            .content(canonical.clone())
    });
    let navigation = site.navigation.iter().map(|item| {
        view! { <a href=format!("#{}", item.target)>{item.label.as_str()}</a> }
    });
    let contact_links = site
        .links
        .all()
        .into_iter()
        .filter(|link| !link.url.trim().is_empty())
        .map(|link| {
            view! {
                <li>
                    <a href=link.url.as_str() rel="noopener noreferrer">{link.label.as_str()}</a>
                </li>
            }
        });
    let project_views = content.projects.iter().map(|project| {
        let metadata = &project.metadata;
        let technologies = metadata.technologies.iter().map(|technology| {
            view! { <li>{technology.as_str()}</li> }
        });
        let project_links = [&metadata.links.github, &metadata.links.documentation]
            .into_iter()
            .filter(|link| !link.url.trim().is_empty())
            .map(|link| {
                view! {
                    <li>
                        <a href=link.url.as_str() rel="noopener noreferrer">
                            {link.label.as_str()}
                        </a>
                    </li>
                }
            });

        view! {
            <article class="project" id=metadata.slug.as_str() data-featured=metadata.featured>
                <header class="project-header">
                    <p class="project-number">{format!("{:02}", metadata.order)}</p>
                    <h3>{metadata.name.as_str()}</h3>
                </header>
                <p class="project-summary">{metadata.summary.as_str()}</p>
                <div class="markdown project-body" inner_html=project.html.as_str()></div>
                <ul class="technology-list" aria-label="Technologies">{technologies.collect_view()}</ul>
                <ul class="project-links">{project_links.collect_view()}</ul>
            </article>
        }
    });

    let organization = organization_view(content);
    let footer_github = optional_footer_link(&site.links.github);
    let footer_linkedin = optional_footer_link(&site.links.linkedin);

    let page = view! {
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>{site.seo_title.as_str()}</title>
                <meta name="description" content=site.seo_description.as_str()/>
                <meta name="robots" content="index, follow"/>
                {canonical_link}
                {leptos::html::meta().attr("property", "og:type").content("website")}
                {leptos::html::meta().attr("property", "og:title").content(site.seo_title.as_str())}
                {leptos::html::meta().attr("property", "og:description").content(site.seo_description.as_str())}
                {open_graph_url}
                {leptos::html::meta().attr("property", "og:image").content(social_image.clone())}
                <meta name="twitter:card" content="summary_large_image"/>
                <meta name="twitter:title" content=site.seo_title.as_str()/>
                <meta name="twitter:description" content=site.seo_description.as_str()/>
                <meta name="twitter:image" content=social_image/>
                <meta name="theme-color" content="#1b1b1a"/>
                <link rel="icon" href="assets/favicon.svg" type="image/svg+xml"/>
                <link rel="stylesheet" href="assets/styles.css"/>
                <script type="application/ld+json" inner_html=person_json></script>
                <script type="application/ld+json" inner_html=website_json></script>
            </head>
            <body id="top">
                <a class="skip-link" href="#content">{site.skip_link_label.as_str()}</a>
                <div class="site-shell">
                    <header class="site-header">
                        <a class="site-name" href="#top">{site.name.as_str()}</a>
                        <nav aria-label="Primary navigation">{navigation.collect_view()}</nav>
                    </header>

                    <main id="content">
                        <section class="introduction" aria-labelledby="page-title">
                            <p class="eyebrow">{site.role.as_str()}</p>
                            <h1 id="page-title">{site.name.as_str()}</h1>
                            <p class="organization">{site.organization_role.as_str()} " " {organization}</p>
                            <div class="markdown introduction-copy" inner_html=content.introduction_html.as_str()></div>
                        </section>

                        <section id="about" aria-labelledby="about-heading">
                            <h2 id="about-heading">{content.about.heading.as_str()}</h2>
                            <div class="markdown" inner_html=content.about.html.as_str()></div>
                        </section>

                        <section id="work" aria-labelledby="work-heading">
                            <h2 id="work-heading">{site.work_heading.as_str()}</h2>
                            <div class="project-list">{project_views.collect_view()}</div>
                        </section>

                        <section id="writing" aria-labelledby="writing-heading">
                            <h2 id="writing-heading">{content.writing.heading.as_str()}</h2>
                            <div class="markdown" inner_html=content.writing.html.as_str()></div>
                        </section>

                        <section id="contact" aria-labelledby="contact-heading">
                            <h2 id="contact-heading">{content.contact.heading.as_str()}</h2>
                            <div class="markdown" inner_html=content.contact.html.as_str()></div>
                            <ul class="contact-links">{contact_links.collect_view()}</ul>
                        </section>
                    </main>

                    <footer class="site-footer">
                        <p>{format!("© {year} {}", site.name)}</p>
                        <nav aria-label="Footer links">
                            {footer_github}
                            {footer_linkedin}
                        </nav>
                    </footer>
                </div>
            </body>
        </html>
    };

    let html = page.to_html().replace("<!>", "");
    format!("<!doctype html>\n{html}\n")
}

fn organization_view(content: &SiteContent) -> impl IntoView + use<> {
    let site = &content.site;
    if site.links.organization.url.trim().is_empty() {
        Either::Left(site.organization.clone())
    } else {
        Either::Right(view! {
            <a href=site.links.organization.url.clone() rel="noopener noreferrer">
                {site.organization.clone()}
            </a>
        })
    }
}

fn optional_footer_link(link: &ContentLink) -> Option<impl IntoView + use<>> {
    (!link.url.trim().is_empty()).then(|| {
        view! {
            <a href=link.url.clone() rel="noopener noreferrer">{link.label.clone()}</a>
        }
    })
}

fn social_image_url(canonical: &str, social_image: &str) -> String {
    if canonical.is_empty() || Url::parse(social_image).is_ok() {
        return social_image.to_owned();
    }
    let base = format!("{}/", canonical.trim_end_matches('/'));
    Url::parse(&base)
        .and_then(|base| base.join(social_image))
        .map_or_else(|_| social_image.to_owned(), |url| url.to_string())
}

fn person_schema(content: &SiteContent) -> serde_json::Value {
    let site = &content.site;
    let same_as = site
        .links
        .all()
        .into_iter()
        .filter(|link| link.url.starts_with("http://") || link.url.starts_with("https://"))
        .map(|link| link.url.as_str())
        .collect::<Vec<_>>();
    let mut person = json!({
        "@context": "https://schema.org",
        "@type": "Person",
        "name": site.name,
        "jobTitle": site.role,
        "affiliation": {
            "@type": "Organization",
            "name": site.organization
        },
        "sameAs": same_as
    });
    if !site.canonical_url.trim().is_empty() {
        person["url"] = json!(site.canonical_url);
    }
    person
}

fn website_schema(content: &SiteContent) -> serde_json::Value {
    let site = &content.site;
    let mut website = json!({
        "@context": "https://schema.org",
        "@type": "WebSite",
        "name": site.seo_title,
        "description": site.seo_description,
        "author": {
            "@type": "Person",
            "name": site.name
        }
    });
    if !site.canonical_url.trim().is_empty() {
        website["url"] = json!(site.canonical_url);
    }
    website
}

fn json_ld(value: &serde_json::Value) -> String {
    serde_json::to_string(value)
        .expect("JSON-LD values are serializable")
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::load_site;
    use std::path::Path;

    #[test]
    fn output_is_static_and_contains_real_content() {
        let content_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("content");
        let content = load_site(&content_dir).expect("repository content should be valid");
        let html = render_document(&content, 2026);
        assert!(html.contains("Darshan Vichhi"));
        assert!(html.contains("Aarambh AI"));
        assert!(!html.contains("<script src="));
        assert!(!html.contains(".wasm"));
        assert!(!html.contains("type=\"module\""));
        assert!(!html.contains("<!>"));
    }
}
