#![allow(nonstandard_style)]
// Generated from ../grammar/IconScript.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::iconscriptparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link IconScriptParser}.
 */
pub trait IconScriptVisitor<'input>: ParseTreeVisitor<'input,IconScriptParserContextType>{
	/**
	 * Visit a parse tree produced by {@link IconScriptParser#script}.
	 * @param ctx the parse tree
	 */
	fn visit_script(&mut self, ctx: &ScriptContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#position}.
	 * @param ctx the parse tree
	 */
	fn visit_position(&mut self, ctx: &PositionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#assignment}.
	 * @param ctx the parse tree
	 */
	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#scope}.
	 * @param ctx the parse tree
	 */
	fn visit_scope(&mut self, ctx: &ScopeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#commands}.
	 * @param ctx the parse tree
	 */
	fn visit_commands(&mut self, ctx: &CommandsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#icon}.
	 * @param ctx the parse tree
	 */
	fn visit_icon(&mut self, ctx: &IconContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#command}.
	 * @param ctx the parse tree
	 */
	fn visit_command(&mut self, ctx: &CommandContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#arc}.
	 * @param ctx the parse tree
	 */
	fn visit_arc(&mut self, ctx: &ArcContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#circle}.
	 * @param ctx the parse tree
	 */
	fn visit_circle(&mut self, ctx: &CircleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#line}.
	 * @param ctx the parse tree
	 */
	fn visit_line(&mut self, ctx: &LineContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#rectangle}.
	 * @param ctx the parse tree
	 */
	fn visit_rectangle(&mut self, ctx: &RectangleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#name}.
	 * @param ctx the parse tree
	 */
	fn visit_name(&mut self, ctx: &NameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#setPosition}.
	 * @param ctx the parse tree
	 */
	fn visit_setPosition(&mut self, ctx: &SetPositionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#setWidth}.
	 * @param ctx the parse tree
	 */
	fn visit_setWidth(&mut self, ctx: &SetWidthContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#setRemove}.
	 * @param ctx the parse tree
	 */
	fn visit_setRemove(&mut self, ctx: &SetRemoveContext<'input>) { self.visit_children(ctx) }

}

pub trait IconScriptVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= IconScriptParserContextType>{
	/**
	 * Visit a parse tree produced by {@link IconScriptParser#script}.
	 * @param ctx the parse tree
	 */
		fn visit_script(&mut self, ctx: &ScriptContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#position}.
	 * @param ctx the parse tree
	 */
		fn visit_position(&mut self, ctx: &PositionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#assignment}.
	 * @param ctx the parse tree
	 */
		fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#scope}.
	 * @param ctx the parse tree
	 */
		fn visit_scope(&mut self, ctx: &ScopeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#commands}.
	 * @param ctx the parse tree
	 */
		fn visit_commands(&mut self, ctx: &CommandsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#icon}.
	 * @param ctx the parse tree
	 */
		fn visit_icon(&mut self, ctx: &IconContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#command}.
	 * @param ctx the parse tree
	 */
		fn visit_command(&mut self, ctx: &CommandContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#arc}.
	 * @param ctx the parse tree
	 */
		fn visit_arc(&mut self, ctx: &ArcContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#circle}.
	 * @param ctx the parse tree
	 */
		fn visit_circle(&mut self, ctx: &CircleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#line}.
	 * @param ctx the parse tree
	 */
		fn visit_line(&mut self, ctx: &LineContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#rectangle}.
	 * @param ctx the parse tree
	 */
		fn visit_rectangle(&mut self, ctx: &RectangleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#name}.
	 * @param ctx the parse tree
	 */
		fn visit_name(&mut self, ctx: &NameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#setPosition}.
	 * @param ctx the parse tree
	 */
		fn visit_setPosition(&mut self, ctx: &SetPositionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#setWidth}.
	 * @param ctx the parse tree
	 */
		fn visit_setWidth(&mut self, ctx: &SetWidthContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link IconScriptParser#setRemove}.
	 * @param ctx the parse tree
	 */
		fn visit_setRemove(&mut self, ctx: &SetRemoveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> IconScriptVisitor<'input> for T
where
	T: IconScriptVisitorCompat<'input>
{
	fn visit_script(&mut self, ctx: &ScriptContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_script(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_position(&mut self, ctx: &PositionContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_position(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_assignment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scope(&mut self, ctx: &ScopeContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_scope(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_commands(&mut self, ctx: &CommandsContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_commands(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_icon(&mut self, ctx: &IconContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_icon(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_command(&mut self, ctx: &CommandContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_command(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arc(&mut self, ctx: &ArcContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_arc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_circle(&mut self, ctx: &CircleContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_circle(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_line(&mut self, ctx: &LineContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_line(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rectangle(&mut self, ctx: &RectangleContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_rectangle(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_name(&mut self, ctx: &NameContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setPosition(&mut self, ctx: &SetPositionContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_setPosition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setWidth(&mut self, ctx: &SetWidthContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_setWidth(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setRemove(&mut self, ctx: &SetRemoveContext<'input>){
		let result = <Self as IconScriptVisitorCompat>::visit_setRemove(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}