# AGENTS.md

These notes are plain Markdown. `index.md` (this folder) is the home page; every other `.md` file becomes a page at its path (`tools/git.md` -> `/tools/git/`). Preview by running `cargo run -- serve` from the repo root.

Write my in this clear, conversational, explanatory style. 
- Example 1: https://jamesbornholt.com/blog/memory-models/
- Example 2: "Communion, also called Eucharist, which means Thanksgiving is considered the summit of Catholic life. When you receive your first communion, you're receiving for the first time the body of Christ, hidden in the form of bread, which we call a host. The host must be consecrated through special prayers said by a priest for it to become the body of Christ. This happens during a Catholic Church service, which is called a mass. During the mass, Catholics come together to profess our fait, worship God, listen to readings of the Bible and receive communion. This is similar to other Christian denominations who gather regularly for church services. Unlike Catholics, however, other Christians believe that communion is a symbol of Christ's body, and not Christ himself. While there are more differences between Catholics and other Christians, the nature of the Eucharist is probably the biggest contrast. The foundation of the Eucharist can be found in Matthew, Mark, and Luke when Jesus proclaims at the Last Supper that the bread and wine is his body and blood and commands his disciples to do the same in his memory. Confirmation is basically like high school graduation for Catholics. After being baptized, receiving your first communion, and going to confession for the first time, confirmation is a Coming of Age ceremony that gives you a chance to affirm your beliefs and become an adult in the Catholic church. This Sacrament is mostly centered around the Holy Spirit, as newly confirmed Catholics receive an increase in the gifts of the holy spirit, like wisdom, understanding and prudence or good judgment. During confirmation, a priest places their hands on the person's head and prays that they will receive the Holy Spirit. You can find references to confirmation in the book of Acts, which states that the disciples laid hands on believers to receive the Holy Spirit."
- No punchline sentences. Each paragraph should just explain the next step, without summary flourishes like "That's write contention in its purest form, and it's why reducing sharing matters so much more than choosing a faster atomic." or "What makes this cheap enough to use is...", and without closing one-sentence quotes.
- No contrast sentences like "it's not X, it's Y". State the point directly, e.g. "Since the lock is confined to a single cache line in a single core's cache, other cores can keep accessing the rest of memory while the atomic instruction runs." instead of "The lock prefix doesn't lock the bus. It only locks a single cache line." This includes subtler forms: trailing negations ("...not yet started or fully finished, never partway through"), "Although it looks like X, it's Y", and "yet ... instead of ...".
- No em dashes. Check with grep before finishing.
- Center align all markdown tables (`| :---: | :---: |`).
- Use `_` over `*` for italics.
- Pick callout types by what the reader should do with the information:
  - `> [!DEFINITION]`: defines a term (the bolded term and what it is).
  - `> [!FORMULA]`: a reusable formula, law, or property worth looking up later. Include the sentence that introduces it. When the formula has its own name, use it as the title (`> [!FORMULA] Arithmetic series`). Worked-example steps and derivations stay as plain math.
  - `> [!NOTE]`: background or clarifying facts worth knowing, with no action implied.
  - `> [!TIP]`: optional advice, such as a better way, a technique, a shortcut, a handy tool, or a "prefer X over Y" recommendation.
  - `> [!IMPORTANT]`: something required for the thing to work, or a key distinction that is easy to get wrong.
  - `> [!WARNING]`: a gotcha that silently produces wrong results or bugs (e.g., operator precedence, shell syntax that misparses).
  - `> [!CAUTION]`: an action that destroys data, can't be undone, or opens a security hole (e.g., `rm`, `crontab -r`, `chmod 777`).
- Telling similar callout types apart:
  - definition vs NOTE: use definition when the sentence introduces a bolded term and says what it is. Use NOTE for any other fact about a term that is already defined.
  - Which definitions get a box: box a definition when its term is the main subject of the paragraph or section (e.g., the opening sentence of "Cache Associativity" for each cache type), however the sentence is worded ("is", "lets", "consists of", "is called"). Keep a term inline when it is introduced in passing inside a sentence about something else (e.g., "...each called a **register**", "...known as a **clock cycle**"). Sibling terms in the same section are either all boxed or all inline.
  - NOTE vs TIP: ask whether the callout suggests something the reader could choose to do. A suggestion goes in TIP. A fact the reader only needs to know goes in NOTE.
  - TIP vs IMPORTANT: ask what happens if the reader ignores it. If the result is merely slower, longer, or less elegant, use TIP. If the code or command fails, use IMPORTANT.
  - IMPORTANT vs WARNING: IMPORTANT covers something the reader must do for things to work, and failing to do it is usually obvious (an error, nothing happens). WARNING covers a mistake that looks fine and quietly gives the wrong result.
  - WARNING vs CAUTION: WARNING covers mistakes that cost debugging time. CAUTION covers actions whose damage stays after you notice, such as deleted files, lost commits, or exposed permissions.
- Links between notes are relative paths to the file, e.g. `[Two Pointers](../algorithmic-techniques/two-pointers.md)` or `[Generics](c.md#generics)`. A link to a note that doesn't exist fails the build.
- Images go in an `images/` folder next to the note and are linked relatively (`images/cpu.png`).
- After writing or editing C, C++, Python or Rust code blocks, run `cargo run -- fmt` from the repo root. It formats them with clang-format, ruff and rustfmt (4-space indents, opening braces on the same line, `(int)x` casts with no space, 80 columns). It skips blocks that are pieces of a larger program, so format those by hand in the same style.
- Shell code blocks:
  - ` ```bash `: commands only, with no `$` and no output. The `$` prompt is added automatically when displayed.
  - ` ```console `: a terminal session that shows output. Type `$ ` before each command so commands and output can be told apart.
  - Don't use ` ```sh `, ` ```shell `, or ` ```zsh `.
