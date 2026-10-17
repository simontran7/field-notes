use std::collections::HashMap;
use std::sync::LazyLock;

use lumis::languages::Language;
use lumis::themes::{self, Theme};
use lumis::{HtmlInlineBuilder, highlight};
use regex::Regex;
use serde_json::Value;

const TEXT: &str = "TEXT";

const CAPTURES: &[(&str, &str)] = &[
    ("comment", "DEFAULT_LINE_COMMENT"),
    ("comment.documentation", "DEFAULT_DOC_COMMENT"),
    ("keyword", "DEFAULT_KEYWORD"),
    ("boolean", "DEFAULT_KEYWORD"),
    ("constant.builtin", "DEFAULT_KEYWORD"),
    ("type.builtin", "DEFAULT_KEYWORD"),
    ("variable.builtin", "DEFAULT_KEYWORD"),
    ("number", "DEFAULT_NUMBER"),
    ("string", "DEFAULT_STRING"),
    ("character", "DEFAULT_STRING"),
    ("string.escape", "DEFAULT_VALID_STRING_ESCAPE"),
    ("escape", "DEFAULT_VALID_STRING_ESCAPE"),
    ("function", "DEFAULT_FUNCTION_DECLARATION"),
    ("function.method", "DEFAULT_FUNCTION_DECLARATION"),
    ("function.call", "DEFAULT_FUNCTION_CALL"),
    ("function.method.call", "DEFAULT_FUNCTION_CALL"),
    ("function.builtin", "DEFAULT_FUNCTION_CALL"),
    ("function.macro", "DEFAULT_FUNCTION_CALL"),
    ("property", "DEFAULT_INSTANCE_FIELD"),
    ("variable.member", "DEFAULT_INSTANCE_FIELD"),
    ("constant", "DEFAULT_CONSTANT"),
    ("constant.macro", "DEFAULT_CONSTANT"),
    ("attribute", "DEFAULT_METADATA"),
    ("type", "DEFAULT_CLASS_NAME"),
    ("type.definition", "DEFAULT_CLASS_NAME"),
    ("constructor", "DEFAULT_IDENTIFIER"),
    ("variable", "DEFAULT_IDENTIFIER"),
    ("variable.parameter", "DEFAULT_PARAMETER"),
    ("module", "DEFAULT_IDENTIFIER"),
    ("label", "DEFAULT_LABEL"),
    ("operator", "DEFAULT_OPERATION_SIGN"),
    ("punctuation", "DEFAULT_OPERATION_SIGN"),
    ("punctuation.bracket", "DEFAULT_PARENTHS"),
    ("punctuation.delimiter", "@delimiter"),
    ("comment.documentation.rust", "org.rust.DOC_COMMENT"),
    ("keyword.rust", "org.rust.KEYWORD"),
    ("function.rust", "org.rust.FUNCTION"),
    ("function.method.rust", "org.rust.METHOD"),
    ("function.call.rust", "org.rust.FUNCTION_CALL"),
    ("function.method.call.rust", "org.rust.METHOD_CALL"),
    ("function.macro.rust", "org.rust.MACRO"),
    ("type.rust", "org.rust.STRUCT"),
    ("type.builtin.rust", "org.rust.PRIMITIVE_TYPE"),
    ("constructor.rust", "org.rust.ENUM_VARIANT"),
    ("constant.builtin.rust", "org.rust.ENUM_VARIANT"),
    ("constant.rust", "org.rust.CONSTANT"),
    ("variable.builtin.rust", "org.rust.SELF_PARAMETER"),
    ("variable.member.rust", "org.rust.FIELD"),
    ("property.rust", "org.rust.FIELD"),
    ("variable.parameter.rust", "org.rust.PARAMETER"),
    ("module.rust", "org.rust.MODULE"),
    ("keyword.modifier.rust", "@rust_modifier"),
    ("attribute.rust", "@rust_attribute"),
    ("attribute.builtin.rust", "@rust_attribute"),
    ("operator.rust", "@rust_operator"),
    ("function.python", "PY.FUNC_DEFINITION"),
    ("function.method.python", "PY.FUNC_DEFINITION"),
    ("function.call.python", "PY.FUNCTION_CALL"),
    ("function.method.call.python", "PY.METHOD_CALL"),
    ("function.builtin.python", "PY.BUILTIN_NAME"),
    ("type.builtin.python", "PY.BUILTIN_NAME"),
    ("variable.builtin.python", "PY.SELF_PARAMETER"),
    ("type.python", "DEFAULT_IDENTIFIER"),
    ("constant.python", "DEFAULT_IDENTIFIER"),
    ("variable.member.python", "DEFAULT_IDENTIFIER"),
    ("property.python", "DEFAULT_IDENTIFIER"),
    ("punctuation.special.python", "PY.FSTRING_FRAGMENT_BRACES"),
    ("keyword.bash", "BASH.KEYWORD"),
    ("string.bash", "BASH.STRING"),
    ("comment.bash", "BASH.LINE_COMMENT"),
    ("function.call.bash", "BASH.EXTERNAL_COMMAND"),
    ("function.builtin.bash", "BASH.EXTERNAL_COMMAND"),
    ("variable.bash", "BASH.VAR_USE"),
    ("variable.builtin.bash", "BASH.VAR_USE"),
    ("constant.bash", "BASH.VAR_USE"),
    ("constant.builtin.bash", "BASH.VAR_USE"),
    ("property.bash", "BASH.VAR_USE"),
    ("variable.member.bash", "BASH.VAR_USE"),
];

const C_CAPTURES: &[(&str, &str)] = &[
    ("keyword", "OC.KEYWORD"),
    ("keyword.directive", "OC.DIRECTIVE"),
    ("keyword.directive.define", "OC.DIRECTIVE"),
    ("keyword.import", "OC.DIRECTIVE"),
    ("keyword.conditional.ternary", "OC.OPERATION_SIGN"),
    ("comment.documentation", "Doxygen Comment"),
    ("string", "OC.STRING"),
    ("type.builtin", "OC.BUILTIN_TYPE_KEYWORD"),
    ("variable.builtin", "OC.SELFSUPERTHIS"),
    ("constant.macro", "OC.MACRONAME"),
    ("function.macro", "OC.MACRONAME"),
    ("type", "OC.STRUCT_LIKE"),
    ("type.definition", "OC.TYPEDEF"),
    ("module", "OC.NAMESPACE_LIKE"),
    ("variable.member", "OC.STRUCT_FIELD"),
    ("property", "OC.STRUCT_FIELD"),
    ("attribute", "DEFAULT_IDENTIFIER"),
    ("function", "OC.FUNCTION_DECLARATION"),
    ("function.call", "OC.FUNCTION"),
    ("function.method.call", "OC.FUNCTION"),
    ("constant", "OC.ENUM_CONST"),
];

struct Scheme {
    name: &'static str,
    styles: HashMap<String, String>,
    pre_style: String,
}

struct Highlighter {
    theme: Theme,
    keys: HashMap<String, String>,
    schemes: Vec<Scheme>,
}

fn css(style: &Value, text_fg: &str) -> String {
    let fg = style.get("fg").and_then(Value::as_str).unwrap_or(text_fg);
    let mut css = format!("color: {fg};");
    if style.get("bold").and_then(Value::as_bool) == Some(true) {
        css.push_str(" font-weight: bold;");
    }
    if style.get("italic").and_then(Value::as_bool) == Some(true) {
        css.push_str(" font-style: italic;");
    }
    if let Some(color) = style.get("underline").and_then(Value::as_str) {
        css.push_str(&format!(" text-decoration: underline {color};"));
    }
    css
}

static HIGHLIGHTER: LazyLock<Highlighter> = LazyLock::new(|| {
    let mut entries: Vec<(String, &str)> =
        CAPTURES.iter().map(|(c, k)| (c.to_string(), *k)).collect();
    for (capture, key) in C_CAPTURES {
        entries.push((format!("{capture}.c"), key));
    }

    let mut placeholder_of: HashMap<String, String> = HashMap::new();
    let mut placeholder = |key: &str| {
        let n = placeholder_of.len();
        placeholder_of
            .entry(key.to_string())
            .or_insert_with(|| format!("#a0{n:04x}"))
            .clone()
    };
    let mut highlights = serde_json::Map::new();
    highlights.insert(
        "normal".into(),
        serde_json::json!({ "fg": placeholder(TEXT), "bg": "#000000" }),
    );
    for (capture, key) in &entries {
        highlights.insert(
            capture.clone(),
            serde_json::json!({ "fg": placeholder(key) }),
        );
    }
    let theme_json = serde_json::json!({
        "name": "intellij-keys", "appearance": "dark", "revision": "1", "highlights": highlights
    });
    let theme = themes::from_json(&theme_json.to_string()).expect("building the key theme");
    let keys = placeholder_of
        .into_iter()
        .map(|(key, color)| (color, key))
        .collect();

    let resolved: Value = serde_json::from_str(include_str!("../../theme/intellij.json"))
        .expect("theme/intellij.json is invalid");
    let schemes = ["dark", "light"]
        .into_iter()
        .map(|name| {
            let scheme = &resolved["schemes"][name];
            let text_fg = scheme["TEXT"]["fg"].as_str().unwrap();
            let bg = scheme["TEXT"]["bg"].as_str().unwrap();
            let styles = scheme
                .as_object()
                .unwrap()
                .iter()
                .map(|(key, style)| (key.clone(), css(style, text_fg)))
                .collect();
            Scheme {
                name,
                styles,
                pre_style: format!("color: {text_fg}; background-color: {bg};"),
            }
        })
        .collect();

    Highlighter {
        theme,
        keys,
        schemes,
    }
});

#[derive(Clone)]
struct Token {
    key: String,
    text: String,
}

impl Token {
    fn trimmed(&self) -> &str {
        self.text.trim()
    }

    fn is_word(&self) -> bool {
        !self.trimmed().is_empty()
    }
}

static TAG_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<span style="color: (#[0-9a-f]{6});[^"]*">|</span>"#).unwrap());

fn tokenize(line: &str) -> Vec<Token> {
    let keys = &HIGHLIGHTER.keys;
    let mut out = Vec::new();
    let mut stack: Vec<&str> = Vec::new();
    let mut last = 0;
    let push = |out: &mut Vec<Token>, text: &str, stack: &[&str]| {
        if !text.is_empty() {
            let key = stack
                .last()
                .and_then(|c| keys.get(*c))
                .map_or(TEXT, String::as_str);
            out.push(Token {
                key: key.to_string(),
                text: text.to_string(),
            });
        }
    };
    for caps in TAG_RE.captures_iter(line) {
        let m = caps.get(0).unwrap();
        push(&mut out, &line[last..m.start()], &stack);
        match caps.get(1) {
            Some(color) => stack.push(color.as_str()),
            None => {
                stack.pop();
            }
        }
        last = m.end();
    }
    push(&mut out, &line[last..], &stack);
    out
}

fn words(tokens: &[Token]) -> Vec<usize> {
    (0..tokens.len()).filter(|&i| tokens[i].is_word()).collect()
}

fn is_ident(word: &str) -> bool {
    word.chars()
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && word.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn is_type_parameter_name(word: &str) -> bool {
    word.len() <= 2
        && word.starts_with(|c: char| c.is_ascii_uppercase())
        && word.chars().skip(1).all(|c| c.is_ascii_digit())
}

fn split_delimiters(tokens: &mut [Token]) {
    for t in tokens.iter_mut().filter(|t| t.key == "@delimiter") {
        t.key = match t.trimmed() {
            ";" => "DEFAULT_SEMICOLON",
            "," => "DEFAULT_COMMA",
            "." => "DEFAULT_DOT",
            _ => "DEFAULT_OPERATION_SIGN",
        }
        .to_string();
    }
}

fn rust_rules(tokens: &mut Vec<Token>) {
    let mut i = 0;
    while i + 1 < tokens.len() {
        if tokens[i].key == "@rust_modifier"
            && tokens[i].text == "&#39;"
            && tokens[i + 1].key == "@rust_attribute"
        {
            let name = tokens.remove(i + 1).text;
            tokens[i] = Token {
                key: "org.rust.LIFETIME".into(),
                text: format!("&#39;{name}"),
            };
        }
        i += 1;
    }
    for t in tokens.iter_mut() {
        let key = match (t.key.as_str(), t.trimmed()) {
            ("@rust_modifier", _) => "org.rust.KEYWORD",
            ("@rust_attribute", _) => "org.rust.ATTRIBUTE",
            ("@rust_operator", "?") => "org.rust.Q_OPERATOR",
            ("@rust_operator", _) => "DEFAULT_OPERATION_SIGN",
            _ => continue,
        };
        t.key = key.into();
    }

    let w = words(tokens);
    let text = |tokens: &[Token], n: usize| tokens[w[n]].trimmed().to_string();

    if w.len() >= 2 && text(tokens, 0) == "#" && text(tokens, 1).starts_with('[') {
        for t in tokens
            .iter_mut()
            .filter(|t| t.is_word() && t.key != "DEFAULT_STRING")
        {
            t.key = "org.rust.ATTRIBUTE".into();
        }
        return;
    }

    for n in 0..w.len() {
        let i = w[n];
        let word = text(tokens, n);
        let prev = if n > 0 {
            text(tokens, n - 1)
        } else {
            String::new()
        };
        let is_type = tokens[i].key == "org.rust.STRUCT";
        let for_follows = w[n + 1..].iter().any(|&j| tokens[j].trimmed() == "for");
        let after_impl = prev == "impl"
            || (prev == "&gt;" && w[..n].iter().any(|&j| tokens[j].trimmed() == "impl"));

        let key = if word.len() > 1 && word.ends_with('!') && is_ident(&word[..word.len() - 1]) {
            Some("org.rust.MACRO")
        } else if is_type && is_type_parameter_name(&word) {
            Some("org.rust.TYPE_PARAMETER")
        } else if is_type
            && (prev == "trait"
                || prev == "dyn"
                || (after_impl && for_follows)
                || (prev == "impl" && n > 1))
        {
            Some("org.rust.TRAIT")
        } else if matches!(word.as_str(), "std" | "core" | "alloc" | "crate")
            && n + 1 < w.len()
            && text(tokens, n + 1).starts_with("::")
        {
            Some("org.rust.CRATE")
        } else {
            None
        };
        if let Some(key) = key {
            tokens[i].key = key.into();
        }
    }
}

fn c_rules(tokens: &mut [Token]) {
    let w = words(tokens);
    let text = |tokens: &[Token], n: usize| tokens[w[n]].trimmed().to_string();
    for n in 0..w.len() {
        let i = w[n];
        let word = text(tokens, n);
        let prev = if n > 0 {
            text(tokens, n - 1)
        } else {
            String::new()
        };
        let next = if n + 1 < w.len() {
            text(tokens, n + 1)
        } else {
            String::new()
        };
        let key = tokens[i].key.as_str();
        let ident = is_ident(&word);
        let all_caps = ident
            && word.len() >= 2
            && word
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');

        let new_key = if word == "NULL"
            || (all_caps && next.starts_with('(') && key != "OC.FUNCTION_DECLARATION")
        {
            Some("OC.MACRONAME")
        } else if ident
            && word.ends_with("_t")
            && matches!(key, "OC.BUILTIN_TYPE_KEYWORD" | "OC.STRUCT_LIKE")
        {
            Some("OC.TYPEDEF")
        } else if ident && (prev.ends_with('.') || prev.ends_with("-&gt;")) {
            Some(if next.starts_with('(') {
                "OC.FUNCTION"
            } else {
                "OC.STRUCT_FIELD"
            })
        } else {
            None
        };
        if let Some(k) = new_key {
            tokens[i].key = k.into();
        }
    }
}

const PYTHON_BUILTINS: &[&str] = &[
    "abs",
    "all",
    "any",
    "ascii",
    "bin",
    "bool",
    "breakpoint",
    "bytearray",
    "bytes",
    "callable",
    "chr",
    "classmethod",
    "compile",
    "complex",
    "delattr",
    "dict",
    "dir",
    "divmod",
    "enumerate",
    "eval",
    "exec",
    "filter",
    "float",
    "format",
    "frozenset",
    "getattr",
    "globals",
    "hasattr",
    "hash",
    "help",
    "hex",
    "id",
    "input",
    "int",
    "isinstance",
    "issubclass",
    "iter",
    "len",
    "list",
    "locals",
    "map",
    "max",
    "memoryview",
    "min",
    "next",
    "object",
    "oct",
    "open",
    "ord",
    "pow",
    "print",
    "property",
    "range",
    "repr",
    "reversed",
    "round",
    "set",
    "setattr",
    "slice",
    "sorted",
    "staticmethod",
    "str",
    "sum",
    "super",
    "tuple",
    "type",
    "vars",
    "zip",
];

fn python_rules(tokens: &mut [Token]) {
    let w = words(tokens);
    if w.is_empty() {
        return;
    }
    if tokens[w[0]].trimmed().starts_with('@') {
        for &i in &w {
            if tokens[i].trimmed().starts_with('(') {
                break;
            }
            tokens[i].key = "PY.DECORATOR".into();
        }
        return;
    }
    for n in 0..w.len() {
        let i = w[n];
        let word = tokens[i].trimmed().to_string();
        let prev = if n > 0 {
            tokens[w[n - 1]].trimmed().to_string()
        } else {
            String::new()
        };
        let is_dunder =
            word.len() > 4 && word.starts_with("__") && word.ends_with("__") && is_ident(&word);
        let new_key = match word.as_str() {
            "self" | "cls" => Some("PY.SELF_PARAMETER"),
            _ if is_dunder && prev == "def" => Some("PY.PREDEFINED_DEFINITION"),
            _ if is_dunder => Some("PY.PREDEFINED_USAGE"),
            _ if prev == "class" && is_ident(&word) => Some("PY.CLASS_DEFINITION"),
            _ if PYTHON_BUILTINS.contains(&word.as_str()) && prev != "." && prev != "def" => {
                Some("PY.BUILTIN_NAME")
            }
            _ => None,
        };
        if let Some(k) = new_key {
            tokens[i].key = k.into();
        }
    }
}

static LINE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?s)<div class="l-line" data-line="\d+">(.*?)\n?</div>"#).unwrap()
});

fn tokenize_code(language: Language, code: &str) -> Vec<Vec<Token>> {
    let formatter = HtmlInlineBuilder::new()
        .language(language)
        .theme(Some(HIGHLIGHTER.theme.clone()))
        .build()
        .expect("building the HTML formatter");
    let html = highlight(code, formatter);
    LINE_RE
        .captures_iter(&html)
        .map(|caps| tokenize(&caps[1]))
        .collect()
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn language_for(lang: &str) -> Language {
    match lang {
        "sh" | "shell" | "zsh" => Language::Bash,
        "py" => Language::Python,
        "rs" => Language::Rust,
        "h" => Language::C,
        other => other.parse().unwrap_or(Language::PlainText),
    }
}

static PLACEHOLDER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"&lt;([^&<>]+)&gt;(\?|\+,?)?").unwrap());

static DEFINITION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^&lt;([^&<>]+)&gt;:(.*)$").unwrap());

fn syntax_placeholders(text: &str) -> String {
    PLACEHOLDER_RE
        .replace_all(text, |caps: &regex::Captures| {
            let count = match caps.get(2).map(|m| m.as_str()) {
                Some("?") => r#"<sub class="syntax-count">opt</sub>"#,
                Some("+") => r#"<sup class="syntax-count">+</sup>"#,
                Some("+,") => r#"<sup class="syntax-count">+,</sup>"#,
                _ => "",
            };
            format!(r#"<em class="syntax-placeholder">{}</em>{count}"#, &caps[1])
        })
        .into_owned()
}

/// One line of a `syntax` block: each `<placeholder>` is shown in italics without its brackets,
/// `<placeholder>?` also gets a subscript "opt" to mark it optional, `<placeholder>+` a
/// superscript "+" to mark one or more, and `<placeholder>+,` a superscript "+," to mark one or
/// more separated by commas. A line that starts with
/// `<placeholder>:` begins that placeholder's definition, and the colon and any note after it
/// (such as "one or more") are notation. Everything else is literal.
fn syntax_line(line: &str) -> Token {
    let escaped = html_escape(line);
    // `<scope>: <summary>` is a template line, not a definition of `scope`
    let definition = DEFINITION_RE
        .captures(&escaped)
        .filter(|caps| !PLACEHOLDER_RE.is_match(&caps[2]));
    let text = match definition {
        Some(caps) => format!(
            r#"<em class="syntax-placeholder">{}</em><span class="syntax-meta">:{}</span>"#,
            &caps[1], &caps[2]
        ),
        None => syntax_placeholders(&escaped),
    };
    Token {
        key: TEXT.into(),
        text,
    }
}

fn lines_for(lang: &str, code: &str) -> Vec<Vec<Token>> {
    if lang == "syntax" {
        return code.lines().map(|line| vec![syntax_line(line)]).collect();
    }

    if lang == "console" {
        return code
            .lines()
            .map(|line| match line.strip_prefix("$ ") {
                Some(command) => {
                    let mut tokens = vec![Token {
                        key: TEXT.into(),
                        text: r#"<span class="console-prompt">$ </span>"#.into(),
                    }];
                    tokens.extend(
                        lines_for("bash", command)
                            .into_iter()
                            .next()
                            .unwrap_or_default(),
                    );
                    tokens
                }
                None => vec![Token {
                    key: TEXT.into(),
                    text: html_escape(line),
                }],
            })
            .collect();
    }

    let language = language_for(lang);
    let mut lines = tokenize_code(language, code);
    for tokens in &mut lines {
        split_delimiters(tokens);
        match language {
            Language::Rust => rust_rules(tokens),
            Language::C => c_rules(tokens),
            Language::Python => python_rules(tokens),
            _ => {}
        }
    }
    lines
}

fn render(lines: &[Vec<Token>], lang: &str, scheme: &Scheme) -> String {
    let mut html = format!(
        r#"<pre class="code-{}" style="{}"><code class="language-{lang}">"#,
        scheme.name, scheme.pre_style
    );
    for (n, tokens) in lines.iter().enumerate() {
        let empty = if tokens.iter().all(|t| !t.is_word()) {
            " l-empty"
        } else {
            ""
        };
        html.push_str(&format!(
            r#"<div class="l-line{empty}" data-line="{}">"#,
            n + 1
        ));
        for t in tokens {
            match scheme.styles.get(&t.key) {
                Some(style) if t.key != TEXT => {
                    html.push_str(&format!(r#"<span style="{style}">{}</span>"#, t.text))
                }
                _ => html.push_str(&t.text),
            }
        }
        html.push_str("\n</div>");
    }
    html.push_str("</code></pre>");
    html
}

pub fn code_block(lang: &str, code: &str) -> String {
    let code = code.strip_suffix('\n').unwrap_or(code);
    let lines = lines_for(lang, code);
    let label = html_escape(lang);
    let variants: String = HIGHLIGHTER
        .schemes
        .iter()
        .map(|scheme| render(&lines, &label, scheme))
        .collect();
    format!(
        r#"<div class="code-block" data-lang="{label}"><span class="code-lang">{label}</span>{variants}</div>"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key_of(lang: &str, code: &str, word: &str) -> String {
        lines_for(lang, code)
            .into_iter()
            .flatten()
            .find(|t| t.trimmed() == word)
            .map(|t| t.key)
            .unwrap_or_else(|| panic!("{word} not found in {code}"))
    }

    #[test]
    fn syntax_placeholders_become_italic() {
        let line = syntax_line("<type> <identifier>[<capacity>] <initializer>?;");
        assert_eq!(
            line.text,
            r#"<em class="syntax-placeholder">type</em> <em class="syntax-placeholder">identifier</em>[<em class="syntax-placeholder">capacity</em>] <em class="syntax-placeholder">initializer</em><sub class="syntax-count">opt</sub>;"#
        );
        // Literal brackets around a placeholder stay, and a spaced ? is literal
        assert_eq!(
            syntax_line("#include <<header>>").text,
            r#"#include &lt;<em class="syntax-placeholder">header</em>&gt;"#
        );
        assert_eq!(
            syntax_line("<values>: comma-separated").text,
            r#"<em class="syntax-placeholder">values</em><span class="syntax-meta">: comma-separated</span>"#
        );
        // A ? or + after a closing paren is literal, as in a regex quantifier
        assert_eq!(
            syntax_line("(?:<pattern>)?").text,
            r#"(?:<em class="syntax-placeholder">pattern</em>)?"#
        );
        assert_eq!(
            syntax_line("<scope>: <what changed>").text,
            r#"<em class="syntax-placeholder">scope</em>: <em class="syntax-placeholder">what changed</em>"#
        );
        assert_eq!(
            syntax_line("{ <case>+ }").text,
            r#"{ <em class="syntax-placeholder">case</em><sup class="syntax-count">+</sup> }"#
        );
        assert_eq!(
            syntax_line("    case <constant>: <statements>").text,
            r#"    case <em class="syntax-placeholder">constant</em>: <em class="syntax-placeholder">statements</em>"#
        );
        assert_eq!(
            syntax_line("{ <value>+, }").text,
            r#"{ <em class="syntax-placeholder">value</em><sup class="syntax-count">+,</sup> }"#
        );
        assert_eq!(
            syntax_line("<a> ? <b> : <c>").text,
            r#"<em class="syntax-placeholder">a</em> ? <em class="syntax-placeholder">b</em> : <em class="syntax-placeholder">c</em>"#
        );
    }

    #[test]
    fn delimiters_get_their_own_keys() {
        let code = "int f(int a, int b) { return s.x; }";
        assert_eq!(key_of("c", code, ";"), "DEFAULT_SEMICOLON");
        assert_eq!(key_of("c", code, ","), "DEFAULT_COMMA");
    }

    #[test]
    fn rust_context() {
        let code = "#[derive(Debug)]\nimpl<T> Shape for Stack<T> { fn f<'a>(&self) -> Option<u8> { g()?; vec![1]; None } }\nfn h(s: &dyn Shape) -> impl Iterator<Item = u8> { std::iter::empty() }";
        assert_eq!(key_of("rust", code, "derive"), "org.rust.ATTRIBUTE");
        assert_eq!(key_of("rust", code, "Shape"), "org.rust.TRAIT");
        assert_eq!(key_of("rust", code, "Stack"), "org.rust.STRUCT");
        assert_eq!(key_of("rust", code, "T"), "org.rust.TYPE_PARAMETER");
        assert_eq!(key_of("rust", code, "&#39;a"), "org.rust.LIFETIME");
        assert_eq!(key_of("rust", code, "?"), "org.rust.Q_OPERATOR");
        assert_eq!(key_of("rust", code, "vec!"), "org.rust.MACRO");
        assert_eq!(key_of("rust", code, "None"), "org.rust.ENUM_VARIANT");
        assert_eq!(key_of("rust", code, "Iterator"), "org.rust.TRAIT");
        assert_eq!(key_of("rust", code, "std"), "org.rust.CRATE");
    }

    #[test]
    fn c_context() {
        let code = "#define MAX(a, b) a\nstruct stack { size_t count; };\nint f(struct stack *s) { return MAX(1, 2) + s->count + NULL; }";
        assert_eq!(key_of("c", code, "#define"), "OC.DIRECTIVE");
        assert_eq!(key_of("c", code, "size_t"), "OC.TYPEDEF");
        assert_eq!(key_of("c", code, "stack"), "OC.STRUCT_LIKE");
        assert_eq!(key_of("c", code, "NULL"), "OC.MACRONAME");
        assert_eq!(key_of("c", code, "count"), "OC.STRUCT_FIELD");
    }

    #[test]
    fn python_context() {
        let code = "@dataclass\nclass A:\n    def __init__(self, x: int):\n        print(len(x))";
        assert_eq!(key_of("python", code, "dataclass"), "PY.DECORATOR");
        assert_eq!(key_of("python", code, "A"), "PY.CLASS_DEFINITION");
        assert_eq!(
            key_of("python", code, "__init__"),
            "PY.PREDEFINED_DEFINITION"
        );
        assert_eq!(key_of("python", code, "self"), "PY.SELF_PARAMETER");
        assert_eq!(key_of("python", code, "int"), "PY.BUILTIN_NAME");
        assert_eq!(key_of("python", code, "print"), "PY.BUILTIN_NAME");
    }

    #[test]
    fn bash_commands_and_console() {
        assert_eq!(
            key_of("bash", "sed -i x \"$FILE\"", "sed"),
            "BASH.EXTERNAL_COMMAND"
        );
        let html = code_block("console", "$ echo hi\nhi");
        assert_eq!(html.matches("class=\"l-line").count(), 4, "{html}");
        assert_eq!(html.matches("class=\"console-prompt\"").count(), 2, "{html}");
        assert_eq!(
            key_of("console", "$ sed -i x\nhi", "sed"),
            "BASH.EXTERNAL_COMMAND"
        );
    }

    #[test]
    fn every_key_has_a_style_in_both_schemes() {
        for scheme in &HIGHLIGHTER.schemes {
            for key in HIGHLIGHTER.keys.values().filter(|k| !k.starts_with('@')) {
                assert!(
                    scheme.styles.contains_key(key),
                    "{} is missing {key}",
                    scheme.name
                );
            }
        }
    }
}
