#!/usr/bin/env node

import fs from "fs";
import path from "path";

// Import parser and generator from parser.mjs (shared code).
import {parseIconsFile} from "./parser.mjs";

function generateIcons(inputFile = "main.iconscript", outputDir = "output") {
    try {
        // Read the specified file or default to `main.iconscript`..
        const iconsContent = fs.readFileSync(inputFile, "utf8");
        const icons = parseIconsFile(iconsContent);

        // Ensure output directory exists.
        if (!fs.existsSync(outputDir)) {
            fs.mkdirSync(outputDir, {recursive: true});
        }

        let iconCount = 0;

        for (let i = 0; i < icons.length; i++) {
            const icon = icons[i];
            const svg = icon.svg;

            if (svg) {
                // Generate filename.
                let filename;
                if (icon.name && icon.name !== "temp") {
                    filename = `${icon.name}.svg`;
                } else {
                    filename = `icon_${i}.svg`;
                }

                const filepath = path.join(outputDir, filename);
                fs.writeFileSync(filepath, svg);
                console.log(`Generated: ${filename}`);
                iconCount++;
            }
        }

        console.log(
            `\nGenerated ${iconCount} SVG files in the ${outputDir} directory.`
        );
    } catch (error) {
        console.error("Error:", error.message);
        process.exit(1);
    }
}

// Parse command-line arguments.
function parseArgs() {
    let inputFile = "main.iconscript";
    let outputDir = "output";

    for (let i = 2; i < process.argv.length; i++) {
        const arg = process.argv[i];
        if (arg === "-i" || arg === "--input") {
            if (i + 1 < process.argv.length) {
                inputFile = process.argv[i + 1];
                i++;
            } else {
                console.error("Error: -i option requires a file path");
                process.exit(1);
            }
        } else if (arg === "-o" || arg === "--output") {
            if (i + 1 < process.argv.length) {
                outputDir = process.argv[i + 1];
                i++;
            } else {
                console.error("Error: -o option requires a directory path");
                process.exit(1);
            }
        } else if (arg === "-h" || arg === "--help") {
            console.log("Usage: generate-svg.mjs [-i <input-file>] [-o <output-dir>]");
            console.log("");
            console.log("Options:");
            console.log("  -i, --input <file>    Input iconscript file (default: main.iconscript)");
            console.log("  -o, --output <dir>   Output directory for SVG files (default: output)");
            console.log("  -h, --help            Show this help message");
            process.exit(0);
        } else if (!arg.startsWith("-")) {
            // Backward compatibility: treat non-option argument as input file.
            inputFile = arg;
        }
    }

    return {inputFile, outputDir};
}

// Run the generator.
const {inputFile, outputDir} = parseArgs();
generateIcons(inputFile, outputDir);

// Export for testing
export {parseIconsFile};
