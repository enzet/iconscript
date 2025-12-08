#!/usr/bin/env node

/**
 * Build script to create bundled-parser.js from parser.mjs.
 * Uses esbuild to bundle the parser and all dependencies into a single file.
 */

import {build} from "esbuild";
import {fileURLToPath} from "url";
import {dirname, join, basename} from "path";
import {statSync} from "fs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

async function buildParser() {
    const minify = process.argv.includes("--minify") || process.argv.includes("-m");
    const outputFile = minify
        ? join(__dirname, "bundled-parser.min.js")
        : join(__dirname, "bundled-parser.js");

    try {
        await build({
            entryPoints: [join(__dirname, "parser.mjs")],
            bundle: true,
            platform: "browser",
            format: "iife",
            globalName: "IconScriptParser",
            outfile: outputFile,
            minify: minify,
            sourcemap: minify, // Generate sourcemap for minified version
            target: "es2020",
            external: [], // Bundle everything, including antlr4 and paper
        });

        const fileSize = statSync(outputFile).size;
        const sizeKB = (fileSize / 1024).toFixed(2);

        console.log(`✓ Successfully created ${basename(outputFile)}`);
        console.log(`  Size: ${sizeKB} KB`);
        console.log("  You can now use it in the browser with:");
        console.log(`  <script src="${basename(outputFile)}"></script>`);
        console.log("  Then access: IconScriptParser.parseIconsFile() and IconScriptParser.IconGenerator");
    } catch (error) {
        console.error("Build failed:", error);
        process.exit(1);
    }
}

buildParser();

