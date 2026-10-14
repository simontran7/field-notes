mod diagram;
mod highlight;
mod layout;
mod math;
pub(crate) mod notes;
mod render;

use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

pub const DIST_DIR: &str = "dist";
pub const PUBLIC_DIR: &str = "public";

pub(crate) fn notes_dir() -> PathBuf {
    std::env::var_os("NOTES_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("notes"))
}

fn hash(value: impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

fn read(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    fs::read_to_string(path).unwrap_or_else(|e| panic!("reading {path:?}: {e}"))
}

struct Rendered {
    html: String,
    broken: Vec<String>,
}

pub struct Stats {
    pub notes: usize,
    pub pages: usize,
    pub rendered: usize,
}

pub struct Site {
    rendered: HashMap<String, (u64, Rendered)>,
    written: HashMap<PathBuf, u64>,
    built_once: bool,
}

impl Site {
    pub fn new(dev: bool) -> Self {
        layout::set_dev_mode(dev);
        Site {
            rendered: HashMap::new(),
            written: HashMap::new(),
            built_once: false,
        }
    }

    pub fn build(&mut self) -> std::io::Result<Stats> {
        let notes_dir = notes_dir();
        let dist_dir = Path::new(DIST_DIR);

        if !self.built_once && dist_dir.exists() {
            fs::remove_dir_all(dist_dir)?;
        }
        self.built_once = true;
        fs::create_dir_all(dist_dir)?;

        let notes = notes::load_notes(&notes_dir);
        let slugs: HashSet<&str> = notes.iter().map(|n| n.slug.as_str()).collect();
        let mut sorted_slugs: Vec<&str> = slugs.iter().copied().collect();
        sorted_slugs.sort_unstable();
        let slugs_hash = hash(&sorted_slugs);

        let mut output = Output::new(dist_dir, std::mem::take(&mut self.written));
        let mut broken = Vec::new();
        let mut rendered = 0;
        let mut render = |source: String, body: &str, dir: &str| {
            let key = hash((&source, body, dir, slugs_hash));
            let cached = self.rendered.get(&source).filter(|(k, _)| *k == key);
            if cached.is_none() {
                let mut broken = Vec::new();
                let html = render::render_note(&source, body, dir, &slugs, &mut broken);
                self.rendered
                    .insert(source.clone(), (key, Rendered { html, broken }));
                rendered += 1;
            }
            let (_, note) = &self.rendered[&source];
            broken.extend(note.broken.iter().cloned());
            note.html.clone()
        };

        let (_, home_body) = notes::split_title(read(notes_dir.join("index.md")));
        let home = render("index.md".to_string(), &home_body, "");
        output.page("index.html", &layout::home_page(&home))?;

        output.page("404.html", &layout::not_found_page())?;

        for note in &notes {
            let html = render(format!("{}.md", note.slug), &note.body, &note.dir);
            output.page(
                &format!("{}/index.html", note.slug),
                &layout::note_page(&note.title, &html),
            )?;
        }

        let sources: HashSet<String> = notes.iter().map(|n| format!("{}.md", n.slug)).collect();
        self.rendered
            .retain(|source, _| source == "index.md" || sources.contains(source));

        output.static_assets(Path::new(PUBLIC_DIR), dist_dir)?;
        output.static_assets(&notes_dir, dist_dir)?;
        self.written = output.finish();

        if !broken.is_empty() {
            return Err(std::io::Error::other(format!(
                "links to notes that don't exist:\n  {}",
                broken.join("\n  ")
            )));
        }

        Ok(Stats {
            notes: notes.len(),
            pages: notes.len() + 1,
            rendered,
        })
    }
}

struct Output<'a> {
    dist_dir: &'a Path,
    previous: HashMap<PathBuf, u64>,
    current: HashMap<PathBuf, u64>,
}

impl<'a> Output<'a> {
    fn new(dist_dir: &'a Path, previous: HashMap<PathBuf, u64>) -> Self {
        Output {
            dist_dir,
            previous,
            current: HashMap::new(),
        }
    }

    fn put(
        &mut self,
        dest: PathBuf,
        key: u64,
        write: impl FnOnce(&Path) -> std::io::Result<()>,
    ) -> std::io::Result<()> {
        if self.previous.get(&dest) != Some(&key) || !dest.exists() {
            fs::create_dir_all(dest.parent().unwrap())?;
            let mut tmp = dest.clone().into_os_string();
            tmp.push(".tmp");
            let tmp = PathBuf::from(tmp);
            write(&tmp)?;
            fs::rename(&tmp, &dest)?;
        }
        self.current.insert(dest, key);
        Ok(())
    }

    fn page(&mut self, rel_path: &str, contents: &str) -> std::io::Result<()> {
        let dest = self.dist_dir.join(rel_path);
        self.put(dest, hash(contents), |tmp| fs::write(tmp, contents))
    }

    fn static_assets(&mut self, src_dir: &Path, dist_dir: &Path) -> std::io::Result<()> {
        for entry in fs::read_dir(src_dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name();
            if notes::is_hidden(&name) {
                continue;
            }
            let dest = dist_dir.join(&name);

            if path.is_dir() {
                self.static_assets(&path, &dest)?;
            } else if path.extension().is_none_or(|e| e != "md") {
                let meta = entry.metadata()?;
                let key = hash((meta.len(), meta.modified()?));
                self.put(dest, key, |tmp| fs::copy(&path, tmp).map(|_| ()))?;
            }
        }
        Ok(())
    }

    fn finish(self) -> HashMap<PathBuf, u64> {
        for stale in self
            .previous
            .keys()
            .filter(|p| !self.current.contains_key(*p))
        {
            let _ = fs::remove_file(stale);
            for dir in stale.ancestors().skip(1) {
                if dir == self.dist_dir || fs::remove_dir(dir).is_err() {
                    break;
                }
            }
        }
        self.current
    }
}
