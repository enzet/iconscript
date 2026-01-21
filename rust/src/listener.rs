use antlr_rust::token::Token;
use antlr_rust::tree::ParseTree;
use antlr_rust::{
    common_token_stream::CommonTokenStream, tree::ParseTreeWalker, InputStream,
};
use anyhow::Result;
use std::collections::HashMap;
use std::rc::Rc;

use crate::generator::{create_circle_path, create_thick_line_path};
use crate::parser::iconscriptparser::*;
use crate::parser::*;
use crate::types::{Icon, PathWithMode, Point, Scope};

const SCALE: f64 = 1.0;

/// Parse iconscript content and return icons with their paths.
pub fn parse_iconscript(
    content: &str,
    sketch_mode: bool,
) -> Result<Vec<(Icon, Vec<PathWithMode>)>> {

    let input = InputStream::new(content);
    let lexer = IconScriptLexer::new(input);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = IconScriptParser::new(token_stream);
    let script = parser.script().expect("Failed to parse script");

    let listener = IconScriptListenerImpl::new(sketch_mode);
    let listener = ParseTreeWalker::<IconScriptParserContextType, _>::walk(
        Box::new(listener),
        &*script,
    );

    Ok(listener.into_icons())
}

struct IconScriptListenerImpl<'input> {
    variables: HashMap<String, Rc<CommandsContextAll<'input>>>,
    icons: Vec<(Icon, Vec<PathWithMode>)>,
    current_icon: Option<Icon>,
    paths: Vec<PathWithMode>,
    scopes: Vec<Scope>,
}

impl<'input> IconScriptListenerImpl<'input> {
    fn new(_sketch_mode: bool) -> Self {
        Self {
            variables: HashMap::new(),
            icons: Vec::new(),
            current_icon: None,
            paths: Vec::new(),
            scopes: vec![Scope::new()],
        }
    }

    fn get_scope(&self) -> &Scope {
        self.scopes.last().unwrap()
    }

    fn get_scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    fn into_icons(self) -> Vec<(Icon, Vec<PathWithMode>)> {
        self.icons
    }

    fn parse_position(&mut self, ctx: &PositionContext) -> Point {
        let x_text = ctx.x.as_ref().unwrap().get_text();
        let y_text = ctx.y.as_ref().unwrap().get_text();

        let x: f64 = x_text.parse().unwrap_or(0.0);
        let y: f64 = y_text.parse().unwrap_or(0.0);

        let is_relative = ctx.relative.is_some();

        let position = if is_relative {
            let current = self.get_scope().position;
            current.add(&Point::new(x, y))
        } else {
            Point::new(x + 0.5, y + 0.5)
        };

        self.get_scope_mut().position = position;
        position
    }

    fn arc_point(center: Point, angle: f64, radius: f64) -> Point {
        Point::new(
            center.x + angle.cos() * radius,
            center.y - angle.sin() * radius,
        )
    }

    /// Walk a `CommandsContext` to process variable expansion. This manually
    /// triggers the listener methods for each child command.
    fn walk_commands(&mut self, commands_ctx: &CommandsContext<'input>) {
        use antlr_rust::tree::Tree;

        // Process children in order (commands and scopes interleaved).
        // We track indices separately because command_all() and scope_all()
        // return them in order within their own type.
        let mut command_idx = 0;
        let mut scope_idx = 0;

        for i in 0..commands_ctx.get_child_count() {
            if let Some(child) = commands_ctx.get_child(i) {
                let rule_index = child.get_rule_index();
                if rule_index == RULE_command {
                    if let Some(cmd) = commands_ctx.command(command_idx) {
                        self.process_command(&cmd);
                    }
                    command_idx += 1;
                } else if rule_index == RULE_scope {
                    if let Some(scope) = commands_ctx.scope(scope_idx) {
                        self.enter_scope(&scope);
                        if let Some(inner_commands) = scope.commands() {
                            self.walk_commands(&inner_commands);
                        }
                        self.exit_scope(&scope);
                    }
                    scope_idx += 1;
                }
            }
        }
    }

    /// Process a single command, handling variable expansion.
    fn process_command(&mut self, ctx: &CommandContext<'input>) {
        use crate::parser::iconscriptparser::CommandContextAttrs;

        // Check if this is a variable reference.
        if let Some(var_token) = ctx.VARIABLE() {
            let var_name = &var_token.get_text()[1..]; // Remove '@' prefix
            if let Some(commands_ctx) = self.variables.get(var_name).cloned() {
                self.walk_commands(&commands_ctx);
            }
            return;
        }

        // Handle other command types by calling their exit methods.
        if let Some(line_ctx) = ctx.line() {
            self.exit_line(&line_ctx);
        } else if let Some(circle_ctx) = ctx.circle() {
            self.exit_circle(&circle_ctx);
        } else if let Some(arc_ctx) = ctx.arc() {
            self.exit_arc(&arc_ctx);
        } else if let Some(rect_ctx) = ctx.rectangle() {
            self.exit_rectangle(&rect_ctx);
        } else if let Some(pos_ctx) = ctx.setPosition() {
            self.exit_setPosition(&pos_ctx);
        } else if let Some(width_ctx) = ctx.setWidth() {
            self.exit_setWidth(&width_ctx);
        } else if let Some(remove_ctx) = ctx.setRemove() {
            self.exit_setRemove(&remove_ctx);
        } else if let Some(fill_ctx) = ctx.setFill() {
            self.exit_setFill(&fill_ctx);
        } else if let Some(name_ctx) = ctx.name() {
            self.exit_name(&name_ctx);
        }
    }
}

impl<'input> antlr_rust::tree::ParseTreeListener<'input, IconScriptParserContextType>
    for IconScriptListenerImpl<'input>
{
}

impl<'input> IconScriptListener<'input> for IconScriptListenerImpl<'input> {
    fn enter_icon(&mut self, _ctx: &IconContext<'input>) {
        self.current_icon = Some(Icon::new());
        self.paths.clear();
        self.scopes = vec![Scope::new()];
    }

    fn exit_icon(&mut self, _ctx: &IconContext<'input>) {
        if let Some(icon) = self.current_icon.take() {
            let paths = std::mem::take(&mut self.paths);
            self.icons.push((icon, paths));
        }
    }

    fn enter_assignment(&mut self, ctx: &AssignmentContext<'input>) {
        if let (Some(left), Some(right)) = (&ctx.left, &ctx.right) {
            let var_name = left.get_text().to_string();
            // Store the CommandsContext for later expansion.
            self.variables.insert(var_name, right.clone());
        }
    }

    fn enter_command(&mut self, ctx: &CommandContext<'input>) {

        // Check if this is a variable reference.
        if let Some(var_token) = ctx.VARIABLE() {
            let var_name = &var_token.get_text()[1..]; // Remove '@' prefix
            if let Some(commands_ctx) = self.variables.get(var_name).cloned() {
                self.walk_commands(&commands_ctx);
            }
        }
    }

    fn enter_scope(&mut self, _ctx: &ScopeContext<'input>) {
        let new_scope = self.scopes.last().unwrap().deep_copy();
        self.scopes.push(new_scope);
    }

    fn exit_scope(&mut self, _ctx: &ScopeContext<'input>) {
        self.scopes.pop();
    }

    fn exit_name(&mut self, ctx: &NameContext<'input>) {
        if let Some(icon) = &mut self.current_icon {
            let name = ctx.get_text();
            icon.name = Some(name);
        }
    }

    fn exit_circle(&mut self, ctx: &CircleContext<'input>) {
        if let Some(pos_ctx) = ctx.position() {
            let center = self.parse_position(&pos_ctx);
            let radius_text = ctx.FLOAT().unwrap().get_text();
            let radius: f64 = radius_text.parse().unwrap_or(0.0);

            if let Some(path) =
                create_circle_path(center.x, center.y, radius / 2.0)
            {
                self.paths.push(PathWithMode {
                    path,
                    mode: self.get_scope().uniting,
                });
            }
        }
    }

    fn exit_line(&mut self, ctx: &LineContext<'input>) {
        let is_filled = ctx.get_text().contains("lf") || self.get_scope().is_filled;
        let positions: Vec<Point> = ctx
            .position_all()
            .iter()
            .map(|pos| self.parse_position(pos))
            .collect();

        if positions.is_empty() {
            return;
        }

        let scope = self.get_scope();
        let width = scope.width;
        let uniting = scope.uniting;

        // Add circles at all points.
        for pos in &positions {
            if let Some(path) = create_circle_path(pos.x, pos.y, width / 2.0) {
                self.paths.push(PathWithMode {
                    path,
                    mode: uniting,
                });
            }
        }

        // Add lines between consecutive points.
        for i in 0..positions.len() - 1 {
            let from = positions[i];
            let to = positions[i + 1];
            if let Some(path) =
                create_thick_line_path(from.x, from.y, to.x, to.y, width)
            {
                self.paths.push(PathWithMode {
                    path,
                    mode: uniting,
                });
            }
        }

        // If filled, add a filled polyline.
        if is_filled && positions.len() >= 2 {
            let mut path = format!("M {} {}", positions[0].x, positions[0].y);
            for pos in &positions[1..] {
                path.push_str(&format!(" L {} {}", pos.x, pos.y));
            }
            path.push_str(" Z");
            self.paths.push(PathWithMode {
                path,
                mode: uniting,
            });
        }
    }

    fn exit_rectangle(&mut self, ctx: &RectangleContext<'input>) {
        let positions = ctx.position_all();
        if positions.len() < 2 {
            return;
        }

        let point1 = self.parse_position(&positions[0]);
        let point2 = self.parse_position(&positions[1]);

        let scope = self.get_scope();
        let width = scope.width;
        let uniting = scope.uniting;

        let p1 = Point::new(point2.x, point1.y);
        let p2 = Point::new(point1.x, point2.y);

        // Add circles at all four corners.
        let corners = [point1, p1, point2, p2];
        for corner in &corners {
            if let Some(path) =
                create_circle_path(corner.x, corner.y, width / 2.0)
            {
                self.paths.push(PathWithMode {
                    path,
                    mode: uniting,
                });
            }
        }

        let half_width = width / 2.0;

        // Add filled rectangles.
        let rect_path1 = format!(
            "M {} {} L {} {} L {} {} L {} {} Z",
            point1.x - half_width,
            point1.y,
            p1.x + half_width,
            p1.y,
            point2.x + half_width,
            point2.y,
            p2.x - half_width,
            p2.y
        );
        let rect_path2 = format!(
            "M {} {} L {} {} L {} {} L {} {} Z",
            point1.x,
            point1.y - half_width,
            p1.x,
            p1.y - half_width,
            point2.x,
            point2.y + half_width,
            p2.x,
            p2.y + half_width
        );

        self.paths.push(PathWithMode {
            path: rect_path1,
            mode: uniting,
        });
        self.paths.push(PathWithMode {
            path: rect_path2,
            mode: uniting,
        });
    }

    fn exit_arc(&mut self, ctx: &ArcContext<'input>) {
        use std::f64::consts::PI;

        if let Some(pos_ctx) = ctx.position() {
            let pos = self.parse_position(&pos_ctx);

            let floats = ctx.FLOAT_all();
            if floats.len() < 3 {
                return;
            }

            let radius: f64 =
                floats[0].get_text().parse().unwrap_or(0.0) * SCALE;
            let start_angle: f64 = floats[1].get_text().parse().unwrap_or(0.0);
            let end_angle: f64 = floats[2].get_text().parse().unwrap_or(0.0);

            let center = Point::new(pos.x + 0.5, pos.y + 0.5);

            let tau = 2.0 * PI;
            let mut delta = end_angle - start_angle;

            if delta.abs() < 1e-9 {
                return;
            }
            if delta.abs() > tau {
                let wrapped = ((delta % tau) + tau) % tau;
                delta = if delta < 0.0 { wrapped - tau } else { wrapped };
                if delta.abs() < 1e-9 {
                    return;
                }
            }

            let half_width = self.get_scope().width / 2.0;
            let uniting = self.get_scope().uniting;
            let outer_radius = radius + half_width;
            let inner_radius = (radius - half_width).max(0.0);

            let large_arc_flag = if delta.abs() > PI { 1 } else { 0 };
            let sweep_flag = if delta < 0.0 { 1 } else { 0 };

            let start_outer =
                Self::arc_point(center, start_angle, outer_radius);
            let end_outer = Self::arc_point(center, end_angle, outer_radius);

            let arc_path = if inner_radius == 0.0 {
                format!(
                    "M {} {} A {} {} 0 {} {} {} {} L {} {} Z",
                    start_outer.x,
                    start_outer.y,
                    outer_radius,
                    outer_radius,
                    large_arc_flag,
                    sweep_flag,
                    end_outer.x,
                    end_outer.y,
                    center.x,
                    center.y
                )
            } else {
                let end_inner =
                    Self::arc_point(center, end_angle, inner_radius);
                let start_inner =
                    Self::arc_point(center, start_angle, inner_radius);
                let sweep_inner = if sweep_flag == 1 { 0 } else { 1 };
                format!(
                    concat!(
                        "M {} {} A {} {} 0 {} {} {} {} L {} {} ",
                        "A {} {} 0 {} {} {} {} Z"
                    ),
                    start_outer.x,
                    start_outer.y,
                    outer_radius,
                    outer_radius,
                    large_arc_flag,
                    sweep_flag,
                    end_outer.x,
                    end_outer.y,
                    end_inner.x,
                    end_inner.y,
                    inner_radius,
                    inner_radius,
                    large_arc_flag,
                    sweep_inner,
                    start_inner.x,
                    start_inner.y
                )
            };

            self.paths.push(PathWithMode {
                path: arc_path,
                mode: uniting,
            });

            // Add round end caps.
            if half_width > 0.0 {
                let cap_start = Self::arc_point(center, start_angle, radius);
                let cap_end = Self::arc_point(center, end_angle, radius);
                if let Some(path1) =
                    create_circle_path(cap_start.x, cap_start.y, half_width)
                {
                    self.paths.push(PathWithMode {
                        path: path1,
                        mode: uniting,
                    });
                }
                if let Some(path2) =
                    create_circle_path(cap_end.x, cap_end.y, half_width)
                {
                    self.paths.push(PathWithMode {
                        path: path2,
                        mode: uniting,
                    });
                }
            }
        }
    }

    fn exit_setPosition(&mut self, ctx: &SetPositionContext<'input>) {
        if let Some(pos_ctx) = ctx.position() {
            self.parse_position(&pos_ctx);
        }
    }

    fn exit_setWidth(&mut self, ctx: &SetWidthContext<'input>) {
        if let Some(float_token) = ctx.FLOAT() {
            let width: f64 = float_token.get_text().parse().unwrap_or(1.0);
            self.get_scope_mut().width = width;
        }
    }

    fn exit_setRemove(&mut self, _ctx: &SetRemoveContext<'input>) {
        self.get_scope_mut().uniting = false;
    }

    fn exit_setFill(&mut self, _ctx: &SetFillContext<'input>) {
        self.get_scope_mut().is_filled = true;
    }
}
