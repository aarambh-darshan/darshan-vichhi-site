use darshan_vichhi_site::content::load_site;
use std::{path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    let content_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content");
    match load_site(&content_dir) {
        Ok(content) => {
            println!(
                "Content is valid: {} project files loaded and ordered.",
                content.projects.len()
            );
            if content.site.canonical_url.trim().is_empty() {
                eprintln!(
                    "warning: content/site.md canonicalUrl is empty; canonical and absolute sitemap URLs will be omitted"
                );
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
