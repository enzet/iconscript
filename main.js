/**
 * Parser for icon shapes for Röntgen project.
 *
 * @author Sergey Vartanov
 * @since 28 August 2022
 * @see https://github.com/enzet/Roentgen
 */

var scale = 4;

// Icons are 14 × 14 pixels, and this is size of icon with 1 pixel margin.
var size = 20.0; 

// Number of rows (icons in a column).
var rows = 100; 

// Number of columns (icons in a row).
var columns = Math.floor(800 / scale / size);

var shift = new Point(2.5 * scale, 2.5 * scale);
var current = new Point(0, 0);

var width = 1.0;
var uniting = "add";

var sketchOpacity = 0.2;
var sketchColor = new Color(0, 0, 0);

var finalOpacity = 1;
var finalColor = new Color(0, 0, 0);

/**
 * Convert position in icon shape coordinates to shifted and scaled coordinates
 * of the canvas.
 *
 * @param {vector} position coordinates to convert
 * @returns point on the canvas
 */
function toCoordinates(position) {

    if (position[0] == "+") {
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
 * @param {vector} position center position of a circle
 * @returns created circle
 */
function addPoint(position, radius) {

    return new Path.Circle({
        center: position,
        radius: scale * width * 0.5 * radius,
        fillColor: sketchColor,
        opacity: sketchOpacity
    });
}

/**
 * Add line represented as a rectangle between two circles.
 *
 * @param {vector} start starting point
 * @param {vector} end ending point
 * @returns created rectangle
 */
function addLine(from, to) {

    var v = to - from;
    v.angle += 90.0;
    v = v / v.length * scale * width * 0.5;

    var path = new Path({insert: true});
    path.fillColor = sketchColor;
    path.opacity = sketchOpacity;
    path.add(from + v, to + v, to - v, from - v);

    return path;
}

var shape = null;

function combine(object) {
    if (!shape) {
        shape = new Path({fillColor: "blue", insert: false})
    }
    if (uniting == "add") {
        shape = shape.unite(object, insert=false);
    } else {
        shape = shape.subtract(object, insert=false);
    }
}

/**
 * Parse shape description file line by line.
 */
function parse() {

    var current = new Path.Circle([0, 0], 10);

    shift = new Point(2.5 * scale, 2.5 * scale);

    project.clear();
    addGrid();

    var lines = area.value.split("\n");
    var variables = {};

    var lexemes = []

    for (var i = 0; i < lines.length; i++) {
        parts = lines[i].trim().split(" ");
        if (parts[1] == "=") {
            variables[parts[0]] = []; // parts.slice(2);
            for (var j = 2; j < parts.length; j++) {
                part = parts[j];
                if (part[0] == "@") {
                    variables[parts[0]] = variables[parts[0]].concat(variables[part.slice(1)]);
                } else {
                    variables[parts[0]].push(part);
                }
            }
        } else {
            for (var j = 0; j < parts.length; j++) {
                part = parts[j];
                if (part[0] == "@") {
                    lexemes = lexemes.concat(variables[part.slice(1)]);
                } else {
                    lexemes.push(part);
                }
            }
        }
    }

    var filled = false;
    var fill = null;
    var mode = null;

    for (var i = 0; i < lexemes.length; i++) {

        var lexeme = lexemes[i];

        if (lexeme == "}" && shape) {

            if (fill) {
                shape = shape.unite(fill);
                fill = null;
            }
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
            shape = null;
        } else if (lexeme == "l" || lexeme == "lf") {
            // `L` means simple one pixel width line. `LF` means `L` but filled.

            if (fill) {
                shape = shape.unite(fill);
                fill = null;
            }
            filled = (lexeme == "lf");
            fill = new Path({insert: true});
            fill.fillColor = sketchColor;
            fill.opacity = sketchOpacity;
            
            var last = null;

            mode = "line";
        } else if (mode == "width") {
            width = Number(lexeme);
            mode = null;
        } else if (lexeme == "p") {
            current = toCoordinates(lexemes[i + 1]);
            i++;
        } else if (lexeme == "r") {
            uniting = "remove";
        } else if (lexeme == "a") {
            uniting = "add";
        } else if (lexeme == "w") {
            width = Number(lexemes[i + 1]);
            i++;
        } else if (lexeme == "c") {
            center = toCoordinates(lexemes[i + 1]);
            radius = Number(lexemes[i + 2]);
            circle = addPoint(center, radius);
            combine(circle);
            i += 2;
        } else if (lexeme.includes(",")) {

            coordinates = toCoordinates(lexeme);

            if (mode == "line") {
                point = addPoint(coordinates, 1);

                combine(point);
                if (filled) {
                    fill.add(coordinates);
                }
                if (last) {
                    segment = addLine(last, coordinates);
                    combine(segment);
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
    // for (var i = 0; i < size * rows; i++) {
    //     addGridLine(0, i, size * columns, i, 0.2, 0.2);
    // }
    // for (var i = 0; i < size * columns; i++) {
    //     addGridLine(i, 0, i, size * rows, 0.2, 0.2);
    // }

    gray = new Color(0.9, 0.9, 0.9);
    for (var i = 0; i <= size * rows; i += size) {
        addGridLine(0, i, size * columns, i, gray, 6 * scale, 1);
    }
    for (var i = 0; i <= size * columns; i += size) {
        addGridLine(i, 0, i, size * rows, gray, 6 * scale, 1);
    }
}

var area = document.getElementById("code");
area.addEventListener("input", parse, false);
var canvas = document.getElementById("canvas");

parse();
