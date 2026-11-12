//! Renders ```tikz code blocks to inline SVG at build time.
//!
//! Each block compiles with `tectonic` (TikZ to PDF), then `pdftocairo -svg`
//! (PDF to SVG). The raw SVG is cached in `tikz-cache/<hash>.svg`, which is
//! committed, so builds without TeX installed (CI) reuse it. A block only
//! needs the tools when its source changes.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const CACHE_DIR: &str = "tikz-cache";

/// Bump when the document template changes, so cached drawings recompile.
const TEMPLATE_VERSION: &str = "2";

/// Lines that belong in the preamble when a block is wrapped.
const PREAMBLE_COMMANDS: [&str; 3] = [r"\usepackage", r"\usetikzlibrary", r"\usepgfplotslibrary"];

/// FNV-1a: stable across Rust versions, so cache names stay valid.
fn stable_hash(text: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in TEMPLATE_VERSION.bytes().chain([0]).chain(text.bytes()) {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

/// Wraps a block in a standalone document. A block that has its own
/// `\documentclass` is used as is. Otherwise `\usepackage` and
/// `\usetikzlibrary` lines go to the preamble, and a body without
/// `\begin{tikzpicture}` is wrapped in one.
fn document(code: &str) -> String {
    if code.contains(r"\documentclass") {
        return code.to_string();
    }
    let (preamble, body): (Vec<&str>, Vec<&str>) = code.lines().partition(|line| {
        let line = line.trim_start();
        PREAMBLE_COMMANDS.iter().any(|c| line.starts_with(c))
    });
    let body = body.join("\n");
    let body = if body.contains(r"\begin{tikzpicture}") {
        body
    } else {
        format!("\\begin{{tikzpicture}}\n{body}\n\\end{{tikzpicture}}")
    };
    format!(
        "\\documentclass[tikz,border=2pt]{{standalone}}\n\
         \\renewcommand{{\\familydefault}}{{\\sfdefault}}\n\
         \\usetikzlibrary{{arrows.meta,calc,positioning}}\n\
         \\tikzset{{every picture/.style={{thick, >=Stealth}}, \
         hl/.style={{fill=black, fill opacity=0.12, text opacity=1}}}}\n\
         {}\n\
         \\begin{{document}}\n{body}\n\\end{{document}}\n",
        preamble.join("\n"),
    )
}

fn run(cmd: &mut Command) -> Result<(), String> {
    let name = cmd.get_program().to_string_lossy().into_owned();
    let out = cmd.output().map_err(|e| format!("can't run `{name}`: {e}"))?;
    if out.status.success() {
        return Ok(());
    }
    let log = String::from_utf8_lossy(&out.stdout) + String::from_utf8_lossy(&out.stderr);
    let tail: Vec<&str> = log.lines().rev().take(15).collect();
    let tail: Vec<&str> = tail.into_iter().rev().collect();
    Err(format!("`{name}` failed:\n    {}", tail.join("\n    ")))
}

fn compile(code: &str, key: &str) -> Result<String, String> {
    // Per process: `serve` and a manual build can compile the same block at once.
    let dir = std::env::temp_dir().join(format!("field-notes-tikz-{key}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    fs::write(dir.join("fig.tex"), document(code)).map_err(|e| e.to_string())?;
    let result = run(Command::new("tectonic")
        .args(["-X", "compile", "fig.tex"])
        .current_dir(&dir))
    .and_then(|()| {
        run(Command::new("pdftocairo")
            .args(["-svg", "fig.pdf", "fig.svg"])
            .current_dir(&dir))
    })
    .and_then(|()| fs::read_to_string(dir.join("fig.svg")).map_err(|e| e.to_string()));
    let _ = fs::remove_dir_all(&dir);
    result
}

/// Prepares raw pdftocairo output for inlining: drops the XML declaration,
/// makes black follow the text color (so dark mode works), and prefixes ids
/// so several drawings on one page don't share glyph or clip ids.
fn inline(svg: &str, key: &str) -> String {
    let svg = svg.trim();
    let svg = match svg.strip_prefix("<?xml") {
        Some(rest) => rest.split_once("?>").map_or(rest, |(_, s)| s).trim_start(),
        None => svg,
    };
    let prefix = format!("tikz-{key}-");
    svg.replace("rgb(0%, 0%, 0%)", "currentColor")
        .replace(r#"id=""#, &format!(r#"id="{prefix}"#))
        .replace(r##"href="#"##, &format!(r##"href="#{prefix}"##))
        .replace("url(#", &format!("url(#{prefix}"))
        .replacen("<svg ", r#"<svg role="img" "#, 1)
}

fn cache_path(key: &str) -> PathBuf {
    Path::new(CACHE_DIR).join(format!("{key}.svg"))
}

/// Returns the drawing as HTML, or `None` (after a warning) when it isn't
/// cached and can't be compiled, so the caller can show the source instead.
pub fn render(code: &str) -> Option<String> {
    let key = stable_hash(code);
    let path = cache_path(&key);
    let svg = match fs::read_to_string(&path) {
        Ok(svg) => svg,
        Err(_) => match compile(code, &key) {
            Ok(svg) => {
                // Write then rename, so a concurrent build never reads a partial file.
                let tmp = path.with_extension(format!("svg.{}.tmp", std::process::id()));
                if let Err(e) = fs::create_dir_all(CACHE_DIR)
                    .and_then(|()| fs::write(&tmp, &svg))
                    .and_then(|()| fs::rename(&tmp, &path))
                {
                    eprintln!("warning: can't cache tikz drawing {path:?}: {e}");
                }
                svg
            }
            Err(e) => {
                eprintln!("warning: can't render tikz drawing {key}: {e}");
                return None;
            }
        },
    };
    Some(format!(r#"<div class="tikz">{}</div>"#, inline(&svg, &key)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_stable() {
        assert_eq!(stable_hash("x"), stable_hash("x"));
        assert_ne!(stable_hash("x"), stable_hash("y"));
        assert_eq!(stable_hash("x").len(), 16);
    }

    #[test]
    fn bare_drawings_are_wrapped() {
        let doc = document("\\usetikzlibrary{calc}\n\\draw (0,0) -- (1,1);");
        assert!(doc.starts_with(r"\documentclass[tikz,border=2pt]{standalone}"), "{doc}");
        let preamble_end = doc.find(r"\begin{document}").unwrap();
        assert!(doc[..preamble_end].contains(r"\usetikzlibrary{calc}"), "{doc}");
        assert!(doc.contains("\\begin{tikzpicture}\n\\draw (0,0) -- (1,1);\n\\end{tikzpicture}"), "{doc}");
    }

    #[test]
    fn pictures_and_full_documents_are_not_rewrapped() {
        let doc = document("\\begin{tikzpicture}\n\\draw (0,0) -- (1,1);\n\\end{tikzpicture}");
        assert_eq!(doc.matches(r"\begin{tikzpicture}").count(), 1, "{doc}");
        let full = "\\documentclass{standalone}\n\\begin{document}x\\end{document}";
        assert_eq!(document(full), full);
    }

    #[test]
    fn inlined_svg_follows_text_color_and_has_unique_ids() {
        let raw = r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="10pt"><defs><g id="glyph-0-0"/><clipPath id="clip-0"/></defs><g clip-path="url(#clip-0)" fill="rgb(0%, 0%, 0%)"><use xlink:href="#glyph-0-0"/></g></svg>"##;
        let svg = inline(raw, "abc");
        assert!(svg.starts_with(r#"<svg role="img" xmlns"#), "{svg}");
        assert!(svg.contains(r#"id="tikz-abc-glyph-0-0""#), "{svg}");
        assert!(svg.contains(r##"xlink:href="#tikz-abc-glyph-0-0""##), "{svg}");
        assert!(svg.contains("url(#tikz-abc-clip-0)"), "{svg}");
        assert!(svg.contains(r#"fill="currentColor""#), "{svg}");
        assert!(!svg.contains("rgb("), "{svg}");
    }
}
