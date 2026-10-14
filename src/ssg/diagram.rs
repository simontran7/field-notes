use std::hash::{Hash, Hasher};
use std::sync::LazyLock;

use mermaid_rs_renderer::{RenderOptions, Theme, render_with_options};
use regex::Regex;

static ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"(id="|url\(#|href="#)([^"\)]+)"##).unwrap());

static BACKGROUND_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(<svg[^>]*>)<rect x="[^"]*" y="[^"]*" width="[^"]*" height="[^"]*" fill="[^"]*"/>"#,
    )
    .unwrap()
});

fn render(source: &str, theme: Theme, prefix: &str) -> Result<String, String> {
    let options = RenderOptions {
        theme,
        ..RenderOptions::mermaid_default()
    };
    let svg = render_with_options(source, options).map_err(|e| e.to_string())?;
    let svg = BACKGROUND_RE.replace(&svg, "$1");
    Ok(ID_RE
        .replace_all(&svg, format!("${{1}}{prefix}-$2"))
        .into_owned())
}

pub fn mermaid(source: &str) -> Option<String> {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    source.hash(&mut hasher);
    let id = format!("d{:x}", hasher.finish() & 0xff_ffff);

    let rendered = render(source, Theme::mermaid_default(), &format!("{id}-light"))
        .and_then(|light| Ok((light, render(source, Theme::dark(), &format!("{id}-dark"))?)));
    match rendered {
        Ok((light, dark)) => Some(format!(
            r#"<div class="diagram"><div class="diagram-light">{light}</div><div class="diagram-dark">{dark}</div></div>"#
        )),
        Err(e) => {
            eprintln!("warning: mermaid diagram doesn't parse, showing its source: {e}");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_both_themes_with_distinct_ids() {
        let html = mermaid("flowchart LR\n  A --> B").unwrap();
        assert_eq!(html.matches("<svg").count(), 2);
        assert!(html.contains("-light-arrow-0") && html.contains("-dark-arrow-0"));
        assert!(!html.contains(r#"id="arrow-0""#));
        assert!(!html.contains("url(#arrow-0)"));
    }

    #[test]
    fn unparsable_diagram_is_none() {
        assert!(mermaid("not a diagram at all").is_none());
    }
}
