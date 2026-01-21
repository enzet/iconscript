#!/usr/bin/env node

import {build} from "esbuild";
import {fileURLToPath} from "url";
import {dirname, join, basename} from "path";
import {existsSync, mkdirSync, copyFileSync, chmodSync} from "fs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

async function buildCLI(): Promise<void> {
    const jsDir = join(__dirname, "..");
    const projectRoot = join(jsDir, "..");
    const outputFile = join(jsDir, "dist", "cli", "generate-svg.js");
    const grammarLexer = join(projectRoot, "grammar", "IconScriptLexer.ts");

    // Check if grammar files exist.
    if (!existsSync(grammarLexer)) {
        console.error(
            "Error: Grammar files not found. Please run './scripts/generate-grammar.sh' first.",
        );
        process.exit(1);
    }

    // Ensure output directory exists.
    const outputDir = dirname(outputFile);
    if (!existsSync(outputDir)) {
        mkdirSync(outputDir, {recursive: true});
    }

    try {
        await build({
            entryPoints: [join(jsDir, "src", "cli", "generate-svg.ts")],
            bundle: true,
            platform: "node",
            format: "esm",
            outfile: outputFile,
            target: "es2020",
            banner: {
                js: "#!/usr/bin/env node",
            },
            // Mark paper and paperjs-offset as external since they have complex
            // Node.js dependencies (jsdom) that are difficult to bundle.
            // These will need to be installed when using the CLI.
            external: ["paper", "paperjs-offset"],
            nodePaths: [join(jsDir, "node_modules")],
        });
        console.log(`Created: ${basename(outputFile)}.`);

        // Copy the CommonJS wrapper for npm binary compatibility.
        const wrapperSource = join(
            jsDir,
            "src",
            "cli",
            "generate-svg-wrapper.cjs",
        );
        const wrapperDest = join(
            jsDir,
            "dist",
            "cli",
            "generate-svg-wrapper.cjs",
        );
        copyFileSync(wrapperSource, wrapperDest);
        chmodSync(wrapperDest, 0o755);
        console.log(`Created: ${basename(wrapperDest)}.`);
    } catch (error) {
        console.error("Build failed:", error);
        process.exit(1);
    }
}

buildCLI();
