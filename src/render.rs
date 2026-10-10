use std::sync::LazyLock;

use comrak::{markdown_to_html, Options};
use regex::Regex;

use crate::notes::LinkMap;

/// Callout icons: GitHub's Octicons (primer/octicons) for GitHub's five alert types,
/// matching the Obsidian vault's callouts.css, plus a book for definitions and
/// Lucide's square-function for formulas.
fn callout_icon(callout_type: &str) -> &'static str {
    match callout_type {
        "note" => r#"<svg fill="currentColor" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8Zm8-6.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13ZM6.5 7.75A.75.75 0 0 1 7.25 7h1a.75.75 0 0 1 .75.75v2.75h.25a.75.75 0 0 1 0 1.5h-2a.75.75 0 0 1 0-1.5h.25v-2h-.25a.75.75 0 0 1-.75-.75ZM8 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"/></svg>"#,
        "tip" => r#"<svg fill="currentColor" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M8 1.5c-2.363 0-4 1.69-4 3.75 0 .984.424 1.625.984 2.304l.214.253c.223.264.47.556.673.848.284.411.537.896.621 1.49a.75.75 0 0 1-1.484.211c-.04-.282-.163-.547-.37-.847a8.456 8.456 0 0 0-.542-.68c-.084-.1-.173-.205-.268-.32C3.201 7.75 2.5 6.766 2.5 5.250 2.5 2.31 4.863 0 8 0s5.5 2.31 5.5 5.25c0 1.516-.701 2.5-1.328 3.259-.095.115-.184.22-.268.319-.207.245-.383.453-.541.681-.208.3-.33.565-.37.847a.751.751 0 0 1-1.485-.212c.084-.593.337-1.078.621-1.489.203-.292.45-.584.673-.848.075-.088.147-.173.213-.253.561-.679.985-1.32.985-2.304 0-2.06-1.637-3.750-4-3.75ZM5.75 12h4.5a.75.75 0 0 1 0 1.5h-4.5a.75.75 0 0 1 0-1.5ZM6 15.25a.75.75 0 0 1 .75-.75h2.5a.75.75 0 0 1 0 1.5h-2.5a.75.75 0 0 1-.75-.75Z"/></svg>"#,
        "important" => r#"<svg fill="currentColor" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v9.5A1.75 1.75 0 0 1 14.25 13H8.06l-2.573 2.573A1.458 1.458 0 0 1 3 14.543V13H1.75A1.75 1.75 0 0 1 0 11.25Zm1.75-.25a.25.25 0 0 0-.25.25v9.5c0 .138.112.25.25.25h2a.75.75 0 0 1 .75.75v2.19l2.72-2.72a.749.749 0 0 1 .53-.22h6.5a.25.25 0 0 0 .25-.25v-9.5a.25.25 0 0 0-.25-.25Zm7 2.25v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 9a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"/></svg>"#,
        "warning" => r#"<svg fill="currentColor" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0 1 14.082 15H1.918a1.75 1.75 0 0 1-1.543-2.575Zm1.763.707a.25.25 0 0 0-.44 0L1.698 13.132a.25.25 0 0 0 .22.368h12.164a.25.25 0 0 0 .22-.368Zm.53 3.996v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"/></svg>"#,
        "caution" => r#"<svg fill="currentColor" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M4.47.22A.749.749 0 0 1 5 0h6c.199 0 .389.079.53.22l4.25 4.25c.141.14.22.331.22.53v6a.749.749 0 0 1-.22.53l-4.25 4.25A.749.749 0 0 1 11 16H5a.749.749 0 0 1-.53-.22L.22 11.53A.749.749 0 0 1 0 11V5c0-.199.079-.389.22-.53Zm.84 1.28L1.5 5.31v5.38l3.81 3.81h5.38l3.81-3.81V5.31L10.69 1.5ZM8 4a.75.75 0 0 1 .75.75v3.5a.75.75 0 0 1-1.5 0v-3.5A.75.75 0 0 1 8 4Zm0 8a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"/></svg>"#,
        "definition" => r#"<svg fill="currentColor" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M0 1.75A.75.75 0 0 1 .75 1h4.253c1.227 0 2.317.59 3 1.501A3.743 3.743 0 0 1 11.006 1h4.245a.75.75 0 0 1 .75.75v10.5a.75.75 0 0 1-.75.75h-4.507a2.25 2.25 0 0 0-1.591.659l-.622.621a.75.75 0 0 1-1.06 0l-.622-.621A2.25 2.25 0 0 0 5.258 13H.75a.75.75 0 0 1-.75-.75Zm7.251 10.324.004-5.073-.002-2.253A2.25 2.25 0 0 0 5.003 2.5H1.5v9h3.757a3.75 3.75 0 0 1 1.994.574ZM8.755 4.75l-.004 7.322a3.752 3.752 0 0 1 1.992-.572H14.5v-9h-3.495a2.25 2.25 0 0 0-2.25 2.25Z"/></svg>"#,
        "formula" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><path d="M9 17c2 0 2.8-1 2.8-2.8V10c0-2 1-3.3 3.2-3"/><path d="M9 11.2h5.7"/></svg>"#,
        _ => unreachable!(),
    }
}

const GITHUB_TYPES: [&str; 5] = ["note", "tip", "important", "warning", "caution"];
const CUSTOM_TYPES: [&str; 2] = ["definition", "formula"];

/// Marks a custom callout so it survives comrak, which only knows GitHub's five types.
const CUSTOM_MARK: &str = "@@callout:";

static CALLOUT_START_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^((?:>[ \t]*)+)\[!(\w+)\][+-]?[ \t]*(.*)$").unwrap());

/// Rewrites Obsidian callout headers into GitHub alerts comrak can parse:
/// `> [!tip]` -> `> [!TIP]`, and custom types like `> [!formula] Arithmetic series`
/// -> `> [!NOTE] @@callout:formula@@Arithmetic series`, restored after rendering.
fn prepare_callouts(markdown: &str) -> String {
    CALLOUT_START_RE
        .replace_all(markdown, |caps: &regex::Captures| {
            let prefix = &caps[1];
            let kind = caps[2].to_lowercase();
            let title = caps[3].trim();
            if CUSTOM_TYPES.contains(&kind.as_str()) {
                format!("{prefix}[!NOTE] {CUSTOM_MARK}{kind}@@{title}")
            } else if GITHUB_TYPES.contains(&kind.as_str()) {
                let upper = kind.to_uppercase();
                if title.is_empty() { format!("{prefix}[!{upper}]") } else { format!("{prefix}[!{upper}] {title}") }
            } else {
                caps[0].to_string()
            }
        })
        .into_owned()
}

static ALERT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?s)<div class="markdown-alert markdown-alert-(note|tip|important|warning|caution)">\s*<p class="markdown-alert-title">(.*?)</p>\s*(.*?)\s*</div>"#,
    )
    .unwrap()
});

static TITLE_MATH_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\$([^$]+)\$").unwrap());

/// comrak doesn't parse `$...$` inside alert titles, so mark it for KaTeX here.
fn title_math(label: &str) -> String {
    TITLE_MATH_RE.replace_all(label, r#"<span data-math-style="inline">$1</span>"#).into_owned()
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    chars.next().map(|c| c.to_uppercase().chain(chars).collect()).unwrap_or_default()
}

fn process_callouts(html: &str) -> String {
    ALERT_RE
        .replace_all(html, |caps: &regex::Captures| {
            let mut kind = caps[1].to_string();
            let mut label = caps[2].to_string();
            if let Some(rest) = label.strip_prefix(CUSTOM_MARK) {
                let (custom, title) = rest.split_once("@@").unwrap_or((rest, ""));
                kind = custom.to_string();
                label = if title.is_empty() { capitalize(custom) } else { title.to_string() };
            }
            let label = title_math(&label);
            let body = &caps[3];
            format!(
                r#"<div class="callout callout-{kind}"><div class="callout-title">{icon}<span>{label}</span></div><div class="callout-body">{body}</div></div>"#,
                icon = callout_icon(&kind),
            )
        })
        .into_owned()
}

static ANCHOR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<a[^>]*\bclass="anchor"[^>]*></a>"#).unwrap());

fn strip_anchors(html: &str) -> String {
    ANCHOR_RE.replace_all(html, "").into_owned()
}

static HREF_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"href="([^"]*)""#).unwrap());

/// Turns note links into site URLs: `list.md`, `../x/list.md#heading` -> `{base}/<slug>#heading`.
/// Links to notes that don't exist (or empty links like `[Virtual Memory]()`) become plain text spans.
fn fix_links(html: &str, links: &LinkMap) -> String {
    let base = crate::layout::base_path();
    HREF_RE
        .replace_all(html, |caps: &regex::Captures| {
            let href = &caps[1];
            if href.starts_with("http") || href.starts_with('#') || href.starts_with("mailto:") {
                return caps[0].to_string();
            }
            let (path, fragment) = href.split_once('#').map_or((href, ""), |(p, f)| (p, f));
            if path.ends_with(".md") {
                if let Some(slug) = links.resolve(path) {
                    let fragment = if fragment.is_empty() { String::new() } else { format!("#{fragment}") };
                    return format!(r#"href="{base}/{slug}/{fragment}""#);
                }
            }
            if let Some(rest) = href.strip_prefix('/') {
                return format!(r#"href="{base}/{rest}""#);
            }
            caps[0].to_string()
        })
        .into_owned()
}

static EMPTY_LINK_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<a href="">(.*?)</a>"#).unwrap());

/// Placeholder links with no target (`[Virtual Memory]()`) render as plain text.
fn drop_empty_links(html: &str) -> String {
    EMPTY_LINK_RE.replace_all(html, "$1").into_owned()
}

static SRC_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"src="([^"]*)""#).unwrap());

/// Image paths in notes are relative to the note's folder (`images/cpu.png`).
fn fix_image_paths(html: &str, dir: &str) -> String {
    let base = crate::layout::base_path();
    SRC_RE
        .replace_all(html, |caps: &regex::Captures| {
            let src = &caps[1];
            if src.starts_with("http") || src.starts_with("data:") {
                return caps[0].to_string();
            }
            let path = src.trim_start_matches("./");
            if let Some(rest) = path.strip_prefix('/') {
                format!(r#"src="{base}/{rest}""#)
            } else if dir.is_empty() {
                format!(r#"src="{base}/{path}""#)
            } else {
                format!(r#"src="{base}/{dir}/{path}""#)
            }
        })
        .into_owned()
}

fn options() -> Options<'static> {
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;
    options.extension.footnotes = true;
    options.extension.alerts = true;
    options.extension.math_dollars = true;
    options.extension.header_id_prefix = Some(String::new());
    options.render.r#unsafe = true;
    options
}

/// Code blocks come out as plain `<pre><code class="language-x">`; Shiki highlights
/// them after the whole site is built (scripts/highlight.mjs).
pub fn render_markdown(body: &str) -> String {
    let html = markdown_to_html(&prepare_callouts(body), &options());
    strip_anchors(&process_callouts(&html))
}

pub fn render_note(body: &str, dir: &str, links: &LinkMap) -> String {
    drop_empty_links(&fix_links(&fix_image_paths(&render_markdown(body), dir), links))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_callout_type_has_an_icon() {
        for t in GITHUB_TYPES.iter().chain(CUSTOM_TYPES.iter()) {
            callout_icon(t);
        }
    }

    #[test]
    fn custom_callouts_keep_type_and_title() {
        let html = render_markdown("> [!formula] Arithmetic series\n> $1+1$\n");
        assert!(html.contains(r#"class="callout callout-formula""#), "{html}");
        assert!(html.contains("<span>Arithmetic series</span>"), "{html}");

        let html = render_markdown("> [!definition]\n> A **cache** is fast.\n");
        assert!(html.contains("callout-definition"), "{html}");
        assert!(html.contains("<span>Definition</span>"), "{html}");
    }

    #[test]
    fn math_in_callout_titles_renders() {
        let html = render_markdown("> [!formula] Formula ($i^k$ series)\n> x\n");
        assert!(html.contains(r#"Formula (<span data-math-style="inline">i^k</span> series)"#), "{html}");
    }

    #[test]
    fn lowercase_github_callouts_work() {
        let html = render_markdown("> [!tip]\n> hi\n");
        assert!(html.contains("callout-tip"), "{html}");
    }
}
