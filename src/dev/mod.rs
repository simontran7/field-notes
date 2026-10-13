//! `cargo run -- serve`: wires the SSG to a file watcher and a web server.
//! The watcher rebuilds `dist/` on every change; the server serves it and
//! tells open pages to reload.

mod server;
mod watch;

use std::path::Path;
use std::time::Instant;

use tower_livereload::LiveReloadLayer;

use crate::ssg::{self, Site};

pub fn run() -> std::io::Result<()> {
    let mut site = Site::new(true);
    let start = Instant::now();
    let stats = site.build()?;
    println!(
        "built {} notes -> {} in {:.1?}",
        stats.notes,
        ssg::DIST_DIR,
        start.elapsed()
    );

    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();

    std::thread::spawn(move || {
        let notes_dir = ssg::notes_dir();
        watch::watch(&[&notes_dir, Path::new(ssg::PUBLIC_DIR)], || {
            let start = Instant::now();
            match site.build() {
                Ok(stats) => {
                    reloader.reload();
                    println!(
                        "rebuilt {} of {} pages in {:.1?}",
                        stats.rendered,
                        stats.pages,
                        start.elapsed()
                    );
                }
                Err(e) => eprintln!("build failed: {e}"),
            }
        });
    });

    server::run(Path::new(ssg::DIST_DIR), livereload)
}
