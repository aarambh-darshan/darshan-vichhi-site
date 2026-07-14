use darshan_vichhi_site::{content::load_site, render::render_document};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    if let Err(error) = build() {
        eprintln!("site build failed: {error}");
        std::process::exit(1);
    }
}

fn build() -> Result<(), Box<dyn Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let content = load_site(&root.join("content"))?;
    let html = render_document(&content, current_year());
    let dist = root.join("dist");

    if dist.exists() {
        fs::remove_dir_all(&dist)?;
    }
    fs::create_dir_all(&dist)?;
    copy_directory(&root.join("assets"), &dist.join("assets"))?;
    fs::write(dist.join("index.html"), html)?;
    fs::write(
        dist.join("sitemap.xml"),
        sitemap(&content.site.canonical_url),
    )?;
    fs::write(dist.join("robots.txt"), robots(&content.site.canonical_url))?;
    fs::write(dist.join(".nojekyll"), "")?;

    if content.site.canonical_url.trim().is_empty() {
        eprintln!(
            "warning: content/site.md canonicalUrl is empty; canonical, Open Graph URL, and sitemap location entries were omitted"
        );
    }
    println!("Built static site at {}", dist.display());
    Ok(())
}

fn copy_directory(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    let mut entries = fs::read_dir(source)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_directory(&source_path, &destination_path)?;
        } else {
            fs::copy(source_path, destination_path)?;
        }
    }
    Ok(())
}

fn sitemap(canonical_url: &str) -> String {
    let location = canonical_url.trim();
    if location.is_empty() {
        return concat!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n",
            "<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\"></urlset>\n"
        )
        .to_owned();
    }
    format!(
        concat!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n",
            "<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
            "  <url><loc>{}</loc></url>\n",
            "</urlset>\n"
        ),
        escape_xml(location)
    )
}

fn robots(canonical_url: &str) -> String {
    let location = canonical_url.trim();
    if location.is_empty() {
        "User-agent: *\nAllow: /\n".to_owned()
    } else {
        format!(
            "User-agent: *\nAllow: /\nSitemap: {}/sitemap.xml\n",
            location.trim_end_matches('/')
        )
    }
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn current_year() -> i32 {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    year_from_unix_days((seconds / 86_400) as i64)
}

fn year_from_unix_days(days: i64) -> i32 {
    let shifted = days + 719_468;
    let era = if shifted >= 0 {
        shifted
    } else {
        shifted - 146_096
    } / 146_097;
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_part = (5 * day_of_year + 2) / 153;
    (year_of_era + era * 400 + i64::from(month_part >= 10)) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_year_from_unix_days() {
        assert_eq!(year_from_unix_days(0), 1970);
        assert_eq!(year_from_unix_days(20_454), 2026);
    }

    #[test]
    fn empty_canonical_produces_valid_empty_sitemap() {
        let output = sitemap("");
        assert!(output.contains("<urlset"));
        assert!(!output.contains("<loc>"));
    }
}
