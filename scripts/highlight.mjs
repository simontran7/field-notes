// Highlights every code block in the built site with Shiki, using the same Darcula
// theme file the Obsidian vault uses (.obsidian/shiki-themes/darcula.json), so code
// looks identical in Obsidian and on the site.
//
// Usage: node scripts/highlight.mjs <dist dir> <theme json>

import { readFile, readdir, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { bundledLanguages, createHighlighter } from "shiki";

const [distDir, themePath] = process.argv.slice(2);

const theme = JSON.parse(await readFile(themePath, "utf8"));
theme.name = "darcula";

const CODE_RE = /<pre><code(?: class="language-([^"\s]+)")?>([\s\S]*?)<\/code><\/pre>/g;

const ENTITIES = { "&lt;": "<", "&gt;": ">", "&quot;": '"', "&#39;": "'", "&amp;": "&" };

function unescapeHtml(text) {
  return text.replace(/&(lt|gt|quot|#39|amp);/g, (m) => ENTITIES[m]);
}

function escapeHtml(text) {
  return text.replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c]);
}

async function htmlFiles(dir) {
  const out = [];
  for (const entry of await readdir(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) out.push(...(await htmlFiles(path)));
    else if (entry.name.endsWith(".html")) out.push(path);
  }
  return out;
}

const files = await htmlFiles(distDir);
const pages = await Promise.all(files.map(async (file) => [file, await readFile(file, "utf8")]));

// Load only the languages the site actually uses
const used = new Set();
for (const [, html] of pages) {
  for (const [, lang] of html.matchAll(CODE_RE)) {
    if (lang && lang in bundledLanguages) used.add(lang);
  }
}

const highlighter = await createHighlighter({ themes: [theme], langs: [...used] });

let count = 0;
for (const [file, html] of pages) {
  const out = html.replace(CODE_RE, (whole, lang = "text", escaped) => {
    if (lang === "mermaid") return whole; // site.js turns these into diagrams
    const code = unescapeHtml(escaped).replace(/\n$/, "");
    const shikiLang = used.has(lang) ? lang : "text";
    const highlighted = highlighter.codeToHtml(code, { lang: shikiLang, theme: "darcula" });
    count++;
    return `<div class="code-block" data-lang="${escapeHtml(lang)}"><span class="code-lang">${escapeHtml(lang)}</span>${highlighted}</div>`;
  });
  if (out !== html) await writeFile(file, out);
}

console.log(`highlighted ${count} code blocks (${[...used].sort().join(", ")})`);
