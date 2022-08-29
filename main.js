/**
 * Parser for icon shapes for Röntgen project.
 *
 * @author Sergey Vartanov
 * @since 28 August 2022
 * @see https://github.com/enzet/Roentgen
 */

var scale = 7.0;
var size = 16.0; // Icons are 14 × 14 pixels, and this is size of icon with
                 // 1 pixel margin.
var shift = new Point(0.5 * scale, 0.5 * scale);
var rows = 10;   // Number of rows (icons in a column).
var columns = 6; // Number of columns (icons in a row).

var sketchOpacity = 0.2;
var sketchColor = new Color(0, 0, 0);

var finalOpacity = 1;
var finalColor = new Color(0, 0, 1);

/**
 * Convert position in icon shape coordinates to shifted and scaled coordinates
 * of the canvas.
 *
 * @param {vector} position coordinates to convert
 * @returns point on the canvas
 */
function toCoordinates(position) {
    return new Point(
        [Number(position[0]), Number(position[1])]
    ) * scale + shift;
}

/**
 * Add point represented as a circle.
 *
 * @param {vector} position center position of a circle
 * @returns created circle
 */
function addPoint(position) {

    return new Path.Circle({
        center: toCoordinates(position),
        radius: scale / 2,
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
function addLine(start, end) {

    var from = toCoordinates(start);
    var to = toCoordinates(end);
    var v = to - from;
    v.angle += 90.0;
    v = v / v.length * scale / 2.0;

    var path = new Path({insert: true});
    path.fillColor = sketchColor;
    path.opacity = sketchOpacity;
    path.add(from + v, to + v, to - v, from - v);

    return path;
}

/**
 * Parse shape description file line by line.
 */
function parse() {

    var current = new Path.Circle([0, 0], 10);

    shift = new Point(0.5 * scale, 0.5 * scale);

    project.clear();
    addGrid();

    var lines = area.value.split("\n");
    var variables = {};
    var shape = null;

    for (var i = 0; i < lines.length; i++) {

        var line = lines[i].trim();

        var parts = line.split(" ");

        if (parts[0] == "shape" && shape) {

            fakeShape = new Path({insert: true});
            fakeShape = fakeShape.unite(shape);
            fakeShape.translate([0, scale * size])
            fakeShape.opacity = finalOpacity;
            fakeShape.fillColor = finalColor;

            shift += new Point(scale * size, 0);
            if (shift.x > scale * columns * size) {
                shift += new Point(0, scale * size * 2);
                shift.x = 0.5 * scale;
            }
            shape = null;
            continue;
        }

        if (parts[1] == "=") {
            variables[parts[0]] = parts.slice(2);
            continue;
        } else {
            newParts = [];
            for (j = 0; j < parts.length; j++) {
                part = parts[j];
                if (part[0] == "@") {
                    newParts = newParts.concat(variables[part.slice(1)]);
                } else {
                    newParts.push(part);
                }
            }
            parts = newParts;
        }

        // `L` means simple one pixel width line. `LF` means `L` but filled.
        if (parts[0] == "l" || parts[0] == "lf") {

            var filled = (parts[0] == "lf");
            var fill = new Path({insert: true});
            fill.fillColor = sketchColor;
            fill.opacity = sketchOpacity;
            
            var last = null;

            for (var j = 1; j < parts.length; j++) {

                coordinates = parts[j].split(",");
                point = addPoint(coordinates);
                console.log("add");

                if (!shape) {
                    shape = new Path({fillColor: "blue", insert: false})
                }
                shape = shape.unite(point, insert=false);

                if (filled) {
                    fill.add(toCoordinates(coordinates));
                }
                if (last) {
                    segment = addLine(last, coordinates);
                    shape = shape.unite(segment);
                    if (filled) {
                        shape = shape.unite(fill);
                    }
                }
                last = coordinates;
            }
        }
    }
}

/**
 * Draw grid part from (x1, y1) to (x2, y2).
 */
function addGridLine(x1, y1, x2, y2) {
    var path = new Path();
    path.strokeColor = "black";
    path.strokeWidth = 0.2;
    path.opacity = 0.4;
    path.add(new Point(x1, y1) * scale, new Point(x2, y2) * scale);
}

/**
 * Draw rectangular grid for parsed icons.
 */
function addGrid() {
    for (var i = 0; i < size * rows; i++) {
        addGridLine(0, i, size * columns, i);
    }
    for (var i = 0; i < size * columns; i++) {
        addGridLine(i, 0, i, size * rows);
    }
}

var area = document.getElementById("code");
area.addEventListener("input", parse, false);
var canvas = document.getElementById("canvas");

parse();
