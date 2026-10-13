use std::net::TcpListener;
use std::path::Path;

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};
use tower_livereload::LiveReloadLayer;

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
fn bind(requested: Option<u16>) -> (TcpListener, String) {
    let candidates: Vec<u16> = match requested {
        Some(port) => vec![port],
        None => (8000..8020).collect(),
    };
    for port in candidates {
        let addr = format!("127.0.0.1:{port}");
        if let Ok(listener) = TcpListener::bind(&addr) {
            return (listener, addr);
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

/// Serves `dist_dir` until the process exits. `livereload` adds a script to every
/// page that reloads it when the watcher calls its reloader.
pub fn run(dist_dir: &Path, livereload: LiveReloadLayer) -> std::io::Result<()> {
    let (listener, addr) = bind(requested_port());
    listener.set_nonblocking(true)?;
    println!("dev server running at http://{addr}");

    // ServeDir redirects /foo to /foo/ and serves its index.html, like GitHub Pages,
    // and refuses paths that escape dist/.
    let files =
        ServeDir::new(dist_dir).not_found_service(ServeFile::new(dist_dir.join("404.html")));
    let app = Router::new().fallback_service(files).layer(livereload);

    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()?
        .block_on(async {
            let listener = tokio::net::TcpListener::from_std(listener)?;
            axum::serve(listener, app).await
        })
}
