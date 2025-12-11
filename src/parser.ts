/**
 * Browser-compatible entry point for IconScript parser.
 * This file exports only the functions needed for browser use,
 * excluding Node.js-specific code (file system, command-line args).
 */

import * as antlr4 from "antlr4";
import {ParseTreeWalker} from "antlr4";
import paper from "paper";

// Import the generated parser (ES6 modules).
import IconScriptLexer from "../grammar/IconScriptLexer";
import IconScriptParser from "../grammar/IconScriptParser";
import GeneratedIconScriptListener from "../grammar/IconScriptListener";
import type {Icon} from "./types.js";
import type {
    AssignmentContext,
    IconContext,
    NameContext,
    LineContext,
    CircleContext,
    ArcContext,
    RectangleContext,
    SetPositionContext,
    SetWidthContext,
    SetRemoveContext,
    CommandContext,
    ScopeContext,
    PositionContext,
    CommandsContext,
} from "../grammar/IconScriptParser.js";

const scale = 1.0;
const defaultWidth = 1.0;

class Point {
    x: number;
    y: number;

    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }

    add(other: Point): Point {
        return new Point(this.x + other.x, this.y + other.y);
    }
}

/**
 * Combine paths using Paper.js.
 *
 * @param paths paths to combine (union or subtract)
 * @param modes modes to apply to paths (true for add, false for remove)
 * @returns combined path
 */
function unionPathsWithModes(paths: string[], modes: boolean[]): string | null {
    if (paths.length === 0) return null;
    if (paths.length === 1) return paths[0];

    // Initialize Paper.js.
    paper.setup(new paper.Size(100, 100));

    try {
        // Convert SVG paths to Paper.js paths.
        const paperPaths = paths
            .map(pathData => {
                try {
                    return new paper.Path(pathData) as paper.Path;
                } catch (e) {
                    const errorMessage =
                        e instanceof Error ? e.message : String(e);
                    console.warn(
                        "Failed to parse path:",
                        pathData,
                        errorMessage
                    );
                    return null;
                }
            })
            .filter((path): path is paper.Path => path !== null);

        if (paperPaths.length === 0) return null;
        if (paperPaths.length === 1) return paperPaths[0].pathData;

        // Perform operations based on individual path modes.
        let result: paper.Path = paperPaths[0];
        for (let i = 1; i < paperPaths.length; i++) {
            try {
                if (modes[i]) {
                    const united = result.unite(paperPaths[i]);
                    result = united as paper.Path;
                } else {
                    const subtracted = result.subtract(paperPaths[i]);
                    result = subtracted as paper.Path;
                }
            } catch (e) {
                const errorMessage = e instanceof Error ? e.message : String(e);
                console.warn(`Operation failed for path ${i}:`, errorMessage);
                continue;
            }
        }
        const combinedPathData = result.pathData;

        // Clean up Paper.js objects.
        paperPaths.forEach(path => path.remove());
        result.remove();

        return combinedPathData;
    } catch (e) {
        const errorMessage = e instanceof Error ? e.message : String(e);
        console.warn("Path operation failed:", errorMessage);
        return paths.join(" ");
    }
}

/**
 * Create a thick line path.
 */
function createThickLinePath(
    x1: number,
    y1: number,
    x2: number,
    y2: number,
    thickness: number
): string | null {
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

    return `M ${x1a} ${y1a} L ${x2a} ${y2a} L ${x2b} ${y2b} L ${x1b} ${y1b} Z`;
}

/**
 * Create a circle path.
 */
function createCirclePath(cx: number, cy: number, r: number): string | null {
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
    uniting: boolean;
    width: number;
    position: Point;

    constructor(
        uniting = true,
        width = defaultWidth,
        position = new Point(0, 0)
    ) {
        this.uniting = uniting;
        this.width = width;
        this.position = position;
    }

    getPosition(pos: PositionContext): Point {
        const x = parseFloat(pos._x.text);
        const y = parseFloat(pos._y.text);

        let position: Point;
        if (pos._relative) {
            position = this.position.add(new Point(x, y));
        } else {
            position = new Point(x + 0.5, y + 0.5);
        }
        this.position = position;

        return position;
    }

    deepCopy(): Scope {
        return new Scope(
            this.uniting,
            this.width,
            new Point(this.position.x, this.position.y)
        );
    }
}

// Custom listener to generate SVG directly from AST.
class IconScriptListener extends GeneratedIconScriptListener {
    variables: Record<string, CommandsContext>;
    icons: Icon[];
    currentIcon: Icon | null;
    paths: string[];
    modes: boolean[];
    scopes: Scope[];
    currentPoint: Point;

    constructor() {
        super();
        this.variables = {};
        this.icons = [];
        this.currentIcon = null;
        this.paths = [];
        this.modes = [];
        this.currentPoint = new Point(0, 0);

        this.scopes = [new Scope()];
    }

    getScope(): Scope {
        return this.scopes[this.scopes.length - 1];
    }

    // Enter a parse tree produced by IconScriptParser#assignment.
    enterAssignment = (ctx: AssignmentContext): void => {
        const varName = ctx._left.text;
        // Store variable commands for later expansion.
        this.variables[varName] = ctx._right;
    };

    // Enter a parse tree produced by IconScriptParser#icon.
    enterIcon = (_ctx: IconContext): void => {
        this.currentIcon = {
            name: null,
            svg: "",
        };
        this.paths = [];
        this.modes = [];
        this.scopes = [new Scope()];
    };

    enterScope = (_ctx: ScopeContext): void => {
        this.scopes.push(this.scopes[this.scopes.length - 1].deepCopy());
    };

    exitScope = (_ctx: ScopeContext): void => {
        this.scopes.pop();
    };

    // Exit a parse tree produced by IconScriptParser#icon.
    exitIcon = (_ctx: IconContext): void => {
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
    };

    enterName = (ctx: NameContext): void => {
        if (this.currentIcon) {
            // Extract from the full text (remove the % prefix).
            const fullText = ctx.getText();
            const name = fullText.startsWith("%")
                ? fullText.slice(1)
                : fullText;

            this.currentIcon.name = name;
        }
    };

    // Exit a parse tree produced by IconScriptParser#line.
    exitLine = (ctx: LineContext): void => {
        const isFilled = ctx.getText().includes("lf");
        const positions = ctx.position_list();
        const coordinates: Point[] = [];

        for (const pos of positions) {
            coordinates.push(this.getScope().getPosition(pos));
        }

        if (coordinates.length === 0) return;

        // Add circles at all points.
        for (const coord of coordinates) {
            const circlePath = createCirclePath(
                coord.x,
                coord.y,
                this.getScope().width / 2
            );
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
                from.x,
                from.y,
                to.x,
                to.y,
                this.getScope().width
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
    };

    // Exit a parse tree produced by IconScriptParser#circle.
    exitCircle = (ctx: CircleContext): void => {
        const center = this.getScope().getPosition(ctx.position());
        const radius = parseFloat(ctx.FLOAT().getText());
        const circlePath = createCirclePath(center.x, center.y, radius / 2);

        if (circlePath) {
            this.paths.push(circlePath);
            this.modes.push(this.getScope().uniting);
        }
    };

    // Exit a parse tree produced by IconScriptParser#arc.
    exitArc = (ctx: ArcContext): void => {
        const pos = ctx.position();
        const x = parseFloat(pos._x.text);
        const y = parseFloat(pos._y.text);
        let center: Point;

        if (pos._relative) {
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
            const currentPoint = this.arcPoint(
                center,
                currentAngle,
                radius * scale
            );
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
    };

    arcPoint(center: Point, angle: number, radius: number): Point {
        return new Point(
            center.x + Math.cos(angle) * radius,
            center.y - Math.sin(angle) * radius
        );
    }

    // Exit a parse tree produced by IconScriptParser#rectangle.
    exitRectangle = (ctx: RectangleContext): void => {
        const point1 = this.getScope().getPosition(ctx.position(0));
        const point2 = this.getScope().getPosition(ctx.position(1));
        const p1 = new Point(point2.x, point1.y);
        const p2 = new Point(point1.x, point2.y);

        // Add circles at all four corners.
        const corners = [point1, p1, point2, p2];
        for (const corner of corners) {
            const circlePath = createCirclePath(
                corner.x,
                corner.y,
                this.getScope().width / 2
            );
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
    };

    exitSetPosition = (ctx: SetPositionContext): void => {
        this.getScope().getPosition(ctx.position());
    };

    exitSetWidth = (ctx: SetWidthContext): void => {
        this.getScope().width = parseFloat(ctx.FLOAT().getText());
    };

    exitSetRemove = (_ctx: SetRemoveContext): void => {
        this.getScope().uniting = false;
    };

    enterCommand = (ctx: CommandContext): void => {
        if (ctx.VARIABLE()) {
            // Variable reference - expand it by walking its parse tree.
            const varName = ctx.getText().slice(1); // Remove `@`.
            const varCommands = this.variables[varName];
            if (varCommands) {
                // Walk the variable commands to trigger exit methods.
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                ParseTreeWalker.DEFAULT.walk(this as any, varCommands);
            }
        }
    };
}

function parseIconsFile(content: string): Icon[] {
    const input = new antlr4.InputStream(content) as antlr4.CharStream;
    const lexer = new IconScriptLexer(input);
    const stream = new antlr4.CommonTokenStream(lexer);
    const parser = new IconScriptParser(stream);

    // Add error listener.
    parser.removeErrorListeners();
    parser.addErrorListener({
        syntaxError: (
            _recognizer: unknown,
            _offendingSymbol: unknown,
            line: number,
            column: number,
            msg: string,
            _e: unknown
        ) => {
            console.error(`Syntax error at line ${line}:${column} - ${msg}`);
        },
    });

    const tree = parser.script();

    const listener = new IconScriptListener();
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    ParseTreeWalker.DEFAULT.walk(listener as any, tree);

    return listener.icons;
}

export {parseIconsFile};
