use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use notify::{RecursiveMode, Watcher};

pub fn watch(dirs: &[&Path], mut on_change: impl FnMut()) {
    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx).expect("failed to create file watcher");
    for dir in dirs {
        watcher
            .watch(dir, RecursiveMode::Recursive)
            .unwrap_or_else(|e| panic!("failed to watch {dir:?}: {e}"));
    }

    for res in rx.iter() {
        let Ok(event) = res else { continue };
        let relevant = event.paths.iter().any(|p| {
            !p.components()
                .any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
        });
        if !relevant {
            continue;
        }
        std::thread::sleep(Duration::from_millis(150));
        while rx.try_recv().is_ok() {}

        on_change();
    }
}
