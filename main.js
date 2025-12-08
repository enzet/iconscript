/**
 * Parser for icon shapes for Röntgen project.
 *
 * @author Sergey Vartanov
 * @since 28 August 2022
 * @see https://github.com/enzet/iconscript
 */
var scale = 8.2;

// Icons are 14 × 14 pixels, and this is size of icon with 1 pixel margin.
var size = 20.0;

// Number of rows (icons in a column).
var rows = 10;

// Number of columns (icons in a row).
var columns = Math.floor(800 / scale / size);

var shift = new Point(2.5 * scale, 2.5 * scale);
var current = new Point(0, 0);

var width = 1.0;
var uniting = true;

var sketchOpacity = 0.2;
var sketchColor = new Color(0, 0, 0);

var finalOpacity = 1;
var finalColor = new Color(0, 0, 0);

/**
 * Convert position in icon shape coordinates to shifted and scaled coordinates
 * of the canvas.
 *
 * @param {array} position coordinates to convert
 * @returns point on the canvas
 */
function toCoordinates(position) {
    if (position[0] === "+") {
        position = position.slice(1).split(",");
        p = new Point([Number(position[0]), Number(position[1])]);
        current = current + p;
    } else {
        position = position.split(",");
        p = new Point([Number(position[0]), Number(position[1])]);
        current = p;
    }
    return current * scale + shift;
}

/**
 * Add point represented as a circle.
 *
 * @param {Point} position center position of a circle
 * @param {Number} radius circle radius
 */
function addPoint(position, radius) {
    circle = new Path.Circle({
        center: position,
        radius: scale * width * 0.5 * radius,
        fillColor: sketchColor,
        opacity: sketchOpacity,
    });
    combine(circle);
}

/**
 * Add line represented as a rectangle between two circles.
 *
 * @param {array} from starting point
 * @param {array} to ending point
 */
function addLine(from, to) {
    var path = new Path();
    path.strokeColor = sketchColor;
    path.strokeWidth = scale;
    path.opacity = sketchOpacity;
    path.add(from, to);

    var paperOffset = window.PaperOffset || paper.PaperOffset;
    if (!paperOffset) {
        console.error("PaperOffset not available. Using fallback stroke.");
        var strokePath = path;
    } else {
        var strokePath = paperOffset.offsetStroke(path, (width * scale) / 2, {
            cap: "butt",
        });
    }
    strokePath.opacity = sketchOpacity;
    combine(strokePath);
}

function arcPoint(center, p1, r) {
    return center + new Point(Math.cos(p1), -Math.sin(p1)) * r * scale;
}

/**
 * Draw the arc (part of the circle).
 *
 * @param {Point} center center point of the circle that defines the arc
 * @param {Number} r radius of the circle
 * @param {Number} p1 angle of arc start in radians
 * @param {Number} p2 angle of arc end in radians
 */
function addArc(center, r, p1, p2) {
    var h = (p1 + p2) / 2;

    var a1 = arcPoint(center, p1, r);
    var a2 = arcPoint(center, h, r);
    var a3 = arcPoint(center, p2, r);

    addPoint(a1, 1);
    addPoint(a3, 1);

    var arc = new Path.Arc({
        from: a1,
        through: a2,
        to: a3,
        strokeColor: sketchColor,
        strokeWidth: scale,
        opacity: sketchOpacity,
    });
    var paperOffset = window.PaperOffset || paper.PaperOffset;
    if (!paperOffset) {
        console.error("PaperOffset not available. Using fallback stroke.");
        var strokePath = arc;
    } else {
        var strokePath = paperOffset.offsetStroke(arc, scale / 2, {
            cap: "butt",
        });
    }
    strokePath.opacity = sketchOpacity;
    combine(strokePath);
}

/**
 * Draw rectangle by two points.
 *
 * @param {Point} from top left point
 * @param {Point} to bottom right point
 */
function addRectangle(from, to) {
    p1 = new Point(from.x, to.y);
    p2 = new Point(to.x, from.y);
    addPoint(from, 1);
    addPoint(to, 1);
    addPoint(p1, 1);
    addPoint(p2, 1);
    addLine(from, p1);
    addLine(p1, to);
    addLine(to, p2);
    addLine(p2, from);
    combine(new Path.Rectangle(from, to));
}

var shape = null;
var fill = null;
var filled = false;

/**
 * Combine (unite or subtract) the object with the currently constructed shape.
 *
 * @param {Path} object path-like object
 */
function combine(object) {
    if (!shape) {
        shape = new Path({fillColor: "blue", insert: false});
    }
    if (uniting) {
        shape = shape.unite(object, (insert = false));
    } else {
        shape = shape.subtract(object, (insert = false));
    }
}

/**
 * Combine (unite or subtract) the fill with the currently constructed shape.
 */
function combineFill() {
    if (fill) {
        if (uniting) {
            shape = shape.unite(fill);
        } else {
            shape = shape.subtract(fill);
        }
        fill = null;
    }
}

/**
 * Process a command from the parsed AST and draw it.
 */
function processCommand(command) {
    if (command.type === "line") {
        combineFill();
        filled = false;
        fill = new Path();
        fill.fillColor = sketchColor;
        fill.opacity = sketchOpacity;

        var last = null;
        for (var i = 0; i < command.positions.length; i++) {
            var coordinates = toCoordinates(command.positions[i]);
            addPoint(coordinates, 1);

            if (last) {
                addLine(last, coordinates);
            }
            last = coordinates;
        }
    } else if (command.type === "line_filled") {
        combineFill();
        filled = true;
        fill = new Path();
        fill.fillColor = sketchColor;
        fill.opacity = sketchOpacity;

        var last = null;
        for (var i = 0; i < command.positions.length; i++) {
            var coordinates = toCoordinates(command.positions[i]);
            addPoint(coordinates, 1);

            if (filled) {
                fill.add(coordinates);
            }
            if (last) {
                addLine(last, coordinates);
            }
            last = coordinates;
        }
    } else if (command.type === "line_single") {
        var coordinates = toCoordinates(command.position);
        addPoint(coordinates, 1);
        if (current) {
            addLine(current, coordinates);
        }
        current = coordinates;
    } else if (command.type === "circle") {
        var center = toCoordinates(command.position);
        var radius = command.radius;
        addPoint(center, radius);
    } else if (command.type === "arc") {
        var center = toCoordinates(command.position);
        var radius = command.radius;
        var startAngle = command.startAngle;
        var endAngle = command.endAngle;
        addArc(center, radius, startAngle, endAngle);
    } else if (command.type === "rectangle") {
        var point1 = toCoordinates(command.position1);
        var point2 = toCoordinates(command.position2);
        addRectangle(point1, point2);
    } else if (command.type === "setPosition") {
        current = toCoordinates(command.position);
    } else if (command.type === "setWidth") {
        width = command.width;
    } else if (command.type === "remove") {
        combineFill();
        uniting = false;
    } else if (command.type === "add") {
        combineFill();
        uniting = true;
    }
}

/**
 * Process commands (scopes are already flattened by the parser).
 */
function processCommands(commands) {
    for (var i = 0; i < commands.length; i++) {
        processCommand(commands[i]);
    }
}

/**
 * Parse shape description file using ANTLR parser.
 */
function parse() {
    project.activeLayer.removeChildren();
    view.draw();

    shift = new Point(2.5 * scale, 2.5 * scale);
    current = new Point(0, 0);
    width = 1.0;
    uniting = true;
    shape = null;
    fill = null;
    filled = false;

    addGrid();

    // Check if bundled parser is available.
    if (typeof IconScriptParser === "undefined") {
        console.error("IconScriptParser not found. Make sure bundled-parser.min.js is loaded.");
        return;
    }

    try {
        // Parse the iconscript code using the bundled parser.
        var icons = IconScriptParser.parseIconsFile(area.value);

        // Process each icon.
        for (var i = 0; i < icons.length; i++) {
            var icon = icons[i];
            shape = null;
            fill = null;
            filled = false;
            current = new Point(0, 0);
            uniting = true;
            width = 1.0;

            // Process all commands for this icon.
            processCommands(icon.commands);

            // Finish the icon.
            if (shape) {
                combineFill();
                var fakeShape = new Path({insert: true});
                fakeShape = fakeShape.unite(shape);
                fakeShape.translate([0, scale * size]);
                fakeShape.opacity = finalOpacity;
                fakeShape.fillColor = finalColor;

                shift += new Point(scale * size, 0);
                if (shift.x > scale * columns * size) {
                    shift += new Point(0, scale * size * 2);
                    shift.x = 2.5 * scale;
                }
                shape = null;
            }
        }
    } catch (error) {
        console.error("Parse error:", error);
    }
}

/**
 * Draw grid part from (x1, y1) to (x2, y2).
 */
function addGridLine(x1, y1, x2, y2, color, width, opacity) {
    var path = new Path();
    path.strokeColor = color;
    path.strokeWidth = width;
    path.opacity = opacity;
    path.add(new Point(x1, y1) * scale, new Point(x2, y2) * scale);
}

/**
 * Draw rectangular grid for parsed icons.
 */
function addGrid() {
    var i;

    var color = new Color(0, 0, 0, 0.2);
    for (i = 0.5; i < rows; i += 1) {
        addGridLine(0, i * size, size * columns, i * size, color, 1, 1);
        for (j = -7; j <= 7; j++) {
            addGridLine(0, i * size + j, size * columns, i * size + j, color, 1, 0.5);
        }
    }
    for (i = 0.5; i < columns; i += 1) {
        addGridLine(i * size, 0, i * size, size * rows, color, 1, 1);
        for (j = -7; j <= 7; j++) {
            addGridLine(i * size + j, 0, i * size + j, size * rows, color, 1, 0.5);
        }
    }

    color = new Color(1, 1, 1);
    for (i = 0; i <= size * rows; i += size) {
        addGridLine(0, i, size * columns, i, color, (size - 14) * scale, 1);
    }
    for (i = 0; i <= size * columns; i += size) {
        addGridLine(i, 0, i, size * rows, color, (size - 14) * scale, 1);
    }
}

var area = document.getElementById("code");
area.addEventListener("input", parse, false);
parse();

// Add download SVG button functionality.
var downloadButton = document.getElementById("download-svg");
downloadButton.addEventListener("click", function () {
    var svgString = project.exportSVG({asString: true});
    var blob = new Blob([svgString], {type: "image/svg+xml"});
    var url = URL.createObjectURL(blob);
    var a = document.createElement("a");
    a.href = url;
    a.download = "icon.svg";
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
});

// Export the whole project as SVG string.
var svgString = project.exportSVG({asString: true});

// Load `icons.txt` from file input.
var loadInput = document.getElementById("load-txt");
loadInput.addEventListener("change", function (event) {
    var file = event.target.files[0];
    if (!file) return;
    var reader = new FileReader();
    reader.onload = function (e) {
        area.value = e.target.result;
        parse();
    };
    reader.readAsText(file, "UTF-8");
});

// Save `icons.txt` from `<textarea>`.
var saveButton = document.getElementById("save-txt");
saveButton.addEventListener("click", function () {
    var text = area.value;
    var blob = new Blob([text], {type: "text/plain"});
    var url = URL.createObjectURL(blob);
    var a = document.createElement("a");
    a.href = url;
    a.download = "icons.txt";
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
});
