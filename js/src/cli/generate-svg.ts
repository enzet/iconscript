import fs from "fs";
import path from "path";

// Import parser and generator from parser.ts (shared code).
import {parseIconsFile} from "../parser.js";

const VERSION = "0.3.0";

function generateIcons(
    inputFile: string = "main.iconscript",
    outputDir: string = "output",
): void {
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
                let filename: string;
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
            `\nGenerated ${iconCount} SVG files in the \`${outputDir}\` directory.`,
        );
    } catch (error) {
        const errorMessage =
            error instanceof Error ? error.message : String(error);
        console.error("Error:", errorMessage);
        process.exit(1);
    }
}

const args = process.argv.slice(2);

if (args.includes("-v") || args.includes("--version")) {
    console.log(`iconscript ${VERSION}`);
    process.exit(0);
}

const inputFile = args[0];
const outputDir = args[1];

generateIcons(inputFile, outputDir);

// Export for testing
export {parseIconsFile};
