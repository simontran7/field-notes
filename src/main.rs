mod layout;
mod notes;
mod render;
mod serve;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Where the notes come from: NOTES_DIR if set, otherwise the Obsidian vault next to
/// this repo (../programming) when it exists, otherwise the synced copy in notes/
/// (what GitHub Actions builds from; refresh it with scripts/sync-notes.sh).
pub(crate) fn notes_dir() -> PathBuf {
    if let Some(dir) = std::env::var_os("NOTES_DIR") {
        return PathBuf::from(dir);
    }
    let vault = PathBuf::from("../programming");
    if vault.is_dir() { vault } else { PathBuf::from("notes") }
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

/// Highlights every code block in dist/ with Shiki and the vault's Darcula theme,
/// so code looks exactly like it does in Obsidian.
fn highlight_code(dist_dir: &Path, notes_dir: &Path) -> std::io::Result<()> {
    let status = Command::new("node")
        .arg("scripts/highlight.mjs")
        .arg(dist_dir)
        .arg(notes_dir.join(".obsidian/shiki-themes/darcula.json"))
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other("syntax highlighting failed (run `npm install` first?)"))
    }
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
    let links = notes::LinkMap::new(&notes);

    let (_, home_body) = notes::split_title(read(notes_dir.join("TOC.md")));
    write(dist_dir, "index.html", &layout::home_page(&render::render_note(&home_body, "", &links)))?;

    write(dist_dir, "404.html", &layout::not_found_page())?;

    for note in &notes {
        let html = render::render_note(&note.body, &note.dir, &links);
        write(dist_dir, &format!("{}/index.html", note.slug), &layout::note_page(&note.title, &html))?;
    }

    copy_static_assets(public_dir, dist_dir)?;
    copy_static_assets(&notes_dir, dist_dir)?;
    highlight_code(dist_dir, &notes_dir)?;

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
