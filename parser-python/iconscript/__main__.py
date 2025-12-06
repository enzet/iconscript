"""Script for converting iconscript files to SVG.

Usage: `python -m iconscript input.txt`
"""

import argparse
import logging
import math
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import override

import antlr4
import shapely
import svgwrite
from shapely.geometry import LineString, Point, Polygon
from shapely.ops import unary_union

from iconscript.parser.IconScriptLexer import IconScriptLexer
from iconscript.parser.IconScriptListener import IconScriptListener
from iconscript.parser.IconScriptParser import IconScriptParser

logger: logging.Logger = logging.getLogger(__name__)


@dataclass
class IconCollector(IconScriptListener):
    """Collect icons from an iconscript file."""

    variables: dict[str, IconScriptParser.CommandsContext] = field(
        default_factory=dict
    )
    icons: list[tuple[str, list[tuple[str, Polygon, float]]]] = field(
        default_factory=list
    )
    current_icon: IconScriptParser.IconContext | None = None
    current_icon_name: str | None = None
    current_shapes: list[tuple[str, Polygon, float]] = field(
        default_factory=list
    )
    current_position: tuple[float, float] = (0.0, 0.0)
    current_width: float = 1.0
    icon_counter: int = 0
    pending_commands: list[IconScriptParser.CommandsContext] = field(
        default_factory=list
    )
    in_icon: bool = False

    @override
    def exitAssignment(self, ctx: IconScriptParser.AssignmentContext) -> None:
        """Store variable assignment: `name = commands`."""
        name: str = ctx.IDENTIFIER().getText()
        commands_context: IconScriptParser.CommandsContext = ctx.commands()
        self.variables[name] = commands_context

    @override
    def enterIcon(self, ctx: IconScriptParser.IconContext) -> None:
        """Start processing an icon."""
        self.current_shapes = []
        self.current_icon_name = None
        self.in_icon = True
        self.current_position = (0.0, 0.0)
        self.current_width = 1.0

    @override
    def exitIcon(self, ctx: IconScriptParser.IconContext) -> None:
        """End processing an icon."""
        name: str = self.current_icon_name
        if name == "temp":
            name = f"icon_{self.icon_counter}"
            self.icon_counter += 1
        if name in [x for x, _ in self.icons]:
            logger.warning("Icon `%s` already defined.", name)
        self.icons.append((name, self.current_shapes))
        self.icon_counter += 1
        self.current_shapes = []
        self.current_icon_name = None
        self.in_icon = False

    @override
    def exitName(self, ctx: IconScriptParser.NameContext) -> None:
        """Process `%name` command."""
        self.current_icon_name = ctx.IDENTIFIER().getText()

    @override
    def exitLine(self, ctx: IconScriptParser.LineContext) -> None:
        """Process `l` (line) or `lf` (line filled) command."""
        positions: list[tuple[float, float]] = [
            self._parse_position(p) for p in ctx.position()
        ]
        if len(positions) >= 2:  # noqa: PLR2004
            line: LineString = LineString(positions)
            command_token: str = ctx.getChild(0).getText()
            if command_token == "lf":  # noqa: S105
                self.current_shapes.append(
                    ("line_filled", line, self.current_width)
                )
            else:
                self.current_shapes.append(("line", line, self.current_width))

    @override
    def exitRectangle(self, ctx: IconScriptParser.RectangleContext) -> None:
        """Process `s` (rectangle) command."""
        position_1: tuple[float, float] = self._parse_position(ctx.position(0))
        position_2: tuple[float, float] = self._parse_position(ctx.position(1))
        # Rectangle as polygon: (p1, (p2.x, p1.y), p2, (p1.x, p2.y), p1)
        rect: Polygon = Polygon(
            [
                position_1,
                (position_2[0], position_1[1]),
                position_2,
                (position_1[0], position_2[1]),
                position_1,
            ]
        )
        self.current_shapes.append(("rect", rect, self.current_width))

    @override
    def exitCircle(self, ctx: IconScriptParser.CircleContext) -> None:
        """Process `c` (circle) command."""
        center: tuple[float, float] = self._parse_position(ctx.position())
        radius: float = float(ctx.FLOAT().getText())
        circle: Polygon = Point(center).buffer(radius)
        self.current_shapes.append(("circle", circle, self.current_width))

    @override
    def exitSetPosition(self, ctx: IconScriptParser.SetPositionContext) -> None:
        """Process `p` (set position) command."""
        self.current_position = self._parse_position(ctx.position())

    @override
    def exitSetWidth(self, ctx: IconScriptParser.SetWidthContext) -> None:
        self.current_width = float(ctx.FLOAT().getText())

    @override
    def exitCommand(self, ctx: IconScriptParser.CommandContext) -> None:
        """Handle variable usage (e.g., `@foo`)."""
        if ctx.VARIABLE():
            variable_name: str = ctx.VARIABLE().getText()[1:]
            if variable_name in self.variables:
                commands_ctx: IconScriptParser.CommandsContext = self.variables[
                    variable_name
                ]
                walker: antlr4.ParseTreeWalker = antlr4.ParseTreeWalker()
                walker.walk(self, commands_ctx)
            else:
                logger.warning(
                    "Variable `@%s` not defined at %d:%d.",
                    variable_name,
                    ctx.start.line,
                    ctx.start.column,
                )
                self.pending_commands.append(ctx)

    def _parse_position(
        self, ctx: IconScriptParser.PositionContext
    ) -> tuple[float, float]:
        """Handle relative and absolute positions."""

        x: float = float(ctx.x.text)
        y: float = float(ctx.y.text)

        if ctx.relative:
            x += self.current_position[0]
            y += self.current_position[1]

        self.current_position = x, y

        return x, y


def shapely_to_svg_path(geometry: shapely.Geometry) -> str:
    """Convert a Shapely geometry to an SVG path.

    Only handles Polygon and LineString for brevity.
    """
    if geometry.is_empty:
        return ""

    if geometry.geom_type == "Polygon":
        exterior: str = (
            "M "
            + " L ".join(f"{x},{y}" for x, y in geometry.exterior.coords)
            + " Z"
        )
        interiors: list[str] = [
            "M " + " L ".join(f"{x},{y}" for x, y in ring.coords) + " Z"
            for ring in geometry.interiors
        ]
        return " ".join([exterior, *interiors])

    if geometry.geom_type == "LineString":
        return "M " + " L ".join(f"{x},{y}" for x, y in geometry.coords)

    if geometry.geom_type == "MultiPolygon":
        return " ".join(
            [shapely_to_svg_path(polygon) for polygon in geometry.geoms]
        )

    if geometry.geom_type == "MultiLineString":
        return " ".join([shapely_to_svg_path(line) for line in geometry.geoms])

    message: str = f"Unsupported geometry type: `{geometry.geom_type}`."
    raise ValueError(message)


def circle_to_bezier_path(
    center_x: float, center_y: float, radius: float
) -> str:
    """Approximate a circle with a Bezier path."""

    kappa: float = 0.5522847498

    x: float = center_x
    y: float = center_y
    r: float = radius
    k: float = kappa

    return (
        f"M {x + r},{y} "
        f"C {x + r},{y + k * r} {x + k * r},{y + r} {x},{y + r} "
        f"C {x - k * r},{y + r} {x - r},{y + k * r} {x - r},{y} "
        f"C {x - r},{y - k * r} {x - k * r},{y - r} {x},{y - r} "
        f"C {x + k * r},{y - r} {x + r},{y - k * r} {x + r},{y} "
        "Z"
    )


def iconscript_to_svg(input_file: Path, output_directory: Path) -> None:
    """Convert an iconscript file to SVG."""

    input_stream: antlr4.InputStream = antlr4.FileStream(
        str(input_file), encoding="utf-8"
    )
    lexer: IconScriptLexer = IconScriptLexer(input_stream)
    stream: antlr4.CommonTokenStream = antlr4.CommonTokenStream(lexer)
    parser: IconScriptParser = IconScriptParser(stream)
    tree: antlr4.ParseTree = parser.script()

    collector: IconCollector = IconCollector()
    walker: antlr4.ParseTreeWalker = antlr4.ParseTreeWalker()
    walker.walk(collector, tree)

    # Ensure output directory exists.
    output_directory.mkdir(exist_ok=True)

    # Output SVGs.
    for name, shapes in collector.icons:
        svg_path: Path = output_directory / f"{name}.svg"
        drawing: svgwrite.Drawing = svgwrite.Drawing(
            str(svg_path), profile="tiny"
        )
        all_geometries: list[shapely.Geometry] = []
        svg_paths: list[str] = []

        for shape_type, shape, width in shapes:
            if shape_type in ("line", "line_filled"):
                all_geometries.append(shape.buffer(width / 2))
            elif shape_type == "circle":
                # For boolean ops, use polygon; for SVG, use Bezier path.
                all_geometries.append(
                    shape
                )  # This is already a polygon from your collector.
                center_x: float = shape.centroid.x
                center_y: float = shape.centroid.y

                # Estimate radius from area (since shape is a polygon).
                radius: float = (shape.area / math.pi) ** 0.5
                svg_paths.append(
                    circle_to_bezier_path(center_x, center_y, radius)
                )
            else:
                all_geometries.append(shape)

        unioned = unary_union(all_geometries)

        # For non-circle shapes, get SVG path.
        svg_paths.append(shapely_to_svg_path(unioned))

        # Combine all path data.
        full_path_data = " ".join(svg_paths)
        drawing.add(drawing.path(d=full_path_data, fill="black", stroke="none"))
        drawing.save()

        logger.debug("SVG file written to `%s`.", svg_path)


def main() -> None:
    """Script entry point."""

    logging.basicConfig(level=logging.INFO)

    parser: argparse.ArgumentParser = argparse.ArgumentParser(
        description="Convert iconscript files to SVG."
    )
    parser.add_argument("-i", "--input", nargs="*", help="Input file names")
    parser.add_argument(
        "-o", "--output", help="Output directory", default="icons"
    )
    arguments: argparse.Namespace = parser.parse_args()

    inputs: list[Path]
    if arguments.input:
        inputs = [Path(input_name) for input_name in arguments.input]
    else:
        inputs = Path().glob("*.iconscript")

    for input_path in inputs:
        logger.info("Processing `%s`...", input_path)
        try:
            iconscript_to_svg(input_path, Path(arguments.output))
        except Exception:
            logger.exception("Error processing `%s`.", input_path)
            return 1

    return None


if __name__ == "__main__":
    sys.exit(main())
