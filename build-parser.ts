#!/usr/bin/env node

import {build} from "esbuild";
import {fileURLToPath} from "url";
import {dirname, join, basename} from "path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

async function buildParser(): Promise<void> {
    const minify =
        process.argv.includes("--minify") || process.argv.includes("-m");
    const outputFile = minify
        ? join(__dirname, "parser.bundle.min.js")
        : join(__dirname, "parser.bundle.js");

    try {
        await build({
            entryPoints: [join(__dirname, "parser.ts")],
            bundle: true,
            platform: "browser",
            format: "iife",
            globalName: "IconScriptParser",
            outfile: outputFile,
            minify: minify,
            sourcemap: minify,
            target: "es2020",
            external: [],
        });
        console.log(`Created: ${basename(outputFile)}.`);
    } catch (error) {
        console.error("Build failed:", error);
        process.exit(1);
    }
}

buildParser();
