//! Renders `$...$` and `$$...$$` to KaTeX HTML at build time, so pages need
//! only KaTeX's stylesheet and fonts (in public/katex/), not its JavaScript.

use std::sync::LazyLock;

use katex::{KatexContext, Settings};
use regex::Regex;

static KATEX: LazyLock<KatexContext> = LazyLock::new(KatexContext::default);

/// comrak's markup for math (`math_dollars`), and `title_math`'s for callout titles.
static MATH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<span data-math-style="(inline|display)">([\s\S]*?)</span>"#).unwrap()
});

/// An opening tag with attributes, and one attribute in it.
static TAG_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<([a-zA-Z][\w:-]*)((?:\s[\w:-]+="[^"]*")+)(\s*/?)>"#).unwrap());
static ATTRIBUTE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\s[\w:-]+="[^"]*""#).unwrap());

/// katex-rs keeps attributes in a HashMap, so their order changes from run to
/// run. Sorting them makes every build of an unchanged note byte-for-byte the same.
fn sort_attributes(html: &str) -> String {
    TAG_RE
        .replace_all(html, |caps: &regex::Captures| {
            let mut attributes: Vec<&str> = ATTRIBUTE_RE
                .find_iter(&caps[2])
                .map(|m| m.as_str())
                .collect();
            attributes.sort_unstable();
            format!("<{}{}{}>", &caps[1], attributes.concat(), &caps[3])
        })
        .into_owned()
}

fn render(tex: &str, display: bool) -> String {
    let settings = Settings::builder()
        .display_mode(display)
        .throw_on_error(false)
        .build();
    // With throw_on_error off, bad LaTeX renders as red source text instead of
    // failing, like it did in the browser.
    match katex::render_to_string(&KATEX, tex, &settings) {
        Ok(html) => sort_attributes(&html),
        Err(e) => {
            eprintln!("warning: can't render math {tex:?}: {e}");
            super::render::escape_html(tex)
        }
    }
}

pub fn render_all(html: &str) -> String {
    MATH_RE
        .replace_all(html, |caps: &regex::Captures| {
            let tex = super::render::unescape_html(&caps[2]);
            render(&tex, &caps[1] == "display")
        })
        .into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_inline_and_display_math() {
        let html = render_all(
            r#"<p><span data-math-style="inline">a &lt; b</span></p><span data-math-style="display">x^2</span>"#,
        );
        assert!(!html.contains("data-math-style"));
        assert!(html.contains(r#"<annotation encoding="application/x-tex">a &lt; b</annotation>"#));
        assert!(html.contains("katex-display"));
    }

    #[test]
    fn output_is_the_same_every_time() {
        let tex = r"\sqrt{x} = \left( \frac{a}{b} \right)";
        let first = render(tex, true);
        for _ in 0..20 {
            assert_eq!(render(tex, true), first);
        }
    }

    #[test]
    fn bad_math_renders_as_an_error_instead_of_failing() {
        let html = render_all(r#"<span data-math-style="inline">\frac{</span>"#);
        assert!(html.contains("katex-error"));
    }
}
