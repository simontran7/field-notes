use std::fs::File;
use std::path::{Component, Path};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock, mpsc};
use std::time::Duration;

use notify::{RecursiveMode, Watcher};
use tiny_http::{Header, Request, Response, Server};

// Requests hold a read lock and rebuilds hold the write lock, so a request never
// sees dist/ half-deleted mid-rebuild.
static BUILD_LOCK: RwLock<()> = RwLock::new(());

/// The port asked for, in any of the ways people type it: `PORT=8002 cargo run -- serve`,
/// `cargo run -- serve 8002`, `cargo run -- serve PORT=8002` or `--port 8002`.
fn requested_port() -> Option<u16> {
    let args: Vec<String> = std::env::args().skip(2).collect();
    let from_args = args.iter().enumerate().find_map(|(i, a)| {
        let value = a
            .strip_prefix("PORT=")
            .or_else(|| a.strip_prefix("--port="))
            .or_else(|| {
                (a == "--port")
                    .then(|| args.get(i + 1).map(String::as_str))
                    .flatten()
            })
            .unwrap_or(a);
        value.parse().ok()
    });
    from_args.or_else(|| std::env::var("PORT").ok()?.parse().ok())
}

/// Binds the requested port, or the first free one from 8000 up if none was asked for.
fn bind(requested: Option<u16>) -> (Server, String) {
    let candidates: Vec<u16> = match requested {
        Some(port) => vec![port],
        None => (8000..8020).collect(),
    };
    for port in candidates {
        let addr = format!("127.0.0.1:{port}");
        if let Ok(server) = Server::http(&addr) {
            return (server, addr);
        }
    }
    match requested {
        Some(port) => {
            eprintln!("port {port} is already in use; pick another, e.g. `cargo run -- serve 8010`")
        }
        None => {
            eprintln!("ports 8000-8019 are all in use; pick one with `cargo run -- serve <port>`")
        }
    }
    std::process::exit(1);
}

pub fn run(dist_dir: &Path) -> std::io::Result<()> {
    let version = Arc::new(AtomicU64::new(0));

    {
        let version = Arc::clone(&version);
        std::thread::spawn(move || watch_and_rebuild(version));
    }

    let (server, addr) = bind(requested_port());
    println!("dev server running at http://{addr}");

    for request in server.incoming_requests() {
        let _guard = BUILD_LOCK.read().unwrap_or_else(|e| e.into_inner());
        let full_url = request.url().to_string();
        let (url, query) = match full_url.split_once('?') {
            Some((url, query)) => (url, Some(query)),
            None => (full_url.as_str(), None),
        };

        if url == "/__reload__" {
            let body = version.load(Ordering::Relaxed).to_string();
            let _ = request.respond(Response::from_string(body));
            continue;
        }

        serve_file(request, dist_dir, url, query);
    }

    Ok(())
}

fn watch_and_rebuild(version: Arc<AtomicU64>) {
    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx).expect("failed to create file watcher");
    let notes_dir = crate::notes_dir();
    watcher
        .watch(&notes_dir, RecursiveMode::Recursive)
        .unwrap_or_else(|e| panic!("failed to watch {notes_dir:?}: {e}"));
    watcher
        .watch(Path::new("public"), RecursiveMode::Recursive)
        .expect("failed to watch public/");

    for res in rx.iter() {
        // Ignore editor temp files and dot-folders (e.g. .git); only real content changes rebuild.
        let Ok(event) = res else { continue };
        let relevant = event.paths.iter().any(|p| {
            !p.components()
                .any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
        });
        if !relevant {
            continue;
        }
        // Debounce: an editor save often fires several events in quick succession.
        std::thread::sleep(Duration::from_millis(150));
        while rx.try_recv().is_ok() {}

        let _guard = BUILD_LOCK.write().unwrap_or_else(|e| e.into_inner());
        match crate::build_site(true) {
            Ok(count) => {
                version.fetch_add(1, Ordering::Relaxed);
                println!("rebuilt {count} notes");
            }
            Err(e) => eprintln!("build failed: {e}"),
        }
    }
}

fn percent_decode(s: &str) -> String {
    fn hex(b: u8) -> Option<u8> {
        (b as char).to_digit(16).map(|d| d as u8)
    }

    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let (Some(hi), Some(lo)) = (hex(bytes[i + 1]), hex(bytes[i + 2]))
        {
            out.push(hi << 4 | lo);
            i += 3;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn serve_file(request: Request, dist_dir: &Path, url: &str, query: Option<&str>) {
    let decoded = percent_decode(url);
    let rel = decoded.trim_start_matches('/');

    // Only plain path segments: no `..`, no absolute paths escaping dist/.
    let safe = Path::new(rel)
        .components()
        .all(|c| matches!(c, Component::Normal(_)));

    let mut path = if !safe {
        dist_dir.join("404.html")
    } else if rel.is_empty() {
        dist_dir.join("index.html")
    } else {
        dist_dir.join(rel)
    };

    if safe && path.is_dir() {
        // Match GitHub Pages, which redirects /foo to /foo/ so relative links resolve the same.
        if !url.ends_with('/') {
            let location = match query {
                Some(q) => format!("{url}/?{q}"),
                None => format!("{url}/"),
            };
            let header = Header::from_bytes(&b"Location"[..], location.as_bytes()).unwrap();
            let _ = request.respond(Response::empty(301).with_header(header));
            return;
        }
        path = path.join("index.html");
    }

    let (path, status) = if safe && path.is_file() {
        (path, 200u16)
    } else {
        (dist_dir.join("404.html"), 404u16)
    };

    let content_type = content_type_for(&path);
    let header = Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap();

    match File::open(&path) {
        Ok(file) => {
            let response = Response::from_file(file)
                .with_status_code(status)
                .with_header(header);
            let _ = request.respond(response);
        }
        Err(_) => {
            let _ = request.respond(Response::from_string("not found").with_status_code(404));
        }
    }
}

fn content_type_for(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("avif") => "image/avif",
        Some("ico") => "image/x-icon",
        Some("svg") => "image/svg+xml",
        Some("woff2") => "font/woff2",
        Some("woff") => "font/woff",
        Some("webmanifest") => "application/manifest+json",
        Some("pdf") => "application/pdf",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}
