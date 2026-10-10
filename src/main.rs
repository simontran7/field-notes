mod highlight;
mod layout;
mod notes;
mod render;
mod serve;

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// The notes the site is built from. Override with NOTES_DIR.
pub(crate) fn notes_dir() -> PathBuf {
    std::env::var_os("NOTES_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("notes"))
}

fn copy_static_assets(src_dir: &Path, dist_dir: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        if notes::is_hidden(&name) {
            continue;
        }
        let dest = dist_dir.join(&name);

        if path.is_dir() {
            fs::create_dir_all(&dest)?;
            copy_static_assets(&path, &dest)?;
        } else if path.extension().is_none_or(|e| e != "md") {
            fs::create_dir_all(dist_dir)?;
            fs::copy(&path, &dest)?;
        }
    }
    Ok(())
}

fn write(dist_dir: &Path, rel_path: &str, contents: &str) -> std::io::Result<()> {
    let path = dist_dir.join(rel_path);
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, contents)
}

fn read(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    fs::read_to_string(path).unwrap_or_else(|e| panic!("reading {path:?}: {e}"))
}

pub(crate) fn build_site(dev: bool) -> std::io::Result<usize> {
    layout::set_dev_mode(dev);

    let notes_dir = notes_dir();
    let public_dir = Path::new("public");
    let dist_dir = Path::new("dist");

    if dist_dir.exists() {
        fs::remove_dir_all(dist_dir)?;
    }
    fs::create_dir_all(dist_dir)?;

    let notes = notes::load_notes(&notes_dir);
    let slugs: HashSet<&str> = notes.iter().map(|n| n.slug.as_str()).collect();
    let mut broken = Vec::new();

    let (_, home_body) = notes::split_title(read(notes_dir.join("index.md")));
    let home = render::render_note("index.md", &home_body, "", &slugs, &mut broken);
    write(dist_dir, "index.html", &layout::home_page(&home))?;

    write(dist_dir, "404.html", &layout::not_found_page())?;

    for note in &notes {
        let source = format!("{}.md", note.slug);
        let html = render::render_note(&source, &note.body, &note.dir, &slugs, &mut broken);
        write(dist_dir, &format!("{}/index.html", note.slug), &layout::note_page(&note.title, &html))?;
    }

    if !broken.is_empty() {
        return Err(std::io::Error::other(format!("links to notes that don't exist:\n  {}", broken.join("\n  "))));
    }

    copy_static_assets(public_dir, dist_dir)?;
    copy_static_assets(&notes_dir, dist_dir)?;

    Ok(notes.len())
}

fn main() -> std::io::Result<()> {
    let dev = std::env::args().nth(1).as_deref() == Some("serve");

    let count = build_site(dev)?;
    println!("built {count} notes -> dist");

    if dev {
        serve::run(Path::new("dist"))?;
    }

    Ok(())
}
