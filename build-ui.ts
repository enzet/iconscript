#!/usr/bin/env node

import {build} from "esbuild";
import {fileURLToPath} from "url";
import {dirname, join, basename} from "path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

async function buildUI(): Promise<void> {
    const minify =
        process.argv.includes("--minify") || process.argv.includes("-m");
    const outputFile = minify
        ? join(__dirname, "ui.bundle.min.js")
        : join(__dirname, "ui.bundle.js");

    try {
        await build({
            entryPoints: [join(__dirname, "ui.ts")],
            bundle: true,
            platform: "browser",
            format: "iife",
            outfile: outputFile,
            minify: minify,
            sourcemap: minify,
            target: "es2020",
        });
        console.log(`Created: ${basename(outputFile)}.`);
    } catch (error) {
        console.error("Build failed:", error);
        process.exit(1);
    }
}

buildUI();
