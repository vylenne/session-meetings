#!/usr/bin/env node
/** Only .ts and .vue in app source. No .js - TypeScript only. */
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = fileURLToPath(new URL(".", import.meta.url));
const root = join(__dirname, "..");
const skip = new Set(["node_modules", ".nuxt", ".output", ".git", "scripts"]);
const allowPaths = ["/scripts/", "eslint.config.mjs"];

function isAllowed(path) {
  return allowPaths.some((a) => path.includes(a));
}

function walk(dir) {
  for (const e of readdirSync(dir)) {
    const p = join(dir, e);
    const rel = p.slice(root.length + 1);
    if (statSync(p).isDirectory()) {
      if (!skip.has(e)) walk(p);
    } else if ((e.endsWith(".js") || e.endsWith(".mjs")) && !isAllowed(rel)) {
      console.error(`[check:no-js] Use TypeScript (.ts), not JS: ${rel}`);
      process.exit(1);
    }
  }
}

walk(root);
