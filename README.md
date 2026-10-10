# CS Handbook

computer science notes

## Installation

### Dependencies

- [Rust](https://www.rust-lang.org/tools/install)

### Steps

```
cargo run -- serve
```

Site will be live at [http://localhost:8000](http://localhost:8000) (use `PORT=8001` if 8000 is taken). It rebuilds and reloads whenever a note changes.

## Writing notes

Notes are plain Markdown in `notes/`. `notes/TOC.md` is the home page, and every other `.md` file becomes a page at its path (`notes/tools/git.md` is served at `/tools/git/`). See `AGENTS.md` for the writing conventions.

- **Links** between notes are relative paths (`[Generics](c.md#generics)`). A link to a note that doesn't exist fails the build.
- **Images** go in an `images/` folder next to the note (`images/cpu.png`).
- **Math** uses `$...$` and `$$...$$` (rendered with KaTeX).
- **Callouts** use GitHub's syntax (`> [!NOTE]`, `> [!TIP]`, `> [!IMPORTANT]`, `> [!WARNING]`, `> [!CAUTION]`), plus `> [!DEFINITION]` and `> [!FORMULA]`. Text after the type becomes the title: `> [!FORMULA] Arithmetic series`.
- **Code** is highlighted at build time with [Lumis](https://lumis.sh) (tree-sitter parsers and Neovim's highlight queries) and `theme/darcula.json`, IntelliJ's Darcula colors taken from JetBrains' own color scheme files, including the Rust, Python, shell and C/C++ plugins. Supported: `rust`, `c`, `cpp`, `python`, `bash`, `console`, `ocaml`, `asm`. ` ```bash ` blocks get a `$` prompt; use ` ```console ` for sessions with output (`$ ` lines are highlighted as bash, output stays plain).
