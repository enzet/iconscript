/**
 * Parser for icon shapes for Röntgen project.
 *
 * @author Sergey Vartanov
 * @since 28 August 2022
 * @see https://github.com/enzet/iconscript
 */
var scale = 4.1;

// Icons are 14 × 14 pixels, and this is size of icon with 1 pixel margin.
var size = 20.0; 

// Number of rows (icons in a column).
var rows = 100; 

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
        opacity: sketchOpacity
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

    var strokePath = PaperOffset.offsetStroke(
        path, width * scale / 2, { cap: 'butt' }
    );
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
        opacity: sketchOpacity
    });
    var strokePath = PaperOffset.offsetStroke(arc, scale / 2, { cap: 'butt' });
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
        shape = new Path({fillColor: "blue", insert: false})
    }
    if (uniting) {
        shape = shape.unite(object, insert=false);
    } else {
        shape = shape.subtract(object, insert=false);
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
 * Parse shape description file line by line.
 */
function parse() {

    project.activeLayer.removeChildren();
    view.draw();
    var current = new Path.Circle([0, 0], 10);

    shift = new Point(2.5 * scale, 2.5 * scale);

    addGrid();

    var lines = area.value.split("\n");
    var variables = {};

    var lexemes = [];

    var i;
    var j;

    for (i = 0; i < lines.length; i++) {
        parts = lines[i].trim().split(" ");
        if (parts[1] === "=") {
            variables[parts[0]] = []; // parts.slice(2);
            for (j = 2; j < parts.length; j++) {
                part = parts[j];
                if (part[0] === "@") {
                    variables[parts[0]] = variables[parts[0]]
                        .concat(variables[part.slice(1)]);
                } else {
                    variables[parts[0]].push(part);
                }
            }
        } else {
            for (j = 0; j < parts.length; j++) {
                part = parts[j];
                if (part[0] === "@") {
                    lexemes = lexemes.concat(variables[part.slice(1)]);
                } else {
                    lexemes.push(part);
                }
            }
        }
    }

    var mode = null;

    for (i = 0; i < lexemes.length; i++) {

        var lexeme = lexemes[i];

        var center;
        var radius;

        if (lexeme === "}" && shape) {

            combineFill();
            fakeShape = new Path({insert: true});
            fakeShape = fakeShape.unite(shape);
            fakeShape.translate([0, scale * size])
            fakeShape.opacity = finalOpacity;
            fakeShape.fillColor = finalColor;

            shift += new Point(scale * size, 0);
            if (shift.x > scale * columns * size) {
                shift += new Point(0, scale * size * 2);
                shift.x = 2.5 * scale;
            }
            shape.selected = true;
            // console.log(shape.exportSVG());
            shape = null;
        } else if (lexeme === "l" || lexeme === "lf") {
            combineFill();
            filled = (lexeme === "lf");
            fill = new Path();
            fill.fillColor = sketchColor;
            fill.opacity = sketchOpacity;

            var last = null;

            mode = "line";
        } else if (mode === "width") {
            width = Number(lexeme);
            mode = null;
        } else if (lexeme === "p") {
            current = toCoordinates(lexemes[i + 1]);
            i++;
        } else if (lexeme === "r") {
            combineFill();
            uniting = false;
        } else if (lexeme === "a") {
            combineFill();
            uniting = true;
        } else if (lexeme === "w") {
            width = Number(lexemes[i + 1]);
            i++;
        } else if (lexeme === "c") {
            center = toCoordinates(lexemes[i + 1]);
            radius = Number(lexemes[i + 2]);
            addPoint(center, radius);
            i += 2;
        } else if (lexeme === "ar") {
            center = toCoordinates(lexemes[i + 1]);
            radius = Number(lexemes[i + 2]);
            p1 = Number(lexemes[i + 3]);
            p2 = Number(lexemes[i + 4]);
            addArc(center, radius, p1, p2);
            i += 4;
        } else if (lexeme === "s") {
            point_1 = toCoordinates(lexemes[i + 1]);
            point_2 = toCoordinates(lexemes[i + 2]);
            addRectangle(point_1, point_2);
            i += 2;
        } else if (lexeme.includes(",")) {

            coordinates = toCoordinates(lexeme);

            if (mode === "line") {
                addPoint(coordinates, 1);

                if (filled) {
                    fill.add(coordinates);
                }
                if (last) {
                    addLine(last, coordinates);
                }
                last = coordinates;
            }
        }
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
    var i

    for (i = 0; i < size * rows; i += size / 2) {
        addGridLine(0, i, size * columns, i, 0.2, 0.2);
    }
    for (i = 0; i < size * columns; i += size / 2) {
        addGridLine(i, 0, i, size * rows, 0.2, 0.2);
    }

    var gray = new Color(1, 1, 1);
    for (i = 0; i <= size * rows; i += size) {
        addGridLine(0, i, size * columns, i, gray, 6 * scale, 1);
    }
    for (i = 0; i <= size * columns; i += size) {
        addGridLine(i, 0, i, size * rows, gray, 6 * scale, 1);
    }
}

var area = document.getElementById("code");
area.addEventListener("input", parse, false);

parse();

// Add download SVG button functionality.
var downloadButton = document.getElementById("download-svg");
downloadButton.addEventListener("click", function() {
    var svgString = project.exportSVG({ asString: true });
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
var svgString = project.exportSVG({ asString: true });
