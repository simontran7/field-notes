use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

static TITLE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^#\s+(.+)$").unwrap());

const NOT_NOTES: [&str; 3] = ["index.md", "AGENTS.md", "README.md"];

pub struct Note {
    pub slug: String,
    pub dir: String,
    pub title: String,
    pub body: String,
}

pub fn is_hidden(name: &OsStr) -> bool {
    name.to_string_lossy().starts_with('.')
}

fn find_markdown_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        if is_hidden(&entry.file_name()) {
            continue;
        }
        let path = entry.path();
        if path.is_dir() {
            find_markdown_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
}

fn slug_for(notes_dir: &Path, file: &Path) -> String {
    let rel = file
        .strip_prefix(notes_dir)
        .unwrap()
        .with_extension("")
        .to_string_lossy()
        .replace('\\', "/");
    rel.strip_suffix("/index").map(String::from).unwrap_or(rel)
}

pub fn split_title(source: String) -> (String, String) {
    match TITLE_RE.captures(&source) {
        Some(caps) => {
            let m = caps.get(0).unwrap();
            let title = caps.get(1).unwrap().as_str().to_string();
            let body = if m.start() == 0 {
                let rest = &source[m.end()..];
                rest.strip_prefix('\n').unwrap_or(rest).to_string()
            } else {
                source
            };
            (title, body)
        }
        None => (String::new(), source),
    }
}

pub fn load_notes(notes_dir: &Path) -> Vec<Note> {
    let mut files = Vec::new();
    find_markdown_files(notes_dir, &mut files);
    files.retain(|f| {
        let top_level = f.parent() == Some(notes_dir);
        !(top_level
            && f.file_name()
                .is_some_and(|n| NOT_NOTES.contains(&&*n.to_string_lossy())))
    });
    files.sort();

    files
        .into_iter()
        .map(|file| {
            let slug = slug_for(notes_dir, &file);
            let dir = slug
                .rsplit_once('/')
                .map(|(d, _)| d.to_string())
                .unwrap_or_default();
            let source =
                fs::read_to_string(&file).unwrap_or_else(|e| panic!("reading {file:?}: {e}"));
            let (title, body) = split_title(source);

            Note {
                slug,
                dir,
                title,
                body,
            }
        })
        .collect()
}
