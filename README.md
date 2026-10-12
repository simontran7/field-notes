# CS Handbook

computer science notes

## Installation

### Dependencies

- [Rust](https://www.rust-lang.org/tools/install)
- For `cargo run -- fmt` only: `clang-format`, [`ruff`](https://docs.astral.sh/ruff/) and `rustfmt` (comes with Rust)

### Steps

```
cargo run -- serve
```

The dev server prints its address: [http://localhost:8000](http://localhost:8000), or the next free port if 8000 is taken. Pick a port with `cargo run -- serve 8002`. It rebuilds and reloads whenever a note changes.

## Writing Notes

Notes are plain Markdown in `notes/`. `notes/index.md` is the home page, and every other `.md` file becomes a page at its path (`notes/tools/git.md` is served at `/tools/git/`). 

### Links

Links between notes are relative paths (e.g., `[Generics](c.md#generics)`). A link to a note that doesn't exist fails the build.

### Images

Images are stored the `images/` folder next to the note.

### Math

- Inline LaTeX: `$...$`
- Block LaTeX: `$$...$$`

### Callouts

Callouts are written as `[!<kind>] <title>`, where they are rendered as "<kind> (<title>)".

  - `> [!DEFINITION]` 
  - `> [!THEOREM]`
  - `> [!PROPOSITION]`
  - `> [!COROLLARY]`
  - `> [!NOTE]`
  - `> [!TIP]`
  - `> [!WARNING]`

### Code

Highlighted at build time with [Lumis](https://lumis.sh) in IntelliJ's own colors: IntelliJ Dark in dark mode and IntelliJ Light in light mode. `theme/intellij.json` holds both schemes, and `src/highlight.rs` maps tree-sitter captures to IntelliJ's color keys. Supported: `rust`, `c`, `cpp`, `python`, `bash`, `console`, `ocaml`, `asm`. ` ```bash ` blocks get a `$` prompt, and ` ```console ` for sessions with output (`$ ` lines are highlighted as bash, output stays plain).

To format code blocks:

```
cargo run -- fmt # format C, C++, Python and Rust blocks in notes/
cargo run -- fmt --check  # only list the blocks that would change
```

The format styles live in `fmt/`: 4-space indents, opening braces on the same line, no space after casts, and 80 columns so lines fit a code block on the site. 
