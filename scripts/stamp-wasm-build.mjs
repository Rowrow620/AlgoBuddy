import { execFileSync } from "node:child_process";
import { readFile, writeFile } from "node:fs/promises";
import path from "node:path";

const siteDirectory = path.resolve(process.env.SMOKE_SITE_DIR ?? "dist");
const indexPath = path.join(siteDirectory, "index.html");
const markerPath = path.join(siteDirectory, "deploy-sha.txt");
const metaName = "algobuddy-deploy-sha";

function resolveRevision() {
  const configuredRevision = process.env.DEPLOY_SHA?.trim();
  if (configuredRevision) {
    return configuredRevision;
  }

  return execFileSync("git", ["rev-parse", "HEAD"], { encoding: "utf8" }).trim();
}

const revision = resolveRevision();
if (!/^[0-9a-f]{40}$/i.test(revision)) {
  throw new Error("DEPLOY_SHA must be a 40-character Git commit SHA");
}

let html = await readFile(indexPath, "utf8");
const existingMeta = new RegExp(
  `<meta\\s+name=["']${metaName}["'][^>]*>`,
  "i",
);
const revisionMeta = `<meta name="${metaName}" content="${revision}">`;

if (existingMeta.test(html)) {
  html = html.replace(existingMeta, revisionMeta);
} else {
  if (!html.includes("</head>")) {
    throw new Error(`Cannot stamp ${indexPath}: closing head element is missing`);
  }
  html = html.replace("</head>", `    ${revisionMeta}\n</head>`);
}

await Promise.all([
  writeFile(indexPath, html, "utf8"),
  writeFile(markerPath, `${revision}\n`, "utf8"),
]);

console.log(`Stamped WebAssembly bundle with revision ${revision}`);
