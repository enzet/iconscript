#![allow(nonstandard_style)]
// Generated from ../grammar/IconScript.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::iconscriptparser::*;

pub trait IconScriptListener<'input> : ParseTreeListener<'input,IconScriptParserContextType>{
/**
 * Enter a parse tree produced by {@link IconScriptParser#script}.
 * @param ctx the parse tree
 */
fn enter_script(&mut self, _ctx: &ScriptContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#script}.
 * @param ctx the parse tree
 */
fn exit_script(&mut self, _ctx: &ScriptContext<'input>) { }
/**
 * Enter a parse tree produced by {@link IconScriptParser#position}.
 * @param ctx the parse tree
 */
fn enter_position(&mut self, _ctx: &PositionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#position}.
 * @param ctx the parse tree
 */
fn exit_position(&mut self, _ctx: &PositionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link IconScriptParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link IconScriptParser#assignment}.
 * @param ctx the parse tree
 */
fn enter_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#assignment}.
 * @param ctx the parse tree
 */
fn exit_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link IconScriptParser#scope}.
 * @param ctx the parse tree
 */
fn enter_scope(&mut self, _ctx: &ScopeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#scope}.
 * @param ctx the parse tree
 */
fn exit_scope(&mut self, _ctx: &ScopeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link IconScriptParser#commands}.
 * @param ctx the parse tree
 */
fn enter_commands(&mut self, _ctx: &CommandsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#commands}.
 * @param ctx the parse tree
 */
fn exit_commands(&mut self, _ctx: &CommandsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link IconScriptParser#icon}.
 * @param ctx the parse tree
 */
fn enter_icon(&mut self, _ctx: &IconContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#icon}.
 * @param ctx the parse tree
 */
fn exit_icon(&mut self, _ctx: &IconContext<'input>) { }
/**
 * Enter a parse tree produced by {@link IconScriptParser#command}.
 * @param ctx the parse tree
 */
fn enter_command(&mut self, _ctx: &CommandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#command}.
 * @param ctx the parse tree
 */
fn exit_command(&mut self, _ctx: &CommandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link IconScriptParser#arc}.
 * @param ctx the parse tree
 */
fn enter_arc(&mut self, _ctx: &ArcContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#arc}.
 * @param ctx the parse tree
 */
fn exit_arc(&mut self, _ctx: &ArcContext<'input>) { }
/**
 * Enter a parse tree produced by {@link IconScriptParser#circle}.
 * @param ctx the parse tree
 */
fn enter_circle(&mut self, _ctx: &CircleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#circle}.
 * @param ctx the parse tree
 */
fn exit_circle(&mut self, _ctx: &CircleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link IconScriptParser#line}.
 * @param ctx the parse tree
 */
fn enter_line(&mut self, _ctx: &LineContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#line}.
 * @param ctx the parse tree
 */
fn exit_line(&mut self, _ctx: &LineContext<'input>) { }
/**
 * Enter a parse tree produced by {@link IconScriptParser#rectangle}.
 * @param ctx the parse tree
 */
fn enter_rectangle(&mut self, _ctx: &RectangleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#rectangle}.
 * @param ctx the parse tree
 */
fn exit_rectangle(&mut self, _ctx: &RectangleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link IconScriptParser#name}.
 * @param ctx the parse tree
 */
fn enter_name(&mut self, _ctx: &NameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#name}.
 * @param ctx the parse tree
 */
fn exit_name(&mut self, _ctx: &NameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link IconScriptParser#setPosition}.
 * @param ctx the parse tree
 */
fn enter_setPosition(&mut self, _ctx: &SetPositionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#setPosition}.
 * @param ctx the parse tree
 */
fn exit_setPosition(&mut self, _ctx: &SetPositionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link IconScriptParser#setWidth}.
 * @param ctx the parse tree
 */
fn enter_setWidth(&mut self, _ctx: &SetWidthContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#setWidth}.
 * @param ctx the parse tree
 */
fn exit_setWidth(&mut self, _ctx: &SetWidthContext<'input>) { }
/**
 * Enter a parse tree produced by {@link IconScriptParser#setRemove}.
 * @param ctx the parse tree
 */
fn enter_setRemove(&mut self, _ctx: &SetRemoveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link IconScriptParser#setRemove}.
 * @param ctx the parse tree
 */
fn exit_setRemove(&mut self, _ctx: &SetRemoveContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : IconScriptListener<'input> }


