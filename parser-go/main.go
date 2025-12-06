// ANTLR-based parser for iconscript.
// Author: Sergey Vartanov <me@enzet.ru>
package main

import (
	"flag"
	"fmt"
	"log"
	"math"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	"github.com/antlr4-go/antlr/v4"
	"github.com/enzet/iconscript/parser-go/parser"
	"github.com/twpayne/go-geos"
)

// Figure is a simple 2D figure on the surface.
type Figure interface {
	// All figure types implement this interface.
}

// Line is a polyline through a set of positions.
type Line struct {
	positions []Position
	isFilled  bool
	width     float32
}

type Rectangle struct {
	start Position
	end   Position
	width float32
}

type Circle struct {
	center Position
	radius float32
	width  float32
}

// Arc is a part of a circle.
type Arc struct {
	center     Position
	radius     float32
	startAngle float32 // Start angle in radians.
	endAngle   float32 // End angle in radians.
	width      float32
}

// Icon is a 2-dimensional shape, described by the number of figures, that
// should be united or subtracted.
type Icon struct {
	name    string
	figures []Figure
}

// Context describes current state of the drawing process.
type Context struct {
	currentPosition *Position
	currentWidth    float32
	isUnionMode     bool
}

func (context *Context) Clear() {
	context.currentWidth = 1.0
	context.currentPosition = &Position{0, 0}
	context.isUnionMode = true
}

// Parser listener, that stores current parser state as well.
type IconScriptListener struct {
	*parser.BaseIconScriptListener

	unnamedIconId int
	currentIcon   *Icon
	icons         []*Icon

	// Stack of contexts.
	contexts []*Context
}

func (listener *IconScriptListener) getContext() *Context {
	return listener.contexts[len(listener.contexts)-1]
}

// Position is a 2-dimensional point on the plane.
type Position struct {
	x float32
	y float32
}

// Get string representation of a position.
func (position *Position) String() string {
	return fmt.Sprintf("%f,%f", position.x, position.y)
}

// Add other position to the given one and return the combination.
func (position *Position) Add(other *Position) Position {

	position.x += other.x
	position.y += other.y
	return Position{position.x, position.y}
}

// Errors are ignored since it should be checked by ANTLR.
func parseFloat(node string) float32 {

	float, _ := strconv.ParseFloat(node, 32)
	return float32(float)
}

// Read position from its string representation.
func readPosition(context parser.IPositionContext) Position {

	x := parseFloat(context.GetX().GetText())
	y := float32(0.0)
	if context.GetY() != nil {
		y = parseFloat(context.GetY().GetText())
	} else {
		println("Error: no Y component.")
	}
	return Position{x, y}
}

// Parse position from string representation and update current position.
func parsePosition(positionContext parser.IPositionContext,
	context *Context) Position {

	position := readPosition(positionContext)

	if positionContext.GetRelative() != nil {
		return context.currentPosition.Add(&position)
	} else {
		*context.currentPosition = position
		return position
	}
}

// ExitLine constructs line and adds it to the current icon.
func (listener *IconScriptListener) ExitLine(context *parser.LineContext) {

	positions := context.AllPosition()
	command := context.GetChild(0).GetPayload().(*antlr.CommonToken).GetText()

	line := &Line{
		positions: make([]Position, len(positions)),
		isFilled:  command == "lf",
		width:     listener.getContext().currentWidth,
	}

	for i, position := range positions {
		line.positions[i] = parsePosition(position, listener.getContext())
	}
	if listener.currentIcon != nil {
		listener.currentIcon.figures =
			append(listener.currentIcon.figures, line)
	}
}

// ExitRectangle constructs a rectangle and adds it to the current icon.
func (listener *IconScriptListener) ExitRectangle(
	context *parser.RectangleContext) {

	positions := context.AllPosition()

	rectangle := &Rectangle{
		start: parsePosition(positions[0], listener.getContext()),
		end:   parsePosition(positions[1], listener.getContext()),
		width: listener.getContext().currentWidth,
	}

	if listener.currentIcon != nil {
		listener.currentIcon.figures =
			append(listener.currentIcon.figures, rectangle)
	}
}

// ExitCircle constructs acircle and adds it to the current icon.
func (listener *IconScriptListener) ExitCircle(context *parser.CircleContext) {

	circle := &Circle{
		center: parsePosition(context.Position(), listener.getContext()),
		radius: parseFloat(context.FLOAT().GetText()),
		width:  listener.getContext().currentWidth,
	}

	if listener.currentIcon != nil {
		listener.currentIcon.figures =
			append(listener.currentIcon.figures, circle)
	}
}

// ExitArc constructs an arc and adds it to the current icon.
func (listener *IconScriptListener) ExitArc(context *parser.ArcContext) {

	floats := context.AllFLOAT()

	arc := &Arc{
		center:     parsePosition(context.Position(), listener.getContext()),
		radius:     parseFloat(floats[0].GetText()),
		startAngle: parseFloat(floats[1].GetText()),
		endAngle:   parseFloat(floats[2].GetText()),
		width:      listener.getContext().currentWidth,
	}

	if listener.currentIcon != nil {
		listener.currentIcon.figures =
			append(listener.currentIcon.figures, arc)
	}
}

// ExitSetPosition updates current position.
func (listener *IconScriptListener) ExitSetPosition(
	context *parser.SetPositionContext) {

	parsePosition(context.Position(), listener.getContext())
}

// ExitSetWidth sets current width.
func (listener *IconScriptListener) ExitSetWidth(
	context *parser.SetWidthContext) {

	listener.getContext().currentWidth = parseFloat(context.FLOAT().GetText())
}

func (listener *IconScriptListener) EnterScope(context *parser.ScopeContext) {

	newContext := listener.getContext() // Shallow copy.
	newContext.currentPosition = listener.getContext().currentPosition

	listener.contexts = append(listener.contexts, newContext)
}

func (listener *IconScriptListener) ExitScope(context *parser.ScopeContext) {

	listener.contexts = listener.contexts[:len(listener.contexts)-1]
}

// ExitName stores icon name.
func (listener *IconScriptListener) ExitName(context *parser.NameContext) {

	name := context.IDENTIFIER().GetText()
	if name == "temp" {
		name = fmt.Sprintf("icon_%d", listener.unnamedIconId)
		listener.unnamedIconId++
	}
	listener.currentIcon.name = name
}

// EnterIcon creates a new icon.
func (listener *IconScriptListener) EnterIcon(_ *parser.IconContext) {
	listener.currentIcon = new(Icon)
}

// ExitIcon adds constructed icon to the final set.
func (listener *IconScriptListener) ExitIcon(_ *parser.IconContext) {

	listener.icons =
		append(listener.icons, listener.currentIcon)

	listener.currentIcon = nil
	listener.getContext().Clear()
}

// Parse iconscript using ANTLR.
func parse(stream antlr.CharStream) (*IconScriptListener, error) {

	lexer := parser.NewIconScriptLexer(stream)
	tokenStream := antlr.NewCommonTokenStream(lexer, antlr.TokenDefaultChannel)
	p := parser.NewIconScriptParser(tokenStream)

	listener := &IconScriptListener{
		contexts:      []*Context{{currentPosition: &Position{0, 0}, currentWidth: 1.0, isUnionMode: true}},
		unnamedIconId: 0,
		currentIcon:   nil,
		icons:         nil,
	}

	antlr.ParseTreeWalkerDefault.Walk(listener, p.Script())

	return listener, nil
}

// Convert GEOS geometry to SVG path string.
func geometryToSVGPath(ctx *geos.Context, geom *geos.Geom) (string, error) {
	if geom.IsEmpty() {
		return "", nil
	}

	geomType := geom.TypeID()

	switch geomType {
	case geos.TypeIDPolygon:
		return polygonToSVGPath(ctx, geom)
	case geos.TypeIDLineString:
		return lineStringToSVGPath(ctx, geom)
	case geos.TypeIDMultiPolygon:
		return multiPolygonToSVGPath(ctx, geom)
	case geos.TypeIDMultiLineString:
		return multiLineStringToSVGPath(ctx, geom)
	default:
		return "", fmt.Errorf("unsupported geometry type: %d", geomType)
	}
}

// Convert polygon to SVG path.
func polygonToSVGPath(ctx *geos.Context, geom *geos.Geom) (string, error) {
	exterior := geom.ExteriorRing()
	if exterior == nil {
		return "", fmt.Errorf("failed to get exterior ring")
	}
	// ExteriorRing returns a reference, not ownership - do not destroy.

	exteriorPath, err := lineStringToSVGPath(ctx, exterior)
	if err != nil {
		return "", err
	}
	exteriorPath += " Z"

	numInteriors := geom.NumInteriorRings()

	var paths []string
	paths = append(paths, exteriorPath)

	for i := 0; i < numInteriors; i++ {
		interior := geom.InteriorRing(i)
		if interior == nil {
			return "", fmt.Errorf("failed to get interior ring %d", i)
		}
		// InteriorRing returns a reference, not ownership - do not destroy.

		interiorPath, err := lineStringToSVGPath(ctx, interior)
		if err != nil {
			return "", err
		}
		paths = append(paths, interiorPath+" Z")
	}

	return strings.Join(paths, " "), nil
}

// Convert LineString to SVG path.
func lineStringToSVGPath(ctx *geos.Context, geom *geos.Geom) (string, error) {
	coordSeq := geom.CoordSeq()
	if coordSeq == nil {
		return "", fmt.Errorf("failed to get coordinate sequence")
	}
	// CoordSeq returns a reference, not ownership - do not destroy.

	numPoints := coordSeq.Size()

	var coords []string
	for i := 0; i < numPoints; i++ {
		x := coordSeq.X(i)
		y := coordSeq.Y(i)
		coords = append(coords, fmt.Sprintf("%f,%f", x, y))
	}

	return "M " + strings.Join(coords, " L "), nil
}

// Convert MultiPolygon to SVG path.
func multiPolygonToSVGPath(ctx *geos.Context, geom *geos.Geom) (string, error) {
	numGeoms := geom.NumGeometries()

	var paths []string
	for i := 0; i < numGeoms; i++ {
		subGeom := geom.Geometry(i)
		if subGeom == nil {
			return "", fmt.Errorf("failed to get geometry %d", i)
		}
		// Geometry returns a reference, not ownership - do not destroy.

		path, err := polygonToSVGPath(ctx, subGeom)
		if err != nil {
			return "", err
		}
		paths = append(paths, path)
	}

	return strings.Join(paths, " "), nil
}

// Convert MultiLineString to SVG path.
func multiLineStringToSVGPath(ctx *geos.Context, geom *geos.Geom) (string, error) {
	numGeoms := geom.NumGeometries()

	var paths []string
	for i := 0; i < numGeoms; i++ {
		subGeom := geom.Geometry(i)
		if subGeom == nil {
			return "", fmt.Errorf("failed to get geometry %d", i)
		}
		// Geometry returns a reference, not ownership - do not destroy.

		path, err := lineStringToSVGPath(ctx, subGeom)
		if err != nil {
			return "", err
		}
		paths = append(paths, path)
	}

	return strings.Join(paths, " "), nil
}

// Convert circle to Bezier path for SVG.
func circleToBezierPath(centerX, centerY, radius float64) string {
	kappa := 0.5522847498
	x := centerX
	y := centerY
	r := radius
	k := kappa

	return fmt.Sprintf(
		"M %f,%f C %f,%f %f,%f %f,%f C %f,%f %f,%f %f,%f C %f,%f %f,%f %f,%f C %f,%f %f,%f %f,%f Z",
		x+r, y,
		x+r, y+k*r, x+k*r, y+r, x, y+r,
		x-k*r, y+r, x-r, y+k*r, x-r, y,
		x-r, y-k*r, x-k*r, y-r, x, y-r,
		x+k*r, y-r, x+r, y-k*r, x+r, y,
	)
}

// Convert Line to GEOS LineString.
func lineToGeometry(ctx *geos.Context, line *Line) (*geos.Geom, error) {
	if len(line.positions) < 2 {
		return nil, fmt.Errorf("line must have at least 2 positions")
	}

	coords := make([][]float64, len(line.positions))
	for i, pos := range line.positions {
		coords[i] = []float64{float64(pos.x), float64(pos.y)}
	}

	return ctx.NewLineString(coords), nil
}

// Convert Rectangle to GEOS Polygon.
func rectangleToGeometry(ctx *geos.Context, rect *Rectangle) (*geos.Geom, error) {
	// Rectangle as polygon: (p1, (p2.x, p1.y), p2, (p1.x, p2.y), p1)
	exterior := [][]float64{
		{float64(rect.start.x), float64(rect.start.y)},
		{float64(rect.end.x), float64(rect.start.y)},
		{float64(rect.end.x), float64(rect.end.y)},
		{float64(rect.start.x), float64(rect.end.y)},
		{float64(rect.start.x), float64(rect.start.y)},
	}

	rings := [][][]float64{exterior}
	return ctx.NewPolygon(rings), nil
}

// Convert Circle to GEOS Polygon (buffered point).
func circleToGeometry(ctx *geos.Context, circle *Circle) (*geos.Geom, error) {
	point := ctx.NewPoint([]float64{float64(circle.center.x), float64(circle.center.y)})
	if point == nil {
		return nil, fmt.Errorf("failed to create point")
	}
	defer point.Destroy()

	return point.Buffer(float64(circle.radius), 16), nil
}

// Process icons and generate SVG files.
func iconsToSVG(ctx *geos.Context, icons []*Icon, outputDir string) error {
	if err := os.MkdirAll(outputDir, 0755); err != nil {
		return err
	}

	for _, icon := range icons {
		var allGeometries []*geos.Geom
		var svgPaths []string

		for _, figure := range icon.figures {
			switch f := figure.(type) {
			case *Line:
				lineGeom, err := lineToGeometry(ctx, f)
				if err != nil {
					return err
				}

				buffered := lineGeom.Buffer(float64(f.width)/2, 16)
				if buffered == nil {
					lineGeom.Destroy()
					return fmt.Errorf("failed to buffer line")
				}
				lineGeom.Destroy()

				allGeometries = append(allGeometries, buffered)
			case *Rectangle:
				rectGeom, err := rectangleToGeometry(ctx, f)
				if err != nil {
					return err
				}

				allGeometries = append(allGeometries, rectGeom)
			case *Circle:
				circleGeom, err := circleToGeometry(ctx, f)
				if err != nil {
					return err
				}

				allGeometries = append(allGeometries, circleGeom)

				// For circles, also add Bezier path for better SVG rendering.
				centroid := circleGeom.Centroid()
				if centroid == nil {
					// Clean up geometries before returning.
					for _, g := range allGeometries {
						g.Destroy()
					}
					return fmt.Errorf("failed to get centroid")
				}

				centerX := centroid.X()
				centerY := centroid.Y()
				centroid.Destroy() // Centroid() returns ownership - must destroy.

				area := circleGeom.Area()
				radius := math.Sqrt(area / math.Pi)

				svgPaths = append(svgPaths, circleToBezierPath(centerX, centerY, radius))
			}
		}

		if len(allGeometries) == 0 {
			continue
		}

		// Union all geometries.
		unioned := allGeometries[0]
		for i := 1; i < len(allGeometries); i++ {
			next := unioned.Union(allGeometries[i])
			if next == nil {
				// Clean up remaining geometries.
				for j := i; j < len(allGeometries); j++ {
					allGeometries[j].Destroy()
				}
				if i > 1 {
					unioned.Destroy()
				}
				return fmt.Errorf("failed to union geometries")
			}
			if i > 1 {
				unioned.Destroy()
			}
			// Destroy the geometry we just unioned with.
			allGeometries[i].Destroy()
			unioned = next
		}

		// Convert unioned geometry to SVG path.
		unionPath, err := geometryToSVGPath(ctx, unioned)
		if err != nil {
			unioned.Destroy()
			return err
		}
		if unionPath != "" {
			svgPaths = append(svgPaths, unionPath)
		}
		unioned.Destroy() // Clean up unioned geometry after use.

		// Combine all path data.
		fullPathData := strings.Join(svgPaths, " ")

		// Write SVG file.
		svgPath := filepath.Join(outputDir, icon.name+".svg")
		if err := writeSVGFile(svgPath, fullPathData); err != nil {
			return err
		}

		log.Printf("SVG file written to `%s`", svgPath)
	}

	return nil
}

// Write SVG file.
func writeSVGFile(filePath, pathData string) error {
	file, err := os.Create(filePath)
	if err != nil {
		return err
	}
	defer file.Close()

	svgContent := fmt.Sprintf(`<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" version="1.1">
  <path d="%s" fill="black" stroke="none"/>
</svg>`, pathData)

	_, err = file.WriteString(svgContent)
	return err
}

// Script entry point: use `-i` to specify file name or `-c` to specify string
// commands.
func main() {

	fileName := flag.String("i", "main.iconscript", "input file name")
	commands := flag.String("c", "", "input string")
	outputDir := flag.String("o", "icons", "output directory")
	flag.Parse()

	// Initialize GEOS context.
	geosCtx := geos.NewContext()

	var parsed *IconScriptListener
	var err error

	if *commands == "" {
		stream, err := antlr.NewFileStream(*fileName)
		if err != nil {
			log.Fatal(err)
		}
		parsed, err = parse(stream)
		if err != nil {
			log.Fatal("Failed to parse file: ", err)
		}
	} else {
		stream := antlr.NewInputStream(*commands)
		parsed, err = parse(stream)
		if err != nil {
			log.Fatal("Failed to parse command: ", err)
		}
	}

	// Generate SVG files.
	if err := iconsToSVG(geosCtx, parsed.icons, *outputDir); err != nil {
		log.Fatal("Failed to generate SVG files: ", err)
	}
}
