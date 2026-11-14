#!/usr/bin/env python3
"""mdBook preprocessor for theorem-style boxes.

Turns a blockquote such as

    > [!PROPOSITION] Law of Addition
    > body...

into a titled box (`<div class="env env-proposition">`) styled by
theme/custom.css. Supported kinds: DEFINITION, THEOREM, PROPOSITION,
COROLLARY. An untitled definition is named after its first **bold** term, so
"> [!DEFINITION]\n> A **cache** is ..." becomes "Definition (Cache)".
GitHub alerts (NOTE, TIP, WARNING, ...) are left to mdBook.
"""

import json
import re
import sys

KINDS = {"definition", "theorem", "proposition", "corollary"}
START = re.compile(r"^>[ \t]*\[!(\w+)\][ \t]*(.*)$")
BOLD = re.compile(r"\*\*(.+?)\*\*")
ABBREVIATION = re.compile(r"\s*\([A-Z0-9]+\)$")


def capitalize(text):
    return text[:1].upper() + text[1:]


def box(kind, title, body):
    heading = capitalize(kind)
    name = title
    if not name and kind == "definition":
        bold = BOLD.search("\n".join(body))
        if bold:
            name = capitalize(ABBREVIATION.sub("", bold.group(1)).strip())
    label = f"{heading} ({name})" if name else heading
    return [
        f'<div class="env env-{kind}">',
        f'<div class="env-title">{label}</div>',
        '<div class="env-body">',
        "",
        *body,
        "",
        "</div>",
        "</div>",
    ]


def convert(markdown):
    lines = markdown.split("\n")
    out = []
    i = 0
    fence = None
    while i < len(lines):
        line = lines[i]
        stripped = line.lstrip()
        if stripped.startswith(("```", "~~~")):
            marker = stripped[:3]
            fence = None if fence == marker else (fence or marker)
        match = None if fence else START.match(line)
        if not match or match.group(1).lower() not in KINDS:
            out.append(line)
            i += 1
            continue
        kind, title = match.group(1).lower(), match.group(2).strip()
        body = []
        i += 1
        while i < len(lines) and lines[i].startswith(">"):
            body.append(re.sub(r"^>[ \t]?", "", lines[i]))
            i += 1
        out.extend(box(kind, title, body))
    return "\n".join(out)


def walk(items):
    for item in items:
        chapter = item.get("Chapter") if isinstance(item, dict) else None
        if chapter:
            chapter["content"] = convert(chapter["content"])
            walk(chapter["sub_items"])


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "supports":
        sys.exit(0)
    _context, book = json.load(sys.stdin)
    walk(book.get("items", book.get("sections", [])))
    json.dump(book, sys.stdout)


if __name__ == "__main__":
    main()
