/**
 * Browser-compatible entry point for IconScript parser.
 * This file exports only the functions needed for browser use,
 * excluding Node.js-specific code (file system, command-line args).
 */

import antlr4 from "antlr4";
import paper from "paper";

// Import the generated parser (ES6 modules).
import IconScriptLexer from "./grammar/IconScriptLexer.js";
import IconScriptParser from "./grammar/IconScriptParser.js";
import GeneratedIconScriptListener from "./grammar/IconScriptListener.js";

const scale = 1.0;
const defaultWidth = 1.0;

class Point {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }

    add(other) {
        return new Point(this.x + other.x, this.y + other.y);
    }
}

/**
 * Combine paths using Paper.js.
 *
 * @param {array} paths paths to combine (union or subtract)
 * @param {array} modes modes to apply to paths (true for add, false for remove)
 * @returns {string} combined path
 */
function unionPathsWithModes(paths, modes) {

    if (paths.length === 0) return null;
    if (paths.length === 1) return paths[0];

    // Initialize Paper.js.
    paper.setup(new paper.Size(100, 100));

    try {
        // Convert SVG paths to Paper.js paths.
        const paperPaths = paths
            .map(pathData => {
                try {
                    return new paper.Path(pathData);
                } catch (e) {
                    console.warn("Failed to parse path:", pathData, e.message);
                    return null;
                }
            })
            .filter(path => path !== null);

        if (paperPaths.length === 0) return null;
        if (paperPaths.length === 1) return paperPaths[0].pathData;

        // Perform operations based on individual path modes.
        let result = paperPaths[0];
        for (let i = 1; i < paperPaths.length; i++) {
            try {
                if (modes[i]) {
                    result = result.unite(paperPaths[i]);
                } else {
                    result = result.subtract(paperPaths[i]);
                }
            } catch (e) {
                console.warn(`Operation failed for path ${i}:`, e.message);
                continue;
            }
        }
        const combinedPathData = result.pathData;

        // Clean up Paper.js objects.
        paperPaths.forEach(path => path.remove());
        result.remove();

        return combinedPathData;
    } catch (e) {
        console.warn("Path operation failed:", e.message);
        return paths.join(" ");
    }
}

/**
 * Create a thick line path.
 */
function createThickLinePath(x1, y1, x2, y2, thickness) {

    // Create a thick line by offsetting the line perpendicular to its direction.
    const dx = x2 - x1;
    const dy = y2 - y1;
    const length = Math.sqrt(dx * dx + dy * dy);

    if (length === 0) return null;

    // Normalize the direction vector.
    const nx = dx / length;
    const ny = dy / length;

    // Perpendicular vector.
    const px = -ny;
    const py = nx;

    // Create a rectangle path.
    const halfThickness = thickness / 2;
    const x1a = x1 + px * halfThickness;
    const y1a = y1 + py * halfThickness;
    const x1b = x1 - px * halfThickness;
    const y1b = y1 - py * halfThickness;
    const x2a = x2 + px * halfThickness;
    const y2a = y2 + py * halfThickness;
    const x2b = x2 - px * halfThickness;
    const y2b = y2 - py * halfThickness;

    return (
        `M ${x1a} ${y1a} L ${x2a} ${y2a} L ${x2b} ${y2b} ` +
        `L ${x1b} ${y1b} Z`
    );
}

/**
 * Create a circle path.
 */
function createCirclePath(cx, cy, r) {
    if (isNaN(cx) || isNaN(cy) || isNaN(r)) {
        console.warn("Invalid circle coordinates:", cx, cy, r);
        return null;
    }

    return (
        `M ${cx - r} ${cy} A ${r} ${r} 0 0 1 ${cx + r} ${cy} ` +
        `A ${r} ${r} 0 0 1 ${cx - r} ${cy} Z`
    );
}

class Scope {
    constructor(uniting = true, width = defaultWidth, position = new Point(0, 0)) {
        this.uniting = uniting;
        this.width = width;
        this.position = position;
    }

    getPosition(pos) {
        const x = parseFloat(pos.x.text);
        const y = parseFloat(pos.y.text);

        let position;
        if (pos.relative) {
            position = this.position.add(new Point(x, y));
        } else {
            position = new Point(x + 0.5, y + 0.5);
        }
        this.position = position;

        return position;
    }

    deepCopy() {
        return new Scope(
            this.uniting,
            this.width,
            new Point(this.position.x, this.position.y),
        );
    }
}

// Custom listener to generate SVG directly from AST.
class IconScriptListener extends GeneratedIconScriptListener {
    constructor() {
        super();
        this.variables = {};
        this.icons = [];
        this.currentIcon = null;
        this.paths = [];
        this.modes = [];

        this.scopes = [new Scope()];
    }

    getScope() {
        return this.scopes[this.scopes.length - 1];
    }

    // Enter a parse tree produced by IconScriptParser#assignment.
    enterAssignment(ctx) {
        const varName = ctx.left.text;
        // Store variable commands for later expansion.
        this.variables[varName] = ctx.right;
    }

    // Enter a parse tree produced by IconScriptParser#icon.
    enterIcon(ctx) {
        this.currentIcon = {
            name: null,
            svg: "",
        };
        this.paths = [];
        this.modes = [];
        this.scopes = [new Scope()];
    }

    enterScope(ctx) {
        this.scopes.push(this.scopes[this.scopes.length - 1].deepCopy());
    }

    exitScope(ctx) {
        this.scopes.pop();
    }

    // Exit a parse tree produced by IconScriptParser#icon.
    exitIcon(ctx) {
        if (this.currentIcon) {
            // Combine all paths into a single SVG path.
            const combinedPath = unionPathsWithModes(this.paths, this.modes);

            if (combinedPath) {
                this.currentIcon.svg =
                    `<?xml version="1.0" encoding="utf-8" ?>` +
                    `<svg baseProfile="tiny" ` +
                    `height="16px" version="1.2" width="16px" viewBox="0 0 16 16" ` +
                    `xmlns="http://www.w3.org/2000/svg" ` +
                    `xmlns:ev="http://www.w3.org/2001/xml-events" ` +
                    `xmlns:xlink="http://www.w3.org/1999/xlink"><defs />` +
                    `<path d="${combinedPath}" fill="black" stroke="none" /></svg>`;
            }

            this.icons.push(this.currentIcon);
            this.currentIcon = null;
        }
    }

    enterName(ctx) {
        if (this.currentIcon) {
            // Extract from the full text (remove the % prefix).
            const fullText = ctx.getText();
            const name = fullText.startsWith("%")
                ? fullText.slice(1)
                : fullText;

            this.currentIcon.name = name;
        }
    }

    // Exit a parse tree produced by IconScriptParser#line.
    exitLine(ctx) {
        const isFilled = ctx.getText().includes("lf");
        const positions = ctx.position();
        const coordinates = [];

        for (const pos of positions) {
            coordinates.push(this.getScope().getPosition(pos));
        }

        if (coordinates.length === 0) return;

        // Add circles at all points.
        for (const coord of coordinates) {
            const circlePath = createCirclePath(coord.x, coord.y, this.getScope().width / 2);
            if (circlePath) {
                this.paths.push(circlePath);
                this.modes.push(this.getScope().uniting);
            }
        }

        // Add lines between consecutive points.
        for (let i = 0; i < coordinates.length - 1; i++) {
            const from = coordinates[i];
            const to = coordinates[i + 1];
            const linePath = createThickLinePath(
                from.x, from.y, to.x, to.y, this.getScope().width
            );
            if (linePath) {
                this.paths.push(linePath);
                this.modes.push(this.getScope().uniting);
            }
        }

        // If filled, add a filled polyline.
        if (isFilled && coordinates.length >= 2) {
            let filledPath = `M ${coordinates[0].x} ${coordinates[0].y}`;
            for (let i = 1; i < coordinates.length; i++) {
                filledPath += ` L ${coordinates[i].x} ${coordinates[i].y}`;
            }
            filledPath += " Z";
            this.paths.push(filledPath);
            this.modes.push(this.getScope().uniting);
        }
    }

    // Exit a parse tree produced by IconScriptParser#circle.
    exitCircle(ctx) {
        const center = this.getScope().getPosition(ctx.position());
        const radius = parseFloat(ctx.FLOAT().getText());
        const circlePath = createCirclePath(center.x, center.y, radius / 2);

        if (circlePath) {
            this.paths.push(circlePath);
            this.modes.push(this.getScope().uniting);
        }
    }

    // Exit a parse tree produced by IconScriptParser#arc.
    exitArc(ctx) {
        const pos = ctx.position();
        const x = parseFloat(pos.x.text);
        const y = parseFloat(pos.y.text);
        let center;

        if (pos.relative) {
            center = this.currentPoint.add(new Point(x, y));
        } else {
            center = new Point(x, y);
        }

        // Convert to center coordinates.
        center = center.add(new Point(0.5, 0.5));
        this.currentPoint = center.add(new Point(-0.5, -0.5));

        const radius = parseFloat(ctx.FLOAT(0).getText());
        const startAngle = parseFloat(ctx.FLOAT(1).getText());
        const endAngle = parseFloat(ctx.FLOAT(2).getText());

        // Create multiple line segments to approximate the arc.
        const segments = 10;
        const angleStep = (endAngle - startAngle) / segments;

        let currentAngle = startAngle;
        let lastPoint = this.arcPoint(center, currentAngle, radius * scale);

        for (let i = 1; i <= segments; i++) {
            currentAngle += angleStep;
            const currentPoint = this.arcPoint(center, currentAngle, radius * scale);
            const linePath = createThickLinePath(
                lastPoint.x,
                lastPoint.y,
                currentPoint.x,
                currentPoint.y,
                this.getScope().width
            );
            if (linePath) {
                this.paths.push(linePath);
                this.modes.push(this.getScope().uniting);
            }
            lastPoint = currentPoint;
        }
    }

    arcPoint(center, angle, radius) {
        return new Point(
            center.x + Math.cos(angle) * radius,
            center.y - Math.sin(angle) * radius
        );
    }

    // Exit a parse tree produced by IconScriptParser#rectangle.
    exitRectangle(ctx) {
        const point1 = this.getScope().getPosition(ctx.position(0));
        const point2 = this.getScope().getPosition(ctx.position(1));
        const p1 = new Point(point2.x, point1.y);
        const p2 = new Point(point1.x, point2.y);

        // Add circles at all four corners.
        const corners = [point1, p1, point2, p2];
        for (const corner of corners) {
            const circlePath = createCirclePath(
                corner.x, corner.y, this.getScope().width / 2);
            if (circlePath) {
                this.paths.push(circlePath);
                this.modes.push(this.getScope().uniting);
            }
        }
        const halfWidth = this.getScope().width / 2;

        // Add filled rectangle as a filled polyline.
        const rectanglePath1 =
            `M ${point1.x - halfWidth} ${point1.y} L ${p1.x + halfWidth} ${p1.y} ` +
            `L ${point2.x + halfWidth} ${point2.y} L ${p2.x - halfWidth} ${p2.y} Z`;
        const rectanglePath2 =
            `M ${point1.x} ${point1.y - halfWidth} L ${p1.x} ${p1.y - halfWidth} ` +
            `L ${point2.x} ${point2.y + halfWidth} L ${p2.x} ${p2.y + halfWidth} Z`;

        this.paths.push(rectanglePath1);
        this.modes.push(this.getScope().uniting);
        this.paths.push(rectanglePath2);
        this.modes.push(this.getScope().uniting);
    }

    exitSetPosition(ctx) {
        this.getScope().getPosition(ctx.position());
    }

    exitSetWidth(ctx) {
        this.getScope().width = parseFloat(ctx.FLOAT().getText());
    }

    exitSetRemove(ctx) {
        this.getScope().uniting = false;
    }

    enterCommand(ctx) {
        if (ctx.VARIABLE()) {
            // Variable reference - expand it by walking its parse tree.
            const varName = ctx.getText().slice(1); // Remove `@`.
            const varCommands = this.variables[varName];
            if (varCommands) {
                // Walk the variable commands to trigger exit methods.
                antlr4.tree.ParseTreeWalker.DEFAULT.walk(this, varCommands);
            }
        }
    }
}

function parseIconsFile(content) {
    const input = new antlr4.InputStream(content);
    const lexer = new IconScriptLexer(input);
    const stream = new antlr4.CommonTokenStream(lexer);
    const parser = new IconScriptParser(stream);

    // Add error listener.
    parser.removeErrorListeners();
    parser.addErrorListener({
        syntaxError: (recognizer, offendingSymbol, line, column, msg, e) => {
            console.error(`Syntax error at line ${line}:${column} - ${msg}`);
        },
    });

    const tree = parser.script();

    const listener = new IconScriptListener();
    antlr4.tree.ParseTreeWalker.DEFAULT.walk(listener, tree);

    return listener.icons;
}

export {parseIconsFile};
