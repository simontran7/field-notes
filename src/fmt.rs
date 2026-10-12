//! `cargo run -- fmt` formats the C, C++, Python and Rust code blocks in notes/ with
//! clang-format, ruff and rustfmt, using the styles in fmt/ (4-space indents, attached
//! braces, 80 columns so lines fit a code block on the site). `fmt --check` only lists
//! the blocks that would change.
//!
//! Blocks that are pieces of a larger program (a function split across several blocks
//! with prose in between) can't be formatted on their own, so they're skipped and listed.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::LazyLock;

use regex::Regex;

static FENCE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^((?:[ \t]*>[ \t]?)*)```([\w+#-]+)[ \t]*$").unwrap());

pub(crate) fn markdown_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if crate::notes::is_hidden(&entry.file_name()) {
            continue;
        }
        if path.is_dir() {
            markdown_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
}

/// The formatter command for a language, or None if the language isn't formatted.
fn formatter(lang: &str) -> Option<Vec<&'static str>> {
    Some(match lang {
        "c" | "h" => vec![
            "clang-format",
            "--style=file:fmt/clang-format",
            "--assume-filename=block.c",
        ],
        "cpp" | "c++" | "cc" | "hpp" => {
            vec![
                "clang-format",
                "--style=file:fmt/clang-format",
                "--assume-filename=block.cpp",
            ]
        }
        "python" | "py" => vec![
            "ruff",
            "format",
            "--config",
            "fmt/ruff.toml",
            "--stdin-filename",
            "block.py",
            "-",
        ],
        "rust" | "rs" => vec![
            "rustfmt",
            "--edition",
            "2024",
            "--config-path",
            "fmt/rustfmt.toml",
        ],
        _ => return None,
    })
}

/// Strips string/char literals and comments so brackets inside them aren't counted.
static C_NOISE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?s)"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|//[^\n]*|/\*.*?\*/"#).unwrap()
});

/// Why a block can't be formatted on its own, if it can't.
fn incomplete(lang: &str, code: &str) -> Option<&'static str> {
    let first = code.lines().find(|l| !l.trim().is_empty())?;
    if first.starts_with([' ', '\t']) {
        return Some("starts indented (a piece of a larger block)");
    }
    if matches!(lang, "c" | "h" | "cpp" | "c++" | "cc" | "hpp") {
        let code = C_NOISE_RE.replace_all(code, "");
        for (open, close) in [('{', '}'), ('(', ')'), ('[', ']')] {
            if code.matches(open).count() != code.matches(close).count() {
                return Some("unbalanced brackets (a piece of a larger block)");
            }
        }
        // e.g. the cleanup labels at the end of a function split across blocks
        if Regex::new(r"^[A-Za-z_]\w*:\s*$").unwrap().is_match(first) {
            return Some("starts with a label (a piece of a larger block)");
        }
    }
    // Comments lined up in a column (`a.checked_add(b);      // None`) are deliberate;
    // rustfmt and ruff would collapse the gap. clang-format keeps them aligned itself.
    let aligned_comment = match lang {
        "rust" | "rs" => Some(r"\S {2,}//"),
        "python" | "py" => Some(r"\S {3,}#"),
        _ => None,
    };
    if aligned_comment.is_some_and(|re| Regex::new(re).unwrap().is_match(code)) {
        return Some("comments aligned in a column");
    }
    None
}

fn run_formatter(cmd: &[&str], code: &str) -> Result<String, String> {
    let mut child = Command::new(cmd[0])
        .args(&cmd[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("can't run {}: {e}", cmd[0]))?;
    child
        .stdin
        .take()
        .unwrap()
        .write_all(code.as_bytes())
        .map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        let err = String::from_utf8_lossy(&out.stderr);
        Err(err
            .lines()
            .find(|l| l.contains("error"))
            .unwrap_or("formatter failed")
            .trim()
            .to_string())
    }
}

/// rustfmt only formats whole items, so a snippet of statements (`let x = ...;`) is
/// wrapped in a function, formatted, and unwrapped again.
fn rust_statements(lang: &str, cmd: &[&str], code: &str, err: String) -> Result<String, String> {
    if !matches!(lang, "rust" | "rs") {
        return Err(err);
    }
    let wrapped = format!("fn __snippet__() {{\n{code}}}\n");
    let formatted = run_formatter(cmd, &wrapped).map_err(|_| err)?;
    let inner: Vec<&str> = formatted.lines().collect();
    if inner.first() != Some(&"fn __snippet__() {") || inner.last() != Some(&"}") {
        return Err("rustfmt changed the wrapper".into());
    }
    Ok(inner[1..inner.len() - 1]
        .iter()
        .map(|l| l.strip_prefix("    ").unwrap_or(l))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n")
}

pub fn run(check: bool) -> std::io::Result<()> {
    let mut files = Vec::new();
    markdown_files(&crate::notes_dir(), &mut files);
    files.sort();

    let (mut changed, mut unchanged, mut skipped) = (0, 0, Vec::new());
    for file in files {
        let source = fs::read_to_string(&file)?;
        let lines: Vec<&str> = source.split('\n').collect();
        let mut out: Vec<String> = Vec::with_capacity(lines.len());
        let mut i = 0;
        let mut file_changed = false;

        while i < lines.len() {
            let Some(caps) = FENCE_RE.captures(lines[i]) else {
                out.push(lines[i].to_string());
                i += 1;
                continue;
            };
            let prefix = caps[1].to_string();
            let lang = caps[2].to_string();
            let close = format!("{prefix}```");
            let Some(end) = (i + 1..lines.len()).find(|&j| lines[j].trim_end() == close.trim_end())
            else {
                out.push(lines[i].to_string());
                i += 1;
                continue;
            };

            // Inside a callout every line starts with "> "; empty lines may be just ">"
            let body: Option<Vec<String>> = lines[i + 1..end]
                .iter()
                .map(|l| {
                    l.strip_prefix(prefix.as_str())
                        .map(String::from)
                        .or_else(|| (l.trim_end() == prefix.trim_end()).then(String::new))
                })
                .collect();

            let location = format!(
                "{}:{}",
                file.strip_prefix(crate::notes_dir())
                    .unwrap_or(&file)
                    .display(),
                i + 1
            );
            let mut replacement = None;
            if let (Some(cmd), Some(body)) = (formatter(&lang), &body) {
                let code = body.join("\n") + "\n";
                if let Some(reason) = incomplete(&lang, &code) {
                    skipped.push(format!("{location} ({lang}): {reason}"));
                } else {
                    match run_formatter(&cmd, &code)
                        .or_else(|e| rust_statements(&lang, &cmd, &code, e))
                    {
                        Ok(formatted) if formatted != code => replacement = Some(formatted),
                        Ok(_) => unchanged += 1,
                        Err(e) => skipped.push(format!("{location} ({lang}): {e}")),
                    }
                }
            }

            out.push(lines[i].to_string());
            match replacement {
                Some(formatted) => {
                    changed += 1;
                    file_changed = true;
                    println!(
                        "{} {location} ({lang})",
                        if check { "would format" } else { "formatted" }
                    );
                    for l in formatted.trim_end_matches('\n').split('\n') {
                        out.push(if l.is_empty() {
                            prefix.trim_end().to_string()
                        } else {
                            format!("{prefix}{l}")
                        });
                    }
                }
                None => out.extend(lines[i + 1..end].iter().map(|l| l.to_string())),
            }
            out.push(lines[end].to_string());
            i = end + 1;
        }

        if file_changed && !check {
            fs::write(&file, out.join("\n"))?;
        }
    }

    println!(
        "\n{changed} block(s) {}, {unchanged} already formatted, {} skipped:",
        if check {
            "need formatting"
        } else {
            "formatted"
        },
        skipped.len()
    );
    for s in &skipped {
        println!("  {s}");
    }
    Ok(())
}
