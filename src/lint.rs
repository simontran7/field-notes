//! `cargo run -- lint` checks notes/ against the house style in notes/AGENTS.md that
//! the renderer doesn't enforce: no em dashes, `_` for italics, centered tables, only
//! the supported callout types, and `bash`/`console` for shell blocks. It lists every
//! violation and fails if there are any.

use std::fs;
use std::sync::LazyLock;

use regex::Regex;

const CALLOUT_TYPES: [&str; 7] = [
    "DEFINITION",
    "THEOREM",
    "PROPOSITION",
    "COROLLARY",
    "NOTE",
    "TIP",
    "WARNING",
];

const SHELL_LANGS: [&str; 3] = ["sh", "shell", "zsh"];

static FENCE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:[ \t]*>[ \t]?)*[ \t]*```[ \t]*([\w+#-]*)").unwrap());

static CALLOUT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:[ \t]*>[ \t]*)+\[!(\w+)\]").unwrap());

/// A table's delimiter row, e.g. `| :---: | --- |`.
static TABLE_DELIM_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[ \t]*>[ \t]?)*[ \t]*\|?(?:[ \t]*:?-+:?[ \t]*\|)+(?:[ \t]*:?-+:?[ \t]*)?$")
        .unwrap()
});

static CENTERED_CELL_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^:-+:$").unwrap());

/// Inline code and math, which may contain `*` that isn't emphasis.
static CODE_OR_MATH_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(`+)[^`]*?`+|\$\$.*?\$\$|\$[^$]*\$").unwrap());

/// `*text*` with no space just inside the stars (so `a * b * c` doesn't count).
static STAR_ITALIC_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*[^\s*](?:[^*]*[^\s*\\])?\*").unwrap());

/// The style problems on one line outside a code block.
fn check_line(line: &str) -> Vec<String> {
    let mut problems = Vec::new();
    if line.contains('\u{2014}') {
        problems.push("em dash".to_string());
    }
    if let Some(caps) = CALLOUT_RE.captures(line) {
        if !CALLOUT_TYPES.contains(&&caps[1]) {
            problems.push(format!(
                "callout [!{}] (use one of {})",
                &caps[1],
                CALLOUT_TYPES.join(", ")
            ));
        }
    }
    if TABLE_DELIM_RE.is_match(line) {
        let cells = line.trim_start_matches(['>', ' ', '\t']).trim();
        let cells = cells.trim_start_matches('|').trim_end_matches('|');
        if !cells
            .split('|')
            .all(|c| CENTERED_CELL_RE.is_match(c.trim()))
        {
            problems.push("table column not centered (use :---:)".to_string());
        }
    }
    let prose = CODE_OR_MATH_RE.replace_all(line, "").replace("**", "");
    if let Some(m) = STAR_ITALIC_RE.find(&prose) {
        problems.push(format!("`{}` uses * for italics (use _)", m.as_str()));
    }
    problems
}

/// Every style problem in a note, as (line number, problem).
fn check(source: &str) -> Vec<(usize, String)> {
    let mut problems = Vec::new();
    let mut in_code = false;
    for (i, line) in source.lines().enumerate() {
        if let Some(caps) = FENCE_RE.captures(line) {
            let lang = &caps[1];
            if !in_code && SHELL_LANGS.contains(&lang) {
                problems.push((i + 1, format!("```{lang} block (use bash or console)")));
            }
            in_code = !in_code;
            continue;
        }
        if in_code {
            continue;
        }
        problems.extend(check_line(line).into_iter().map(|p| (i + 1, p)));
    }
    problems
}

pub fn run() -> std::io::Result<()> {
    let notes_dir = crate::ssg::notes_dir();
    let mut files = Vec::new();
    crate::fmt::markdown_files(&notes_dir, &mut files);
    files.sort();

    let mut count = 0;
    for file in &files {
        let rel = file.strip_prefix(&notes_dir).unwrap_or(file).display();
        for (line, problem) in check(&fs::read_to_string(file)?) {
            println!("{rel}:{line}: {problem}");
            count += 1;
        }
    }

    if count > 0 {
        return Err(std::io::Error::other(format!("{count} style problem(s)")));
    }
    println!("checked {} notes, no style problems", files.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problems(source: &str) -> Vec<String> {
        check(source).into_iter().map(|(_, p)| p).collect()
    }

    #[test]
    fn clean_note_passes() {
        let note = "# Title\n\nSome _italic_ and **bold** text, $a * b * c$ and `x*y*z`.\n\n\
                    > [!DEFINITION] Cache\n> A **cache** is fast.\n\n\
                    | a | b |\n| :---: | :---: |\n| 1 | 2 |\n\n\
                    ```bash\nls *.md*\n```\n\n- list item\n* star item\n";
        assert_eq!(problems(note), Vec::<String>::new());
    }

    #[test]
    fn flags_each_rule() {
        assert_eq!(problems("a \u{2014} b\n").len(), 1);
        assert_eq!(problems("one *italic* word\n").len(), 1);
        assert_eq!(problems("> [!IMPORTANT]\n> x\n").len(), 1);
        assert_eq!(problems("| a | b |\n| --- | :---: |\n").len(), 1);
        assert_eq!(problems("```sh\nls\n```\n").len(), 1);
    }

    #[test]
    fn ignores_code_blocks() {
        assert_eq!(problems("```\n*a* \u{2014} | --- |\n```\n").len(), 0);
        assert_eq!(problems("> ```c\n> int *p = *q;\n> ```\n").len(), 0);
    }
}
