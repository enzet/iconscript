import fs from "fs";
import path from "path";

// Import parser and generator from parser.ts (shared code).
import {parseIconsFile} from "../parser.js";

const VERSION = "0.4.2";

/**
 * Parse version string into [major, minor, patch] tuple.
 *
 * Minor and patch default to 0 if not provided.
 */
function parseVersion(versionStr: string): [number, number, number] | null {
    const parts = versionStr.split(".");
    if (parts.length === 0 || parts.length > 3) {
        return null;
    }
    const major = parseInt(parts[0], 10);
    if (isNaN(major)) {
        return null;
    }
    const minor = parts[1] !== undefined ? parseInt(parts[1], 10) : 0;
    const patch = parts[2] !== undefined ? parseInt(parts[2], 10) : 0;
    if (isNaN(minor) || isNaN(patch)) {
        return null;
    }
    return [major, minor, patch];
}

/**
 * Compare two versions. Returns:
 *   - negative if a < b,
 *   - 0 if a === b,
 *   - positive if a > b.
 */
function compareVersions(
    a: [number, number, number],
    b: [number, number, number],
): number {
    if (a[0] !== b[0]) return a[0] - b[0];
    if (a[1] !== b[1]) return a[1] - b[1];
    return a[2] - b[2];
}

/**
 * Check if the file version is compatible with the tool version.
 */
function checkVersionCompatibility(content: string): void {
    const firstLine = content.split("\n")[0] || "";
    const prefix = "# iconscript ";

    if (!firstLine.startsWith(prefix)) {
        return;
    }

    const fileVersionStr = firstLine.slice(prefix.length).trim();
    if (fileVersionStr === "") {
        return;
    }

    const fileVersion = parseVersion(fileVersionStr);
    if (fileVersion === null) {
        return; // Invalid version format, proceed normally.
    }

    const toolVersion = parseVersion(VERSION);
    if (toolVersion === null) {
        throw new Error("Invalid tool version.");
    }

    if (compareVersions(fileVersion, toolVersion) > 0) {
        throw new Error(
            `File requires iconscript version ${fileVersionStr}, but this is ` +
                `version ${VERSION}`,
        );
    }
}

function generateIcons(
    inputFile: string = "main.iconscript",
    outputDir: string = "output",
): void {
    try {
        // Read the specified file or default to `main.iconscript`.
        const iconsContent = fs.readFileSync(inputFile, "utf8");

        // Check version compatibility.
        checkVersionCompatibility(iconsContent);

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
                console.log(`Generated: ${filename}.`);
                iconCount++;
            }
        }

        console.log(
            `\nGenerated ${iconCount} SVG files in the \`${outputDir}\` ` +
                `directory.`,
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

// Export for testing.
export {parseIconsFile};
