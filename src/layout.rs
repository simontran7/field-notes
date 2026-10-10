use std::sync::atomic::{AtomicBool, Ordering};

static DEV_MODE: AtomicBool = AtomicBool::new(false);

const RELOAD_SCRIPT: &str = r#"<script>
      (function () {
        var version = null;
        setInterval(function () {
          fetch('/__reload__').then(function (r) { return r.text() }).then(function (v) {
            if (version === null) { version = v; return }
            if (v !== version) { location.reload() }
          }).catch(function () {})
        }, 500);
      })();
    </script>"#;

const THEME_TOGGLE_SVG: &str = r#"<button class="theme-toggle" onclick="window.__toggleTheme()" aria-label="Toggle dark mode">
  <svg class="sun-icon" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <circle cx="12" cy="12" r="5" />
    <line x1="12" y1="1" x2="12" y2="3" /><line x1="12" y1="21" x2="12" y2="23" />
    <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" /><line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
    <line x1="1" y1="12" x2="3" y2="12" /><line x1="21" y1="12" x2="23" y2="12" />
    <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" /><line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />
  </svg>
  <svg class="moon-icon" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
  </svg>
</button>"#;

const SITE_NAME: &str = "CS Handbook";

pub fn set_dev_mode(enabled: bool) {
    DEV_MODE.store(enabled, Ordering::Relaxed);
}

pub fn base_path() -> &'static str {
    // Prefix internal links and asset URLs since GitHub Pages serves
    // this project from /cs-handbook/, but the local dev
    // server serves it from `/`
    if DEV_MODE.load(Ordering::Relaxed) { "" } else { "/cs-handbook" }
}

pub fn page(title: &str, body: &str) -> String {
    let reload_script = if DEV_MODE.load(Ordering::Relaxed) { RELOAD_SCRIPT } else { "" };
    let base = base_path();
    // Mermaid is large, so only pages with a diagram load it
    let mermaid_script = if body.contains("language-mermaid") {
        "    <script defer src=\"https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.min.js\"></script>\n"
    } else {
        ""
    };
    format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{title}</title>
    <link rel="icon" type="image/svg+xml" href="{base}/favicon.svg" />
    <link rel="manifest" href="{base}/site.webmanifest" />
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css" />
    <link rel="stylesheet" href="{base}/style.css" />
    <script>
      (function () {{
        var saved = localStorage.getItem('theme-manual');
        var theme = saved || (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
        document.documentElement.setAttribute('data-theme', theme);
      }})();
    </script>
    {reload_script}
  </head>
  <body>
    {THEME_TOGGLE_SVG}
    {body}
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js"></script>
{mermaid_script}    <script defer src="{base}/site.js"></script>
  </body>
</html>
"#
    )
}

pub fn home_page(content_html: &str) -> String {
    let body = format!(
        r#"<div class="container page"><main class="prose">{heading}{content_html}</main></div>"#,
        heading = heading(SITE_NAME)
    );
    page(SITE_NAME, &body)
}

pub fn not_found_page() -> String {
    let base = base_path();
    let body = format!(
        r#"<div class="container page"><main class="prose not-found"><h1>Page not found</h1><p><a href="{base}/">Back to the table of contents</a></p></main></div>"#
    );
    page(&format!("Not Found · {SITE_NAME}"), &body)
}

fn heading(title: &str) -> String {
    if title.is_empty() { String::new() } else { format!("<h1>{title}</h1>") }
}

fn tab_title(title: &str) -> String {
    if title.is_empty() { SITE_NAME.to_string() } else { format!("{title} · {SITE_NAME}") }
}

pub fn note_page(title: &str, content_html: &str) -> String {
    let body = format!(
        r#"<div class="container page"><main class="prose">{heading}{content_html}</main></div>"#,
        heading = heading(title)
    );
    page(&tab_title(title), &body)
}
