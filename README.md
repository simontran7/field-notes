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

The dev server prints its address: [http://localhost:8000](http://localhost:8000), or the next free port if 8000 is taken. Pick a port with `cargo run -- serve 8002`. It rebuilds and reloads whenever a note changes, re-rendering only the notes that changed (adding or removing a note re-renders all of them, since that can fix or break links).

## Writing Notes

Notes are plain Markdown in `notes/`. `notes/index.md` is the home page, and every other `.md` file becomes a page at its path (`notes/tools/git.md` is served at `/tools/git/`). 

### Links

Links between notes are relative paths (e.g., `[Generics](c.md#generics)`). A link to a note that doesn't exist fails the build.

### Images

Images are stored the `images/` folder next to the note.

### Math

- Inline LaTeX: `$...$`
- Block LaTeX: `$$...$$`

Rendered to HTML at build time with [katex-rs](https://github.com/katex-rs/katex-rs), a Rust port of KaTeX 0.18.5. Pages load only KaTeX's stylesheet and fonts, from `public/katex/`; update those together with the `katex-rs` version. LaTeX that doesn't parse shows up red on the page, and the build prints a warning.

### Callouts

Callouts are written as `[!<kind>] <title>`, where they are rendered as "<kind> (<title>)".

- General
  - `> [!DEFINITION]`
  - `> [!NOTE]`
  - `> [!TIP]`
  - `> [!WARNING]`
- Math
  - `> [!THEOREM]`
  - `> [!PROPOSITION]`
  - `> [!COROLLARY]`

### Code

Highlighted at build time with [Lumis](https://lumis.sh) in IntelliJ's own colors: IntelliJ Dark in dark mode and IntelliJ Light in light mode. `theme/intellij.json` holds both schemes, and `src/ssg/highlight.rs` maps tree-sitter captures to IntelliJ's color keys. Supported: `rust`, `c`, `python`, `bash`, `console`, `asm`. ` ```console ` is for commands typed at a prompt: start each command with `$ ` and put its output on the lines under it (commands are highlighted as bash, output stays plain, and the copy button copies only the commands). ` ```bash ` is for the contents of a script file. Commands with `<placeholders>` go in ` ```syntax ` blocks instead.

To format code blocks:

```
cargo run -- fmt # format C, Python and Rust blocks in notes/
cargo run -- fmt --check  # only list the blocks that would change
```

To check notes against the style rules in `notes/AGENTS.md` (no em dashes, `_` for italics, centered tables, supported callout types, `console`/`bash` shell blocks):

```
cargo run -- lint
```

It also runs on every push, and a failure stops the deploy.

The format styles live in `fmt/`: 4-space indents, opening braces on the same line, no space after casts, and 80 columns so lines fit a code block on the site. 
