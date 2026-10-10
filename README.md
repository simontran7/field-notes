# CS Handbook

Computer science notes, written in Obsidian and published as a static site.

## Installation

### Dependencies

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org) (Shiki syntax highlighting at build time)

### Steps

```
npm install
cargo run -- serve
```

Site will be live at [http://localhost:8000](http://localhost:8000) (use `PORT=8001` if 8000 is taken). It rebuilds whenever a note changes.

## Where the notes come from

The build reads, in order:

1. `NOTES_DIR`, if set
2. the Obsidian vault at `../programming`, if it exists
3. `notes/`, a synced copy of the vault that GitHub Actions builds from

Before pushing, refresh `notes/` from the vault:

```
scripts/sync-notes.sh
```

`TOC.md` becomes the home page. Code blocks are highlighted with Shiki using the vault's own Darcula theme (`.obsidian/shiki-themes/darcula.json`).
