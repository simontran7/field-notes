//! Syntax highlighting with Lumis (tree-sitter parsers + Neovim's highlight queries)
//! and the Darcula theme in theme/darcula.json.
//!
//! Tree-sitter parses the code, so it can tell a function declaration from a call,
//! fields from variables, and so on, which Darcula colors differently.

use std::sync::LazyLock;

use lumis::languages::Language;
use lumis::themes::{self, Theme};
use lumis::{highlight, HtmlInlineBuilder};
use regex::Regex;

static THEME: LazyLock<Theme> =
    LazyLock::new(|| themes::from_json(include_str!("../theme/darcula.json")).expect("theme/darcula.json is invalid"));

const PLAIN: &str = "#a9b7c6";
const ORANGE: &str = "#cc7832";

// Placeholder colors from theme/darcula.json for tokens tree-sitter groups together
// but Darcula colors differently. They are resolved by text below.
const DELIMITER: &str = "#a9b7c5"; // ; and , are orange, . and :: are plain
const RUST_OPERATOR: &str = "#a9b7c4"; // ? is bold orange, other operators plain
const RUST_MODIFIER: &str = "#a9b7c3"; // the ' of a lifetime, or mut / const / dyn
const RUST_ATTRIBUTE: &str = "#a9b7c2"; // a lifetime's name, or an #[attribute]

fn span(color: &str, text: &str) -> String {
    format!(r#"<span style="color: {color};">{text}</span>"#)
}

static LIFETIME_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r#"<span style="color: {RUST_MODIFIER};">&#39;</span><span style="color: {RUST_ATTRIBUTE};">([^<]*)</span>"#
    ))
    .unwrap()
});

static PLACEHOLDER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r#"<span style="color: ({DELIMITER}|{RUST_OPERATOR}|{RUST_MODIFIER}|{RUST_ATTRIBUTE});">([^<]*)</span>"#
    ))
    .unwrap()
});

fn resolve_placeholders(html: &str) -> String {
    // Rust lifetimes ('a) are teal italic as a whole (Rust plugin LIFETIME)
    let html = LIFETIME_RE.replace_all(
        html,
        r#"<span style="color: #20999d; font-style: italic;">&#39;$1</span>"#,
    );
    PLACEHOLDER_RE
        .replace_all(&html, |caps: &regex::Captures| {
            let text = &caps[2];
            match (&caps[1], text) {
                (DELIMITER, ";" | ",") => span(ORANGE, text),
                (DELIMITER, _) => span(PLAIN, text),
                (RUST_OPERATOR, "?") => {
                    format!(r#"<span style="color: {ORANGE}; font-weight: bold;">?</span>"#)
                }
                (RUST_OPERATOR, _) => span(PLAIN, text),
                (RUST_MODIFIER, _) => span(ORANGE, text),
                (RUST_ATTRIBUTE, _) => span("#bbb529", text),
                _ => caps[0].to_string(),
            }
        })
        .into_owned()
}

static EMPTY_LINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<div class="l-line" (data-line="\d+")>(\n?)</div>"#).unwrap());

/// Marks blank lines so the bash `$` prompt can skip them.
fn mark_empty_lines(html: &str) -> String {
    EMPTY_LINE_RE.replace_all(html, r#"<div class="l-line l-empty" $1>$2</div>"#).into_owned()
}

/// Python names PyCharm colors by what they are, wherever they appear
/// (PythonDarcula.xml): built-ins, `self`, dunder names and decorators.
const PYTHON_BUILTINS: &[&str] = &[
    "abs", "all", "any", "ascii", "bin", "bool", "breakpoint", "bytearray", "bytes", "callable", "chr",
    "classmethod", "compile", "complex", "delattr", "dict", "dir", "divmod", "enumerate", "eval", "exec",
    "filter", "float", "format", "frozenset", "getattr", "globals", "hasattr", "hash", "help", "hex", "id",
    "input", "int", "isinstance", "issubclass", "iter", "len", "list", "locals", "map", "max", "memoryview",
    "min", "next", "object", "oct", "open", "ord", "pow", "print", "property", "range", "repr", "reversed",
    "round", "set", "setattr", "slice", "sorted", "staticmethod", "str", "sum", "super", "tuple", "type",
    "vars", "zip",
];

static TOKEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<span style="color: (#[0-9a-f]{6});([^"]*)">([^<]*)</span>"#).unwrap());

/// A decorator comes out as `<span yellow>@<span yellow>name</span>...</span>`.
static DECORATOR_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<span style="color: #ffc66d;">@((?:<span[^>]*>[^<]*</span>|[^<])*)</span>"#).unwrap()
});

fn python_names(html: &str) -> String {
    let html = DECORATOR_RE.replace_all(html, |caps: &regex::Captures| {
        let inner = caps[1].replace("color: #ffc66d;", "color: #bbb529;");
        format!(r#"<span style="color: #bbb529;">@{inner}</span>"#)
    });
    TOKEN_RE
        .replace_all(&html, |caps: &regex::Captures| {
            let (color, extra, text) = (&caps[1], &caps[2], &caps[3]);
            let is_dunder = text.len() > 4 && text.starts_with("__") && text.ends_with("__");
            let new_color = match text {
                "self" | "cls" => "#94558d",
                _ if is_dunder => "#b200b2",
                _ if color == PLAIN && PYTHON_BUILTINS.contains(&text) => "#8888c6",
                _ => return caps[0].to_string(),
            };
            format!(r#"<span style="color: {new_color};{extra}">{text}</span>"#)
        })
        .into_owned()
}

// Rust plugin colors (RustDarcula.xml) that need context tree-sitter doesn't give:
const RUST_TYPE: &str = "#86985d"; // STRUCT / ENUM / TYPE_ALIAS
const RUST_TRAIT: &str = "#8d91dc"; // TRAIT, CRATE
const RUST_TYPE_PARAMETER: &str = "#20999d";
const RUST_MACRO: &str = "#4eade5";
const ATTRIBUTE: &str = "#bbb529";

static LINE_DIV_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?s)(<div class="l-line[^"]*" data-line="\d+">)(.*?)(</div>)"#).unwrap());

static SPAN_OR_TEXT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<span style="color: (#[0-9a-f]{6});([^"]*)">([^<]*)</span>|([^<]+)"#).unwrap());

struct Token {
    color: Option<String>,
    extra: String,
    text: String,
}

impl Token {
    fn html(&self) -> String {
        match &self.color {
            Some(color) => format!(r#"<span style="color: {color};{}">{}</span>"#, self.extra, self.text),
            None => self.text.clone(),
        }
    }

    fn is_word(&self) -> bool {
        !self.text.trim().is_empty()
    }
}

static TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<span ([^>]*)>|</span>"#).unwrap());

/// `<span A>x<span B>y</span>z</span>` -> `<span A>x</span><span B>y</span><span A>z</span>`,
/// so every piece of text sits in exactly one span.
fn flatten_spans(line: &str) -> String {
    let mut out = String::new();
    let mut stack: Vec<String> = Vec::new();
    let mut last = 0;
    for m in TAG_RE.find_iter(line) {
        let text = &line[last..m.start()];
        if !text.is_empty() {
            match stack.last() {
                Some(attrs) => out.push_str(&format!("<span {attrs}>{text}</span>")),
                None => out.push_str(text),
            }
        }
        let tag = m.as_str();
        if tag == "</span>" {
            stack.pop();
        } else {
            stack.push(tag["<span ".len()..tag.len() - 1].to_string());
        }
        last = m.end();
    }
    out.push_str(&line[last..]);
    out
}

fn tokens(line: &str) -> Option<Vec<Token>> {
    let line = &flatten_spans(line);
    let mut out = Vec::new();
    let mut end = 0;
    for caps in SPAN_OR_TEXT_RE.captures_iter(line) {
        let m = caps.get(0).unwrap();
        if m.start() != end {
            return None; // nested spans: leave the line as is
        }
        end = m.end();
        out.push(match caps.get(4) {
            Some(text) => Token { color: None, extra: String::new(), text: text.as_str().to_string() },
            None => Token { color: Some(caps[1].to_string()), extra: caps[2].to_string(), text: caps[3].to_string() },
        });
    }
    (end == line.len()).then_some(out)
}

fn rust_line(line: &str) -> String {
    let Some(mut toks) = tokens(line) else { return line.to_string() };
    let words: Vec<usize> = (0..toks.len()).filter(|&i| toks[i].is_word()).collect();
    let text = |toks: &Vec<Token>, w: usize| toks[words[w]].text.trim().to_string();

    // #[attribute] lines are ATTRIBUTE colored throughout (strings keep their color)
    if words.len() >= 2 && text(&toks, 0) == "#" && text(&toks, 1).starts_with('[') {
        for t in toks.iter_mut().filter(|t| t.color.is_some() && t.color.as_deref() != Some("#6a8759")) {
            t.color = Some(ATTRIBUTE.to_string());
        }
        return toks.iter().map(Token::html).collect();
    }

    for w in 0..words.len() {
        let i = words[w];
        let word = text(&toks, w);
        let prev = if w > 0 { text(&toks, w - 1) } else { String::new() };
        let color = toks[i].color.clone().unwrap_or_default();

        if word.ends_with('!') && word.len() > 1 && word[..word.len() - 1].chars().all(|c| c.is_alphanumeric() || c == '_') {
            toks[i].color = Some(RUST_MACRO.to_string());
        } else if color == RUST_TYPE && word.len() <= 2 && word.chars().next().is_some_and(|c| c.is_ascii_uppercase()) && word.chars().skip(1).all(|c| c.is_ascii_digit()) {
            toks[i].color = Some(RUST_TYPE_PARAMETER.to_string());
        } else if color == RUST_TYPE && (prev == "trait" || prev == "dyn" || (prev == "impl" && w > 1)) {
            // `trait X`, `dyn X`, and `impl X` in type position (`-> impl Iterator`)
            toks[i].color = Some(RUST_TRAIT.to_string());
        } else if color == RUST_TYPE && prev == "impl" || color == RUST_TYPE && prev == "&gt;" && words.iter().take(w).any(|&j| toks[j].text.trim() == "impl") && !words.iter().take(w).any(|&j| toks[j].text.trim() == "for") {
            // `impl X for Y` / `impl<T> X for Y`: X is a trait when `for` follows on the line
            let rest_has_for = words[w + 1..].iter().any(|&j| toks[j].text.trim() == "for");
            if rest_has_for {
                toks[i].color = Some(RUST_TRAIT.to_string());
            }
        } else if matches!(word.as_str(), "std" | "core" | "alloc") && w + 1 < words.len() && text(&toks, w + 1).starts_with("::") {
            toks[i].color = Some(RUST_TRAIT.to_string());
        }
    }
    toks.iter().map(Token::html).collect()
}

fn rust_context(html: &str) -> String {
    LINE_DIV_RE
        .replace_all(html, |caps: &regex::Captures| format!("{}{}{}", &caps[1], rust_line(&caps[2]), &caps[3]))
        .into_owned()
}

// CLion colors (OCDarcula.xml, CLion 2026.2.3.1) that need context tree-sitter doesn't give:
const C_MACRO: &str = "#908b25"; // OC.MACRONAME
const C_TYPEDEF: &str = "#b9bcd1"; // OC.TYPEDEF, OC.TEMPLATE_TYPE
const C_STRUCT_LIKE: &str = "#b5b6e3"; // OC.STRUCT_LIKE
const C_FIELD: &str = "#9373a5"; // OC.STRUCT_FIELD

fn c_line(line: &str, cpp: bool) -> String {
    let Some(mut toks) = tokens(line) else { return line.to_string() };
    let words: Vec<usize> = (0..toks.len()).filter(|&i| toks[i].is_word()).collect();
    let text = |toks: &Vec<Token>, w: usize| toks[words[w]].text.trim().to_string();

    for w in 0..words.len() {
        let i = words[w];
        let word = text(&toks, w);
        let prev = if w > 0 { text(&toks, w - 1) } else { String::new() };
        let next = if w + 1 < words.len() { text(&toks, w + 1) } else { String::new() };
        let color = toks[i].color.clone().unwrap_or_default();
        let is_ident = word.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
            && word.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
        let all_caps = is_ident && word.len() >= 2 && word.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');

        if word == "NULL" || (all_caps && color == PLAIN && next.starts_with('(')) {
            // NULL and SHOUTING_CASE calls are macros
            toks[i].color = Some(C_MACRO.to_string());
        } else if is_ident && word.ends_with("_t") && (color == ORANGE || color == C_STRUCT_LIKE) {
            toks[i].color = Some(C_TYPEDEF.to_string());
        } else if cpp && color == C_STRUCT_LIKE && word.len() <= 2 && word.starts_with(|c: char| c.is_ascii_uppercase()) && word.chars().skip(1).all(|c| c.is_ascii_digit()) {
            toks[i].color = Some(C_TYPEDEF.to_string());
        } else if is_ident && (prev.ends_with('.') || prev.ends_with("-&gt;")) {
            // s.field / s->field are fields; s.method() calls stay plain
            toks[i].color = Some(if next.starts_with('(') { PLAIN } else { C_FIELD }.to_string());
        }
    }
    toks.iter().map(Token::html).collect()
}

fn c_context(html: &str, cpp: bool) -> String {
    LINE_DIV_RE
        .replace_all(html, |caps: &regex::Captures| format!("{}{}{}", &caps[1], c_line(&caps[2], cpp), &caps[3]))
        .into_owned()
}

fn language_for(lang: &str) -> Language {
    match lang {
        "sh" | "shell" | "zsh" => Language::Bash,
        other => other.parse().unwrap_or(Language::PlainText),
    }
}

fn highlight_as(language: Language, code: &str) -> String {
    let formatter = HtmlInlineBuilder::new()
        .language(language)
        .theme(Some(THEME.clone()))
        .build()
        .expect("building the HTML formatter");
    resolve_placeholders(&highlight(code, formatter))
}

static CODE_INNER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?s)^(.*?<code[^>]*>)(.*)(</code></pre>)\s*$"#).unwrap());

static LINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?s)<div class="l-line" data-line="\d+">(.*?)</div>"#).unwrap());

/// A ```console session: `$ command` lines are highlighted as bash, output stays plain.
fn highlight_console(code: &str) -> String {
    let plain = highlight_as(Language::PlainText, code);
    let caps = CODE_INNER_RE.captures(&plain).expect("unexpected Lumis HTML");
    let (open, close) = (caps[1].to_string(), caps[3].to_string());

    let mut lines = String::new();
    for (i, line) in code.lines().enumerate() {
        let n = i + 1;
        let content = match line.strip_prefix("$ ") {
            Some(command) => {
                let html = highlight_as(Language::Bash, command);
                let inner = LINE_RE.captures(&html).map(|c| c[1].trim_end_matches('\n').to_string());
                format!("{}{}", span("#808080", "$ "), inner.unwrap_or_default())
            }
            None => html_escape(line),
        };
        lines.push_str(&format!("<div class=\"l-line\" data-line=\"{n}\">{content}\n</div>"));
    }
    format!("{open}{lines}{close}")
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Highlights one fenced code block and wraps it with its language label.
pub fn code_block(lang: &str, code: &str) -> String {
    let code = code.strip_suffix('\n').unwrap_or(code);
    let html = match lang {
        "console" => highlight_console(code),
        "python" | "py" => python_names(&highlight_as(Language::Python, code)),
        "rust" | "rs" => rust_context(&highlight_as(Language::Rust, code)),
        "c" | "h" => c_context(&highlight_as(Language::C, code), false),
        "cpp" | "c++" | "cc" | "hpp" => c_context(&highlight_as(Language::CPlusPlus, code), true),
        _ => highlight_as(language_for(lang), code),
    };
    let label = html_escape(lang);
    format!(
        r#"<div class="code-block" data-lang="{label}"><span class="code-lang">{label}</span>{}</div>"#,
        mark_empty_lines(&html)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semicolons_and_commas_are_orange_dots_plain() {
        let html = code_block("c", "int f(int a, int b) { return s.x; }");
        assert!(html.contains(&span(ORANGE, ";")), "{html}");
        assert!(html.contains(&span(ORANGE, ",")), "{html}");
        assert!(html.contains(&span(PLAIN, ".")), "{html}");
        assert!(!html.contains("#a9b7c5"), "{html}");
    }

    #[test]
    fn rust_lifetimes_and_question_mark() {
        let html = code_block("rust", "fn f<'a>(x: &'a str) -> Option<u8> { g()?; None }");
        assert!(html.contains(r#"<span style="color: #20999d; font-style: italic;">&#39;a</span>"#), "{html}");
        assert!(html.contains("font-weight: bold;\">?</span>"), "{html}");
        for placeholder in [DELIMITER, RUST_OPERATOR, RUST_MODIFIER, RUST_ATTRIBUTE] {
            assert!(!html.contains(placeholder), "{html}");
        }
    }

    #[test]
    fn console_highlights_commands_only() {
        let html = code_block("console", "$ echo hi\nhi");
        assert!(html.contains(&span("#808080", "$ ")), "{html}");
        assert!(html.matches("l-line").count() == 2, "{html}");
    }
}

#[cfg(test)]
mod probe {
    #[test]
    #[ignore]
    fn print_samples() {
        for (lang, code) in [
            ("python", "@dataclass\nclass A:\n    def __init__(self, x: int = 1):\n        self.x = len([x])\n        print(f\"{x}\", sep=\"\")\n"),
            ("bash", "sed -i 's/a/b/g' \"$FILE\" | grep -rn x . && echo $HOME"),
            ("c", "// stack.h\n#ifndef STACK_H\n#define STACK_H\n#include <stddef.h>\n#define MAX(a, b) ((a) > (b) ? (a) : (b))\nstruct stack { size_t count; int *data; };\nenum color { RED, GREEN };\n[[nodiscard]] struct stack *stack_new(size_t capacity);\nint f(struct stack *s) { int x = MAX(1, 2); return s->count + NULL + RED; }\n#endif"),
            ("cpp", "#include <vector>\nnamespace ds {\ntemplate <typename T>\nclass Stack {\npublic:\n    void push(const T &value) { items_.push_back(value); this->n++; }\nprivate:\n    std::vector<T> items_;\n    std::size_t n;\n};\n}"),
            ("rust", "#[derive(Debug)]\nstruct Stack<T> { items: Vec<T> }\ntrait Shape { fn area(&self) -> f64; }\nimpl Shape for Stack<u8> {\n    fn area(&self) -> f64 { self.items.len() as f64 }\n}\nimpl<T> Stack<T> {\n    pub fn new() -> Self { Stack { items: Vec::new() } }\n}\nfn f(s: &dyn Shape) -> impl Iterator<Item = u32> { let mut v = vec![1]; v.push(2); std::iter::empty() }\nconst MAX: usize = 3;\nmatch x { Some(y) => y, None => 0 }"),
        ] {
            println!("== {lang}\n{}", super::code_block(lang, code));
        }
    }
}
