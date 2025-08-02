#!/usr/bin/env node

import fs from "fs";
import path from "path";
import antlr4 from "antlr4";
import paper from "paper";

// Import the generated parser (ES6 modules)
import IconScriptLexer from "./grammar/IconScriptLexer.js";
import IconScriptParser from "./grammar/IconScriptParser.js";

// Configuration
const scale = 1.0;
const width = 1.0;

/*
 * Simple 2D point.
 */
class Point {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }

    add(other) {
        return new Point(this.x + other.x, this.y + other.y);
    }

    multiply(factor) {
        return new Point(this.x * factor, this.y * factor);
    }

    toString() {
        return `${this.x},${this.y}`;
    }
}

class SVGPath {
    constructor() {
        this.currentPath = "";
        this.currentPoint = new Point(0, 0);
    }

    moveTo(point) {
        this.currentPoint = point;
        this.currentPath += `M ${point.x},${point.y} `;
    }

    lineTo(point) {
        this.currentPoint = point;
        this.currentPath += `L ${point.x},${point.y} `;
    }

    arcTo(center, radius, startAngle, endAngle) {
        const startPoint = this.arcPoint(center, startAngle, radius);
        const endPoint = this.arcPoint(center, endAngle, radius);
        
        // Determine if we need to draw the large arc.
        const largeArcFlag = Math.abs(endAngle - startAngle) > Math.PI ? 1 : 0;
        
        // Determine sweep flag (direction).
        const sweepFlag = endAngle > startAngle ? 1 : 0;
        
        this.currentPath += `A ${radius},${radius} 0 ${largeArcFlag} ${sweepFlag} ${endPoint.x},${endPoint.y} `;
        this.currentPoint = endPoint;
    }

    arcPoint(center, angle, radius) {
        return new Point(
            center.x + Math.cos(angle) * radius * scale,
            center.y - Math.sin(angle) * radius * scale
        );
    }

    closePath() {
        this.currentPath += "Z ";
    }

    getPathData() {
        return this.currentPath.trim();
    }
}

class IconGenerator {
    constructor() {
        this.variables = {};
        this.currentPoint = new Point(0, 0);
        this.width = width;
        this.uniting = true;
        this.shape = null;
        this.fill = null;
        this.filled = false;
        this.elements = [];
    }

    toCoordinates(position) {
        if (position.startsWith("+")) {
            const coords = position.slice(1).split(",");
            const p = new Point(Number(coords[0]), Number(coords[1]));
            this.currentPoint = this.currentPoint.add(p);
        } else {
            const coords = position.split(",");
            const p = new Point(Number(coords[0]), Number(coords[1]));
            this.currentPoint = p;
        }
        return this.currentPoint.add(new Point(0.5, 0.5));
    }

    addPoint(position, diameter) {
        const center = position;
        const r = diameter / 2; // Convert diameter to radius
        
        this.elements.push({
            type: 'circle',
            center: center,
            radius: r,
            mode: this.uniting
        });
    }

    addLine(from, to) {
        this.elements.push({
            type: 'line',
            from: from,
            to: to,
            strokeWidth: this.width || 1,
            mode: this.uniting
        });
    }

    addFilledPolyline(positions) {

        const points = positions.map(pos => {
            const coords = this.toCoordinates(pos);
            return { x: coords.x, y: coords.y };
        });
        
        if (points.length >= 2) {
            this.elements.push({
                type: 'filledPolyline',
                points: points,
                mode: this.uniting
            });
        }
    }

    addFilledPolylineFromCoordinates(coordinates) {

        if (coordinates.length >= 2) {
            this.elements.push({
                type: 'filledPolyline',
                points: coordinates,
                mode: this.uniting
            });
        }
    }

    lineToPath(lineData) {
        // Extract coordinates from line JavaScript structure
        const x1 = lineData.from.x;
        const y1 = lineData.from.y;
        const x2 = lineData.to.x;
        const y2 = lineData.to.y;
        const strokeWidth = lineData.strokeWidth;
        
        // Create a thick line path by offsetting the line
        return this.createThickLinePath(x1, y1, x2, y2, strokeWidth);
    }

    circleToPath(circleData) {
        // Extract coordinates from circle JavaScript structure
        const cx = circleData.center.x;
        const cy = circleData.center.y;
        const r = circleData.radius;
        
        if (isNaN(cx) || isNaN(cy) || isNaN(r)) {
            console.warn('Invalid circle coordinates:', cx, cy, r);
            return null;
        }
        
        // Create a circle path
        return `M ${cx - r} ${cy} A ${r} ${r} 0 0 1 ${cx + r} ${cy} A ${r} ${r} 0 0 1 ${cx - r} ${cy} Z`;
    }

    createThickLinePath(x1, y1, x2, y2, thickness) {
        // Create a thick line by offsetting the line perpendicular to its direction
        const dx = x2 - x1;
        const dy = y2 - y1;
        const length = Math.sqrt(dx * dx + dy * dy);
        
        if (length === 0) return null;
        
        // Normalize the direction vector
        const nx = dx / length;
        const ny = dy / length;
        
        // Perpendicular vector
        const px = -ny;
        const py = nx;
        
        // Create a rectangle path
        const halfThickness = thickness / 2;
        const x1a = x1 + px * halfThickness;
        const y1a = y1 + py * halfThickness;
        const x1b = x1 - px * halfThickness;
        const y1b = y1 - py * halfThickness;
        const x2a = x2 + px * halfThickness;
        const y2a = y2 + py * halfThickness;
        const x2b = x2 - px * halfThickness;
        const y2b = y2 - py * halfThickness;
        
        // Create a simpler path that might work better with subtraction
        return `M ${x1a} ${y1a} L ${x2a} ${y2a} L ${x2b} ${y2b} L ${x1b} ${y1b} Z`;
    }

    unionPaths(paths) {
        if (paths.length === 0) return null;
        if (paths.length === 1) return paths[0];
        
        // Initialize Paper.js
        paper.setup(new paper.Size(100, 100));
        
        try {
            // Convert SVG paths to Paper.js paths
            const paperPaths = paths.map(pathData => {
                try {
                    return new paper.Path(pathData);
                } catch (e) {
                    console.warn('Failed to parse path:', pathData, e.message);
                    return null;
                }
            }).filter(path => path !== null);
            
            if (paperPaths.length === 0) return null;
            if (paperPaths.length === 1) return paperPaths[0].pathData;
            
            // Perform operations based on current mode
            let result = paperPaths[0];
            for (let i = 1; i < paperPaths.length; i++) {
                if (this.uniting) {
                    // Add mode - use union operation
                    result = result.unite(paperPaths[i]);
                } else {
                    // Remove mode - use difference operation
                    result = result.subtract(paperPaths[i]);
                }
            }
            
            // Get the combined path data
            const combinedPathData = result.pathData;
            
            // Clean up Paper.js objects
            paperPaths.forEach(path => path.remove());
            result.remove();
            
            return combinedPathData;
            
        } catch (e) {
            console.warn('Path operation failed:', e.message);
            // Fallback to concatenation
            return paths.join(' ');
        }
    }

    /*
     * paths: array of paths
     * modes: array of booleans (true for add, false for remove)
     *
     * returns: combined path
     */
    unionPathsWithModes(paths, modes) {
        if (paths.length === 0) return null;
        if (paths.length === 1) return paths[0];
        
        // Initialize Paper.js
        paper.setup(new paper.Size(100, 100));

        try {
            // Convert SVG paths to Paper.js paths
            const paperPaths = paths.map(pathData => {
                try {
                    return new paper.Path(pathData);
                } catch (e) {
                    console.warn('Failed to parse path:', pathData, e.message);
                    return null;
                }
            }).filter(path => path !== null);
            
            if (paperPaths.length === 0) return null;
            if (paperPaths.length === 1) return paperPaths[0].pathData;
            
            // Perform operations based on individual path modes
            let result = paperPaths[0];
            for (let i = 1; i < paperPaths.length; i++) {
                try {
                    if (modes[i]) {
                        // Add mode - use union operation
                        result = result.unite(paperPaths[i]);
                    } else {
                        // Remove mode - use difference operation
                        result = result.subtract(paperPaths[i]);
                    }
                } catch (e) {
                    console.warn(`Operation failed for path ${i}:`, e.message);
                    // Skip this path if the operation fails
                    continue;
                }
            }
            
            // Get the combined path data.
            const combinedPathData = result.pathData;
            
            // Clean up Paper.js objects.
            paperPaths.forEach(path => path.remove());
            result.remove();
            
            return combinedPathData;
            
        } catch (e) {
            console.warn('Path operation failed:', e.message);
            return paths.join(' ');
        }
    }

    addArc(center, r, p1, p2) {
        // Create multiple line segments to approximate the arc
        const segments = 10; // Number of line segments to approximate the arc
        const angleStep = (p2 - p1) / segments;
        
        let currentAngle = p1;
        let lastPoint = this.arcPoint(center, currentAngle, r);
        
        for (let i = 1; i <= segments; i++) {
            currentAngle += angleStep;
            const currentPoint = this.arcPoint(center, currentAngle, r);
            this.addLine(lastPoint, currentPoint);
            lastPoint = currentPoint;
        }
    }

    arcPoint(center, angle, radius) {
        return new Point(
            center.x + Math.cos(angle) * radius * scale,
            center.y - Math.sin(angle) * radius * scale
        );
    }

    addRectangle(from, to) {
        // Create the four corners of the rectangle
        const p1 = new Point(from.x, to.y);
        const p2 = new Point(to.x, from.y);
        
        // Add circles at all four corners
        this.addPoint(from, this.width || 1);
        this.addPoint(p1, this.width || 1);
        this.addPoint(to, this.width || 1);
        this.addPoint(p2, this.width || 1);
        
        // Add stroke lines to form the rectangle outline
        this.addLine(from, p1);
        this.addLine(p1, to);
        this.addLine(to, p2);
        this.addLine(p2, from);
        
        // Add filled rectangle as a filled polyline
        const rectanglePoints = [from, p1, to, p2, from];
        this.addFilledPolylineFromCoordinates(rectanglePoints);
    }

    combineFill() {
        if (this.fill) {
            const fillPath = this.fill.getPathData();
            this.shape = this.shape ? this.shape + " " + fillPath : fillPath;
            this.fill = null;
        }
    }

    generateSVG(iconName) {
        if (this.elements.length === 0) return null;

        // Combine all elements into a single path using union operations
        const combinedPath = this.combineAllElements();
        
        if (!combinedPath) return null;

        const svg = `<?xml version="1.0" encoding="utf-8" ?>
<svg baseProfile="tiny" height="16px" version="1.2" width="16px" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg" xmlns:ev="http://www.w3.org/2001/xml-events" xmlns:xlink="http://www.w3.org/1999/xlink"><defs /><path d="${combinedPath}" fill="black" stroke="none" /></svg>`;
        return svg;
    }

    combineAllElements() {
        const paths = [];
        const modes = [];
        
        // Process all elements in order
        for (const element of this.elements) {
            let path = null;
            
            if (element.type === 'line') {
                path = this.lineToPath(element);
            } else if (element.type === 'filledPolyline') {
                // Convert points to path string
                const points = element.points;
                if (points.length >= 2) {
                    path = `M ${points[0].x} ${points[0].y}`;
                    for (let i = 1; i < points.length; i++) {
                        path += ` L ${points[i].x} ${points[i].y}`;
                    }
                    path += ' Z'; // Close the path for filling
                }
            } else if (element.type === 'circle') {
                path = this.circleToPath(element);
            }
            
            if (path) {
                paths.push(path);
                modes.push(element.mode);
            }
        }
        
        if (paths.length === 0) return null;
        
        // Combine all paths using the tracked modes
        return this.unionPathsWithModes(paths, modes);
    }

    // Process commands from the parsed AST
    processCommands(commands) {
        this.elements = []; // Unified array for all elements
        this.fill = null;
        this.filled = false;
        this.currentPoint = new Point(0, 0);
        this.uniting = true;
        this.width = width;

        for (const command of commands) {
            this.processCommand(command);
        }

        return this.generateSVG();
    }

    processCommand(command) {
        if (command.type === 'line') {
            // Regular line - create individual line elements and circles on joints
            let last = null;
            for (const position of command.positions) {
                const coordinates = this.toCoordinates(position);
                this.addPoint(coordinates, this.width || 1);
                
                if (last) {
                    this.addLine(last, coordinates);
                }
                last = coordinates;
            }
        } else if (command.type === 'line_filled') {
            // Filled line - create filled polyline with stroke lines and circles at all joints (except last)
            const processedCoordinates = [];
            let last = null;
            
            for (const position of command.positions) {
                const coordinates = this.toCoordinates(position);
                processedCoordinates.push(coordinates);
                
                // Add stroke lines between points
                if (last) {
                    this.addLine(last, coordinates);
                }
                last = coordinates;
            }
            // Add the filled polyline
            this.addFilledPolylineFromCoordinates(processedCoordinates);
            
            // Add circles at all connection points except the last one
            for (let i = 0; i < processedCoordinates.length - 1; i++) {
                this.addPoint(processedCoordinates[i], this.width || 1);
            }
        } else if (command.type === 'line_single') {
            // Handle single line commands (l position)
            const coordinates = this.toCoordinates(command.position);
            this.addPoint(coordinates, this.width || 1);
            
            // Draw line from current position to the new position
            if (this.currentPoint) {
                this.addLine(this.currentPoint, coordinates);
            }
            this.currentPoint = coordinates;
        } else if (command.type === 'circle') {
            const center = this.toCoordinates(command.position);
            const radius = command.radius;
            this.addPoint(center, radius);
        } else if (command.type === 'arc') {
            const center = this.toCoordinates(command.position);
            const radius = command.radius;
            const p1 = command.startAngle;
            const p2 = command.endAngle;
            this.addArc(center, radius, p1, p2);
        } else if (command.type === 'rectangle') {
            const point1 = this.toCoordinates(command.position1);
            const point2 = this.toCoordinates(command.position2);
            this.addRectangle(point1, point2);
        } else if (command.type === 'setPosition') {
            // For setPosition, we want to store the raw coordinates without the 0.5 offset
            if (command.position.startsWith("+")) {
                const coords = command.position.slice(1).split(",");
                const p = new Point(Number(coords[0]), Number(coords[1]));
                this.currentPoint = this.currentPoint.add(p);
            } else {
                const coords = command.position.split(",");
                const p = new Point(Number(coords[0]), Number(coords[1]));
                this.currentPoint = p;
            }
        } else if (command.type === 'setWidth') {
            this.width = command.width;
        } else if (command.type === 'add') {
            this.combineFill();
            this.uniting = true;
        } else if (command.type === 'remove') {
            this.combineFill();
            this.uniting = false;
        }
    }
}

// Import the generated listener
import GeneratedIconScriptListener from './grammar/IconScriptListener.js';

// Custom listener to extract commands from the AST
class IconScriptListener extends GeneratedIconScriptListener {
    constructor() {
        super();
        this.variables = {};
        this.icons = [];
        this.currentIcon = null;
    }

    // Enter a parse tree produced by IconScriptParser#assignment.
    enterAssignment(ctx) {
        const varName = ctx.left.text;
        const commands = this.extractCommands(ctx.right);
        this.variables[varName] = commands;
    }

    // Enter a parse tree produced by IconScriptParser#icon.
    enterIcon(ctx) {
        this.currentIcon = {
            name: null,
            commands: []
        };
    }

    // Exit a parse tree produced by IconScriptParser#icon.
    exitIcon(ctx) {
        if (this.currentIcon) {
            // Process variables in the icon
            const processedCommands = this.processVariables(this.currentIcon.commands);
            this.icons.push({
                name: this.currentIcon.name,
                commands: processedCommands
            });
            console.log(`Icon completed - name: ${this.currentIcon.name}, commands: ${processedCommands.length}`);
            this.currentIcon = null;
        }
    }

    // Enter a parse tree produced by IconScriptParser#name.
    enterName(ctx) {
        if (this.currentIcon) {
            // Extract from the full text (remove the % prefix)
            const fullText = ctx.getText();
            const name = fullText.startsWith('%') ? fullText.slice(1) : fullText;
            
            this.currentIcon.name = name;
            console.log(`Setting icon name: ${this.currentIcon.name}`);
        }
    }

    // Enter a parse tree produced by IconScriptParser#command.
    enterCommand(ctx) {
        if (!this.currentIcon) return;

        if (ctx.name()) {
            // This is a name command (comment)
            return;
        }

        let command = null;

        if (ctx.VARIABLE()) {
            // Variable reference - will be processed later
            const varName = ctx.getText().slice(1); // Remove @
            command = { type: 'variable', name: varName };
        } else if (ctx.getText() === 'a') {
            command = { type: 'add' };
        } else if (ctx.getText() === 'r') {
            command = { type: 'remove' };
        } else if (ctx.line()) {
            command = this.processLineCommand(ctx.line());
        } else if (ctx.circle()) {
            command = this.processCircleCommand(ctx.circle());
        } else if (ctx.arc()) {
            command = this.processArcCommand(ctx.arc());
        } else if (ctx.rectangle()) {
            command = this.processRectangleCommand(ctx.rectangle());
        } else if (ctx.setPosition()) {
            command = this.processSetPositionCommand(ctx.setPosition());
        } else if (ctx.setWidth()) {
            command = this.processSetWidthCommand(ctx.setWidth());
        }

        if (command) {
            this.currentIcon.commands.push(command);
        }
    }

    processLineCommand(ctx) {
        const isFilled = ctx.getText().includes('lf');
        const positions = [];
        
        for (const pos of ctx.position()) {
            const relative = pos.relative ? '+' : '';
            const x = pos.x.text;
            const y = pos.y.text;
            positions.push(`${relative}${x},${y}`);
        }

        // If there's only one position, treat it as a single line command
        if (positions.length === 1) {
            return {
                type: 'line_single',
                position: positions[0]
            };
        }

        return {
            type: isFilled ? 'line_filled' : 'line',
            positions: positions
        };
    }

    processCircleCommand(ctx) {
        const pos = ctx.position();
        const relative = pos.relative ? '+' : '';
        const x = pos.x.text;
        const y = pos.y.text;
        const position = `${relative}${x},${y}`;
        
        // Get the radius from the FLOAT token
        const floatToken = ctx.FLOAT();
        const radius = parseFloat(floatToken ? floatToken.getText() : '0');

        return {
            type: 'circle',
            position: position,
            radius: radius
        };
    }

    processArcCommand(ctx) {
        const pos = ctx.position();
        const relative = pos.relative ? '+' : '';
        const x = pos.x.text;
        const y = pos.y.text;
        const position = `${relative}${x},${y}`;
        const radius = parseFloat(ctx.FLOAT(0).text);
        const startAngle = parseFloat(ctx.FLOAT(1).text);
        const endAngle = parseFloat(ctx.FLOAT(2).text);

        return {
            type: 'arc',
            position: position,
            radius: radius,
            startAngle: startAngle,
            endAngle: endAngle
        };
    }

    processRectangleCommand(ctx) {
        const pos1 = ctx.position(0);
        const pos2 = ctx.position(1);
        
        const relative1 = pos1.relative ? '+' : '';
        const x1 = pos1.x.text;
        const y1 = pos1.y.text;
        const position1 = `${relative1}${x1},${y1}`;
        
        const relative2 = pos2.relative ? '+' : '';
        const x2 = pos2.x.text;
        const y2 = pos2.y.text;
        const position2 = `${relative2}${x2},${y2}`;

        return {
            type: 'rectangle',
            position1: position1,
            position2: position2
        };
    }

    processSetPositionCommand(ctx) {
        const pos = ctx.position();
        const relative = pos.relative ? '+' : '';
        const x = pos.x.text;
        const y = pos.y.text;
        const position = `${relative}${x},${y}`;

        return {
            type: 'setPosition',
            position: position
        };
    }

    processSetWidthCommand(ctx) {
        const width = parseFloat(ctx.FLOAT().getText());
        return {
            type: 'setWidth',
            width: width
        };
    }

    extractCommands(ctx) {
        const commands = [];
        for (const command of ctx.command()) {
            // Process each command similar to enterCommand
            let processedCommand = null;
            
            if (command.VARIABLE()) {
                const varName = command.getText().slice(1); // Remove @
                processedCommand = { type: 'variable', name: varName };
            } else if (command.getText() === 'a') {
                processedCommand = { type: 'add' };
            } else if (command.getText() === 'r') {
                processedCommand = { type: 'remove' };
            } else if (command.line()) {
                processedCommand = this.processLineCommand(command.line());
            } else if (command.circle()) {
                processedCommand = this.processCircleCommand(command.circle());
            } else if (command.arc()) {
                processedCommand = this.processArcCommand(command.arc());
            } else if (command.rectangle()) {
                processedCommand = this.processRectangleCommand(command.rectangle());
            } else if (command.setPosition()) {
                processedCommand = this.processSetPositionCommand(command.setPosition());
            } else if (command.setWidth()) {
                processedCommand = this.processSetWidthCommand(command.setWidth());
            }
            
            if (processedCommand) {
                commands.push(processedCommand);
            }
        }
        return commands;
    }

    processVariables(commands) {
        const processed = [];
        for (const command of commands) {
            if (command.type === 'variable') {
                const varCommands = this.variables[command.name];
                if (varCommands) {
                    // Process the variable commands recursively
                    const processedVarCommands = this.processVariables(varCommands);
                    processed.push(...processedVarCommands);
                }
            } else {
                processed.push(command);
            }
        }
        return processed;
    }
}

function parseIconsFile(content) {
    const input = new antlr4.InputStream(content);
    const lexer = new IconScriptLexer(input);
    const stream = new antlr4.CommonTokenStream(lexer);
    const parser = new IconScriptParser(stream);
    
    // Add error listener
    parser.removeErrorListeners();
    parser.addErrorListener({
        syntaxError: (recognizer, offendingSymbol, line, column, msg, e) => {
            console.error(`Syntax error at line ${line}:${column} - ${msg}`);
        }
    });
    
    const tree = parser.script();

    const listener = new IconScriptListener();
    console.log("Walker available:", typeof antlr4.tree.ParseTreeWalker);
    console.log("Walker DEFAULT:", typeof antlr4.tree.ParseTreeWalker.DEFAULT);
    console.log("Walker walk method:", typeof antlr4.tree.ParseTreeWalker.DEFAULT.walk);
    antlr4.tree.ParseTreeWalker.DEFAULT.walk(listener, tree);

    console.log(`Parsed ${listener.icons.length} icons`);
    console.log(`Found ${Object.keys(listener.variables).length} variables`);
    
    return listener.icons;
}

function generateIcons(inputFile = "icons.txt") {
    try {
        // Read the specified file or default to `icons.txt`.
        const iconsContent = fs.readFileSync(inputFile, "utf8");
        const icons = parseIconsFile(iconsContent);
        
        // Ensure icons directory exists.
        if (!fs.existsSync("icons")) {
            fs.mkdirSync("icons");
        }

        const generator = new IconGenerator();
        let iconCount = 0;

        for (let i = 0; i < icons.length; i++) {
            const icon = icons[i];
            const svg = generator.processCommands(icon.commands);
            
            if (svg) {
                // Generate filename.
                let filename;
                if (icon.name) {
                    filename = `${icon.name}.svg`;
                } else {
                    filename = `icon_${i}.svg`;
                }
                
                const filepath = path.join("icons", filename);
                fs.writeFileSync(filepath, svg);
                console.log(`Generated: ${filename}`);
                iconCount++;
            }
        }

        console.log(`\nGenerated ${iconCount} SVG files in the icons directory.`);
        
    } catch (error) {
        console.error("Error:", error.message);
        process.exit(1);
    }
}

// Run the generator.
const inputFile = process.argv[2] || "icons.txt";
generateIcons(inputFile);

// Export for testing
export { IconGenerator, parseIconsFile }; 