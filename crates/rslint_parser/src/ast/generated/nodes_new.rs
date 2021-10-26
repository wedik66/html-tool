//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{
	ast::*,
	SyntaxKind::{self, *},
	SyntaxNode, SyntaxToken, T,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsScript {
	pub(crate) syntax: SyntaxNode,
}
impl JsScript {
	pub fn interpreter_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![interpreter])
	}
	pub fn directives_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![directives])
	}
	pub fn stmts(&self) -> AstChildren<JsStatement> { support::children(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsModule {
	pub(crate) syntax: SyntaxNode,
}
impl JsModule {
	pub fn interpreter_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![interpreter])
	}
	pub fn directives_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![directives])
	}
	pub fn stmts(&self) -> AstChildren<JsStatement> { support::children(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxNode {
	pub(crate) syntax: SyntaxNode,
}
impl SyntaxNode {
	pub fn syntax_node(&self) -> Option<SyntaxNode> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxToken {
	pub(crate) syntax: SyntaxNode,
}
impl SyntaxToken {
	pub fn syntax_token(&self) -> Option<SyntaxToken> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownStatement {
	pub fn syntax_element(&self) -> AstChildren<SyntaxElement> { support::children(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownExpression {
	pub fn syntax_element(&self) -> AstChildren<SyntaxElement> { support::children(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownPattern {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownPattern {
	pub fn syntax_element(&self) -> AstChildren<SyntaxElement> { support::children(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownMember {
	pub fn syntax_element(&self) -> AstChildren<SyntaxElement> { support::children(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsBlockStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsBlockStatement {
	pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
	pub fn stmts(&self) -> AstChildren<JsStatement> { support::children(&self.syntax) }
	pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsBreakStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsBreakStatement {
	pub fn break_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![break]) }
	pub fn label_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![label]) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsClassDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl JsClassDeclaration {
	pub fn class_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![class]) }
	pub fn id(&self) -> Option<JsBindingIdentifier> { support::child(&self.syntax) }
	pub fn extends_clause(&self) -> Option<JsExtendsClause> { support::child(&self.syntax) }
	pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
	pub fn members(&self) -> AstChildren<JsClassMember> { support::children(&self.syntax) }
	pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsDebuggerStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsDebuggerStatement {
	pub fn debugger_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![debugger])
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsEmptyStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsEmptyStatement {
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsExpressionStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsExpressionStatement {
	pub fn expression(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsForStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsForStatement {
	pub fn for_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![for]) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn init(&self) -> Option<JsForInit> { support::child(&self.syntax) }
	pub fn init_semicolon_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![init_semicolon])
	}
	pub fn test(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn test_semicolon_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![test_semicolon])
	}
	pub fn update(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
	pub fn body(&self) -> Option<JsStatement> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsForInStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsForInStatement {
	pub fn for_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![for]) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn left(&self) -> Option<JsForLeft> { support::child(&self.syntax) }
	pub fn in_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![in]) }
	pub fn right(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
	pub fn body(&self) -> Option<JsStatement> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsForOfStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsForOfStatement {
	pub fn for_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![for]) }
	pub fn await_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![await]) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn left(&self) -> Option<JsForLeft> { support::child(&self.syntax) }
	pub fn of_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![of]) }
	pub fn right(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
	pub fn body(&self) -> Option<JsStatement> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsFunctionDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl JsFunctionDeclaration {
	pub fn function_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![function])
	}
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn id(&self) -> Option<JsBindingIdentifier> { support::child(&self.syntax) }
	pub fn parameter_list(&self) -> Option<JsParameterList> { support::child(&self.syntax) }
	pub fn body(&self) -> Option<JsFunctionBody> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsIfStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsIfStatement {
	pub fn if_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![if]) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn test(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
	pub fn consequence(&self) -> Option<JsStatement> { support::child(&self.syntax) }
	pub fn else_clause(&self) -> Option<JsElseClause> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsLabeledStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsLabeledStatement {
	pub fn label_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![label]) }
	pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [:]) }
	pub fn body(&self) -> Option<JsStatement> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsReturnStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsReturnStatement {
	pub fn return_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![return]) }
	pub fn argument(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsSwitchStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsSwitchStatement {
	pub fn switch_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![switch]) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn discriminant(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
	pub fn cases(&self) -> AstChildren<JsSwitchCase> { support::children(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsTryStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsTryStatement {
	pub fn try_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![try]) }
	pub fn block(&self) -> Option<JsBlockStatement> { support::child(&self.syntax) }
	pub fn catch_clause(&self) -> Option<JsCatchClause> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsVariableDeclarationStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsVariableDeclarationStatement {
	pub fn declaration(&self) -> Option<JsVariableDeclaration> { support::child(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsWhileStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsWhileStatement {
	pub fn while_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![while]) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn test(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
	pub fn body(&self) -> Option<JsStatement> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsWithStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsWithStatement {
	pub fn with_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![with]) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn object(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
	pub fn body(&self) -> Option<JsStatement> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsExport {
	pub(crate) syntax: SyntaxNode,
}
impl JsExport {
	pub fn export_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![export]) }
	pub fn declaration(&self) -> Option<JsExportDeclaration> { support::child(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsExportDefault {
	pub(crate) syntax: SyntaxNode,
}
impl JsExportDefault {
	pub fn export_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![export]) }
	pub fn default_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![default]) }
	pub fn argument(&self) -> Option<JsExportDefaultArgument> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsExportFrom {
	pub(crate) syntax: SyntaxNode,
}
impl JsExportFrom {
	pub fn export_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![export]) }
	pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
	pub fn specifiers(&self) -> AstChildren<JsExportFromSpecifier> {
		support::children(&self.syntax)
	}
	pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
	pub fn from_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![from]) }
	pub fn module_specifier(&self) -> Option<JsStringLiteral> { support::child(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsExportAllFrom {
	pub(crate) syntax: SyntaxNode,
}
impl JsExportAllFrom {
	pub fn export_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![export]) }
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn export_name(&self) -> Option<JsExportName> { support::child(&self.syntax) }
	pub fn from_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![from]) }
	pub fn source(&self) -> Option<JsStringLiteral> { support::child(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsImport {
	pub(crate) syntax: SyntaxNode,
}
impl JsImport {
	pub fn import_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![import]) }
	pub fn clause(&self) -> Option<JsImportClause> { support::child(&self.syntax) }
	pub fn from_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![from]) }
	pub fn module_specifier(&self) -> Option<JsStringLiteral> { support::child(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsImportModule {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportModule {
	pub fn import_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![import]) }
	pub fn module_specifier(&self) -> Option<JsStringLiteral> { support::child(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsContinueStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsContinueStatement {
	pub fn continue_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![continue])
	}
	pub fn label_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![label]) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsDoWhileStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsDoWhileStatement {
	pub fn do_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![do]) }
	pub fn body(&self) -> Option<JsStatement> { support::child(&self.syntax) }
	pub fn while_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![while]) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn test(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsVariableDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl JsVariableDeclaration {
	pub fn kind_token_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![kind_token])
	}
	pub fn kind_token_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![kind_token])
	}
	pub fn kind_token_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![kind_token])
	}
	pub fn declarators(&self) -> AstChildren<JsVariableDeclarator> {
		support::children(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsBindingIdentifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsBindingIdentifier {
	pub fn name_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![name]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsParameterList {
	pub(crate) syntax: SyntaxNode,
}
impl JsParameterList {
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn parameters(&self) -> AstChildren<JsParameter> { support::children(&self.syntax) }
	pub fn rest(&self) -> Option<JsRestParameter> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsFunctionBody {
	pub(crate) syntax: SyntaxNode,
}
impl JsFunctionBody {
	pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
	pub fn directives_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![directives])
	}
	pub fn body(&self) -> AstChildren<JsStatement> { support::children(&self.syntax) }
	pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsElseClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsElseClause {
	pub fn else_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![else]) }
	pub fn alternate(&self) -> Option<JsStatement> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsCaseClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsCaseClause {
	pub fn case_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![case]) }
	pub fn test(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [:]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsDefaultClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsDefaultClause {
	pub fn default_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![default]) }
	pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [:]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsThrowStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsThrowStatement {
	pub fn throw_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![throw]) }
	pub fn argument(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsCatchClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsCatchClause {
	pub fn catch_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![catch]) }
	pub fn declaration(&self) -> Option<JsCatchDeclaration> { support::child(&self.syntax) }
	pub fn body(&self) -> Option<JsBlockStatement> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsTryFinallyStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsTryFinallyStatement {
	pub fn try_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![try]) }
	pub fn block(&self) -> Option<JsBlockStatement> { support::child(&self.syntax) }
	pub fn catch_clause(&self) -> Option<JsCatchClause> { support::child(&self.syntax) }
	pub fn finally_clause(&self) -> Option<JsFinallyClause> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsFinallyClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsFinallyClause {
	pub fn finally_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![finally]) }
	pub fn body(&self) -> Option<JsBlockStatement> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsCatchDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl JsCatchDeclaration {
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn binding(&self) -> Option<JsBinding> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsVariableDeclarator {
	pub(crate) syntax: SyntaxNode,
}
impl JsVariableDeclarator {
	pub fn id(&self) -> Option<JsBinding> { support::child(&self.syntax) }
	pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [=]) }
	pub fn init(&self) -> Option<JsExpression> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsArrayExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayExpression {
	pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
	pub fn elements(&self) -> AstChildren<AnyJsArrayElement> { support::children(&self.syntax) }
	pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsArrowFunctionExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrowFunctionExpression {
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn parameters(&self) -> Option<JsArrowFunctionParameters> { support::child(&self.syntax) }
	pub fn fat_arrow_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [=>]) }
	pub fn body(&self) -> Option<JsStatementOrExpression> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsAssignmentExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsAssignmentExpression {
	pub fn left(&self) -> Option<JsAssignmentTarget> { support::child(&self.syntax) }
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn right(&self) -> Option<JsExpression> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsAwaitExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsAwaitExpression {
	pub fn await_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![await]) }
	pub fn argument(&self) -> Option<JsExpression> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsBinaryExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsBinaryExpression {
	pub fn left(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn right(&self) -> Option<JsExpression> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsCallExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsCallExpression {
	pub fn callee(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn arguments(&self) -> AstChildren<JsCallArgument> { support::children(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsClassExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsClassExpression {
	pub fn class_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![class]) }
	pub fn id(&self) -> Option<JsBindingIdentifier> { support::child(&self.syntax) }
	pub fn extends_clause(&self) -> Option<JsExtendsClause> { support::child(&self.syntax) }
	pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
	pub fn members(&self) -> AstChildren<JsClassMember> { support::children(&self.syntax) }
	pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsConditionalExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsConditionalExpression {
	pub fn test(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn question_mark_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [?])
	}
	pub fn consequent(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [:]) }
	pub fn alternate(&self) -> Option<JsExpression> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsDoExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsDoExpression {
	pub fn do_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![do]) }
	pub fn body(&self) -> Option<JsBlockStatement> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsFunctionExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsFunctionExpression {
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn function_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![function])
	}
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn id(&self) -> Option<JsBindingIdentifier> { support::child(&self.syntax) }
	pub fn parameter_list(&self) -> Option<JsParameterList> { support::child(&self.syntax) }
	pub fn body(&self) -> Option<JsFunctionBody> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsImportCall {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportCall {
	pub fn import_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![import]) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn argument(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsLogicalExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsLogicalExpression {
	pub fn left(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn right(&self) -> Option<JsExpression> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsMemberExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsMemberExpression {
	pub fn object(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn member(&self) -> Option<JsMember> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsNewExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsNewExpression {
	pub fn new_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![new]) }
	pub fn callee(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn arguments(&self) -> AstChildren<JsCallArgument> { support::children(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsObjectExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectExpression {
	pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
	pub fn members(&self) -> AstChildren<JsObjectMember> { support::children(&self.syntax) }
	pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsOptionalCallExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsOptionalCallExpression {
	pub fn optional_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![optional])
	}
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn arguments(&self) -> AstChildren<JsCallArgument> { support::children(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsParenthesizedExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsParenthesizedExpression {
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn expression(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsReferenceIdentifierExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsReferenceIdentifierExpression {
	pub fn name_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![name]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsSequenceExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsSequenceExpression {
	pub fn left(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn comma_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![comma]) }
	pub fn second(&self) -> Option<JsExpression> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsSuperExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsSuperExpression {
	pub fn super_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![super]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsTaggedTemplateExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsTaggedTemplateExpression {
	pub fn tag(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn literal(&self) -> Option<JsTemplateLiteral> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsThisExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsThisExpression {
	pub fn this_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![this]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsUnaryExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnaryExpression {
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn argument(&self) -> Option<JsExpression> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsPreUpdateExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsPreUpdateExpression {
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operand(&self) -> Option<JsSimpleAssignmentTarget> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsPostUpdateExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsPostUpdateExpression {
	pub fn operand(&self) -> Option<JsSimpleAssignmentTarget> { support::child(&self.syntax) }
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
	pub fn operator_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![operator])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsYieldExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsYieldExpression {
	pub fn yield_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![yield]) }
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn argument(&self) -> Option<JsExpression> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsBooleanLiteral {
	pub(crate) syntax: SyntaxNode,
}
impl JsBooleanLiteral {
	pub fn value_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![value]) }
	pub fn value_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![value]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsBigIntLiteral {
	pub(crate) syntax: SyntaxNode,
}
impl JsBigIntLiteral {
	pub fn value_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![value]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsNullLiteral {
	pub(crate) syntax: SyntaxNode,
}
impl JsNullLiteral {
	pub fn value_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![value]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsNumberLiteral {
	pub(crate) syntax: SyntaxNode,
}
impl JsNumberLiteral {
	pub fn value_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![value]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsStringLiteral {
	pub(crate) syntax: SyntaxNode,
}
impl JsStringLiteral {
	pub fn value_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![value]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsTemplateLiteral {
	pub(crate) syntax: SyntaxNode,
}
impl JsTemplateLiteral {
	pub fn left_tick_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![left_tick])
	}
	pub fn elements(&self) -> AstChildren<JsTemplateElement> { support::children(&self.syntax) }
	pub fn right_tick_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![right_tick])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsArraySpreadElement {
	pub(crate) syntax: SyntaxNode,
}
impl JsArraySpreadElement {
	pub fn dotdotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [...]) }
	pub fn argument(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsArrayElement {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayElement {
	pub fn expression(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsArrayHole {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayHole {
	pub fn comma_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![comma]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsParameter {
	pub(crate) syntax: SyntaxNode,
}
impl JsParameter {
	pub fn binding(&self) -> Option<JsBinding> { support::child(&self.syntax) }
	pub fn default(&self) -> Option<JsDefaultValueClause> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsCallArgument {
	pub(crate) syntax: SyntaxNode,
}
impl JsCallArgument {
	pub fn value(&self) -> Option<JsExpressionOrSpread> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsSpread {
	pub(crate) syntax: SyntaxNode,
}
impl JsSpread {
	pub fn dotdotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [...]) }
	pub fn argument(&self) -> Option<JsExpression> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsStaticMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsStaticMember {
	pub fn dot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [.]) }
	pub fn name(&self) -> Option<JsStaticMemberName> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsComputedMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsComputedMember {
	pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
	pub fn name(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsIdentifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsIdentifier {
	pub fn name_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![name]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsStringTemplateElement {
	pub(crate) syntax: SyntaxNode,
}
impl JsStringTemplateElement {
	pub fn value_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![value]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsExtendsClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsExtendsClause {
	pub fn extends_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![extends]) }
	pub fn super_class(&self) -> Option<JsExpression> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsConstructorClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsConstructorClassMember {
	pub fn constructor_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![constructor])
	}
	pub fn parameter_list(&self) -> Option<JsParameterList> { support::child(&self.syntax) }
	pub fn body(&self) -> Option<JsFunctionBody> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsPropertyClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsPropertyClassMember {
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn key(&self) -> Option<JsClassMemberName> { support::child(&self.syntax) }
	pub fn value(&self) -> Option<JsPropertyClassMemberInitializer> { support::child(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsPrivatePropertyClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsPrivatePropertyClassMember {
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn key(&self) -> Option<JsPrivateClassMemberName> { support::child(&self.syntax) }
	pub fn value(&self) -> Option<JsPropertyClassMemberInitializer> { support::child(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsMethodClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsMethodClassMember {
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn key(&self) -> Option<JsClassMemberName> { support::child(&self.syntax) }
	pub fn parameter_list(&self) -> Option<JsParameterList> { support::child(&self.syntax) }
	pub fn body(&self) -> Option<JsFunctionBody> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsGetterClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsGetterClassMember {
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn get_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![get]) }
	pub fn key(&self) -> Option<JsClassMemberName> { support::child(&self.syntax) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
	pub fn body(&self) -> Option<JsFunctionBody> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsSetterClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsSetterClassMember {
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn set_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![set]) }
	pub fn key(&self) -> Option<JsClassMemberName> { support::child(&self.syntax) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn value(&self) -> Option<JsParameter> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
	pub fn body(&self) -> Option<JsFunctionBody> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsPrivateClassMemberName {
	pub(crate) syntax: SyntaxNode,
}
impl JsPrivateClassMemberName {
	pub fn hash_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [#]) }
	pub fn id_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![id]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsPropertyClassMemberInitializer {
	pub(crate) syntax: SyntaxNode,
}
impl JsPropertyClassMemberInitializer {
	pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [=]) }
	pub fn expression(&self) -> Option<JsExpression> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsPrivateMethodClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsPrivateMethodClassMember {
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn key(&self) -> Option<JsPrivateClassMemberName> { support::child(&self.syntax) }
	pub fn parameter_list(&self) -> Option<JsParameterList> { support::child(&self.syntax) }
	pub fn body(&self) -> Option<JsFunctionBody> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsMethodObjectMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsMethodObjectMember {
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn key(&self) -> Option<JsObjectMemberKey> { support::child(&self.syntax) }
	pub fn parameter_list(&self) -> Option<JsParameterList> { support::child(&self.syntax) }
	pub fn body(&self) -> Option<JsFunctionBody> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsGetterObjectMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsGetterObjectMember {
	pub fn get_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![get]) }
	pub fn key(&self) -> Option<JsObjectMemberKey> { support::child(&self.syntax) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
	pub fn body(&self) -> Option<JsFunctionBody> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsSetterObjectMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsSetterObjectMember {
	pub fn set_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![set]) }
	pub fn key(&self) -> Option<JsObjectMemberKey> { support::child(&self.syntax) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn value(&self) -> Option<JsParameter> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
	pub fn body(&self) -> Option<JsFunctionBody> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsPropertyObjectMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsPropertyObjectMember {
	pub fn key(&self) -> Option<JsObjectMemberKey> { support::child(&self.syntax) }
	pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [:]) }
	pub fn value(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsSpreadObjectMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsSpreadObjectMember {
	pub fn dotdotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [...]) }
	pub fn argument(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsExportFromSpecifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsExportFromSpecifier {
	pub fn name(&self) -> Option<JsIdentifier> { support::child(&self.syntax) }
	pub fn export_name(&self) -> Option<JsExportName> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsExportName {
	pub(crate) syntax: SyntaxNode,
}
impl JsExportName {
	pub fn as_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![as]) }
	pub fn name(&self) -> Option<JsIdentifier> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsExportDefaultExpressionArgument {
	pub(crate) syntax: SyntaxNode,
}
impl JsExportDefaultExpressionArgument {
	pub fn expression(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsImportDefaultBinding {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportDefaultBinding {
	pub fn local_name(&self) -> Option<JsBindingIdentifier> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsNamespaceImportClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsNamespaceImportClause {
	pub fn default_binding(&self) -> Option<JsImportDefaultBinding> { support::child(&self.syntax) }
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn as_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![as]) }
	pub fn name(&self) -> Option<JsBindingIdentifier> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsNamedImportClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsNamedImportClause {
	pub fn default_binding(&self) -> Option<JsImportDefaultBinding> { support::child(&self.syntax) }
	pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
	pub fn named_imports(&self) -> AstChildren<JsImportSpecifier> {
		support::children(&self.syntax)
	}
	pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsImportSpecifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportSpecifier {
	pub fn name(&self) -> Option<JsIdentifier> { support::child(&self.syntax) }
	pub fn binding(&self) -> Option<JsImportBinding> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsImportBinding {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportBinding {
	pub fn as_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![as]) }
	pub fn name(&self) -> Option<JsBindingIdentifier> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsObjectBinding {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectBinding {
	pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
	pub fn properties(&self) -> AstChildren<AnyJsPropertyBinding> {
		support::children(&self.syntax)
	}
	pub fn rest(&self) -> Option<JsObjectRestBinding> { support::child(&self.syntax) }
	pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsArrayBinding {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayBinding {
	pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
	pub fn elements(&self) -> AstChildren<AnyJsArrayBindingElement> {
		support::children(&self.syntax)
	}
	pub fn rest(&self) -> Option<JsArrayRestBinding> { support::child(&self.syntax) }
	pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsDefaultValueClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsDefaultValueClause {
	pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [=]) }
	pub fn value(&self) -> Option<JsExpression> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsArrayRestBinding {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayRestBinding {
	pub fn dotdotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [...]) }
	pub fn binding(&self) -> Option<JsBinding> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsArrayBindingElement {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayBindingElement {
	pub fn binding(&self) -> Option<JsBinding> { support::child(&self.syntax) }
	pub fn default_value(&self) -> Option<JsDefaultValueClause> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsObjectRestBinding {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectRestBinding {
	pub fn dotdotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [...]) }
	pub fn binding(&self) -> Option<JsBindingIdentifier> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsShorthandPropertyBinding {
	pub(crate) syntax: SyntaxNode,
}
impl JsShorthandPropertyBinding {
	pub fn identifier(&self) -> Option<JsBindingIdentifier> { support::child(&self.syntax) }
	pub fn default_value(&self) -> Option<JsDefaultValueClause> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsPropertyBinding {
	pub(crate) syntax: SyntaxNode,
}
impl JsPropertyBinding {
	pub fn member(&self) -> Option<JsMember> { support::child(&self.syntax) }
	pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [:]) }
	pub fn binding(&self) -> Option<JsBinding> { support::child(&self.syntax) }
	pub fn default_value(&self) -> Option<JsDefaultValueClause> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsMemberAssignmentTarget {
	pub(crate) syntax: SyntaxNode,
}
impl JsMemberAssignmentTarget {
	pub fn object(&self) -> Option<JsExpression> { support::child(&self.syntax) }
	pub fn member(&self) -> Option<JsMember> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsIdentifierAssignmentTarget {
	pub(crate) syntax: SyntaxNode,
}
impl JsIdentifierAssignmentTarget {
	pub fn name_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![name]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsArrayAssignmentTarget {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayAssignmentTarget {
	pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
	pub fn elements(&self) -> AstChildren<AnyJsArrayAssignmentTargetElement> {
		support::children(&self.syntax)
	}
	pub fn rest(&self) -> Option<JsArrayAssignmentRest> { support::child(&self.syntax) }
	pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsObjectAssignmentTarget {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectAssignmentTarget {
	pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
	pub fn properties(&self) -> AstChildren<JsPropertyAssignmentTarget> {
		support::children(&self.syntax)
	}
	pub fn rest(&self) -> Option<JsObjectRestAssignmentTarget> { support::child(&self.syntax) }
	pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsArrayAssignmentRest {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayAssignmentRest {
	pub fn dotdotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [...]) }
	pub fn target(&self) -> Option<JsAssignmentTarget> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsArrayAssignmentTargetElement {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayAssignmentTargetElement {
	pub fn target(&self) -> Option<JsAssignmentTarget> { support::child(&self.syntax) }
	pub fn default_value(&self) -> Option<JsDefaultValueClause> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsObjectRestAssignmentTarget {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectRestAssignmentTarget {
	pub fn dotdotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [...]) }
	pub fn target(&self) -> Option<JsSimpleAssignmentTarget> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsShorthandPropertyAssignmentTarget {
	pub(crate) syntax: SyntaxNode,
}
impl JsShorthandPropertyAssignmentTarget {
	pub fn identifier(&self) -> Option<JsIdentifierAssignmentTarget> {
		support::child(&self.syntax)
	}
	pub fn default_value(&self) -> Option<JsDefaultValueClause> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsObjectPropertyAssignmentTarget {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectPropertyAssignmentTarget {
	pub fn member(&self) -> Option<JsMember> { support::child(&self.syntax) }
	pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [:]) }
	pub fn target(&self) -> Option<JsAssignmentTarget> { support::child(&self.syntax) }
	pub fn default_value(&self) -> Option<JsDefaultValueClause> { support::child(&self.syntax) }
	pub fn trailing_comma_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![trailing_comma])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsRestParameter {
	pub(crate) syntax: SyntaxNode,
}
impl JsRestParameter {
	pub fn dotdotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [...]) }
	pub fn binding(&self) -> Option<JsBinding> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsStatement {
	JsBlockStatement(JsBlockStatement),
	JsBreakStatement(JsBreakStatement),
	JsClassDeclaration(JsClassDeclaration),
	JsDebuggerStatement(JsDebuggerStatement),
	JsEmptyStatement(JsEmptyStatement),
	JsExpressionStatement(JsExpressionStatement),
	JsForStatement(JsForStatement),
	JsForInStatement(JsForInStatement),
	JsForOfStatement(JsForOfStatement),
	JsFunctionDeclaration(JsFunctionDeclaration),
	JsIfStatement(JsIfStatement),
	JsLabeledStatement(JsLabeledStatement),
	JsReturnStatement(JsReturnStatement),
	JsSwitchStatement(JsSwitchStatement),
	JsTryStatement(JsTryStatement),
	JsVariableDeclarationStatement(JsVariableDeclarationStatement),
	JsWhileStatement(JsWhileStatement),
	JsWithStatement(JsWithStatement),
	JsExport(JsExport),
	JsExportDefault(JsExportDefault),
	JsExportFrom(JsExportFrom),
	JsExportAllFrom(JsExportAllFrom),
	JsImport(JsImport),
	JsImportModule(JsImportModule),
	JsUnknownStatement(JsUnknownStatement),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SyntaxElement {
	SyntaxNode(SyntaxNode),
	SyntaxToken(SyntaxToken),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsExpression {
	JsArrayExpression(JsArrayExpression),
	JsArrowFunctionExpression(JsArrowFunctionExpression),
	JsAssignmentExpression(JsAssignmentExpression),
	JsAwaitExpression(JsAwaitExpression),
	JsBinaryExpression(JsBinaryExpression),
	JsCallExpression(JsCallExpression),
	JsClassExpression(JsClassExpression),
	JsConditionalExpression(JsConditionalExpression),
	JsDoExpression(JsDoExpression),
	JsFunctionExpression(JsFunctionExpression),
	JsImportCall(JsImportCall),
	JsLogicalExpression(JsLogicalExpression),
	JsMemberExpression(JsMemberExpression),
	JsNewExpression(JsNewExpression),
	JsObjectExpression(JsObjectExpression),
	JsOptionalCallExpression(JsOptionalCallExpression),
	JsParenthesizedExpression(JsParenthesizedExpression),
	JsReferenceIdentifierExpression(JsReferenceIdentifierExpression),
	JsSequenceExpression(JsSequenceExpression),
	JsSuperExpression(JsSuperExpression),
	JsTaggedTemplateExpression(JsTaggedTemplateExpression),
	JsThisExpression(JsThisExpression),
	JsUnaryExpression(JsUnaryExpression),
	JsPreUpdateExpression(JsPreUpdateExpression),
	JsPostUpdateExpression(JsPostUpdateExpression),
	JsYieldExpression(JsYieldExpression),
	JsBooleanLiteral(JsBooleanLiteral),
	JsBigIntLiteral(JsBigIntLiteral),
	JsNullLiteral(JsNullLiteral),
	JsNumberLiteral(JsNumberLiteral),
	JsStringLiteral(JsStringLiteral),
	JsTemplateLiteral(JsTemplateLiteral),
	JsUnknownExpression(JsUnknownExpression),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsForInit {
	JsExpression(JsExpression),
	JsVariableDeclaration(JsVariableDeclaration),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsForLeft {
	JsVariableDeclaration(JsVariableDeclaration),
	JsAssignmentTarget(JsAssignmentTarget),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsAssignmentTarget {
	JsSimpleAssignmentTarget(JsSimpleAssignmentTarget),
	JsArrayAssignmentTarget(JsArrayAssignmentTarget),
	JsObjectAssignmentTarget(JsObjectAssignmentTarget),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsSwitchCase {
	JsCaseClause(JsCaseClause),
	JsDefaultClause(JsDefaultClause),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsBinding {
	JsObjectBinding(JsObjectBinding),
	JsArrayBinding(JsArrayBinding),
	JsBindingIdentifier(JsBindingIdentifier),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnyJsArrayElement {
	JsArraySpreadElement(JsArraySpreadElement),
	JsArrayElement(JsArrayElement),
	JsArrayHole(JsArrayHole),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsArrowFunctionParameters {
	JsParameterList(JsParameterList),
	JsParameter(JsParameter),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsStatementOrExpression {
	JsStatement(JsStatement),
	JsExpression(JsExpression),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsExpressionOrSpread {
	JsExpression(JsExpression),
	JsSpread(JsSpread),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsMember {
	JsStaticMember(JsStaticMember),
	JsComputedMember(JsComputedMember),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsStaticMemberName {
	JsIdentifier(JsIdentifier),
	JsStringLiteral(JsStringLiteral),
	JsNumberLiteral(JsNumberLiteral),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsSimpleAssignmentTarget {
	JsMemberAssignmentTarget(JsMemberAssignmentTarget),
	JsIdentifierAssignmentTarget(JsIdentifierAssignmentTarget),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsTemplateElement {
	JsStringTemplateElement(JsStringTemplateElement),
	JsExpression(JsExpression),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsClassMember {
	JsConstructorClassMember(JsConstructorClassMember),
	JsPropertyClassMember(JsPropertyClassMember),
	JsPrivatePropertyClassMember(JsPrivatePropertyClassMember),
	JsMethodClassMember(JsMethodClassMember),
	JsGetterClassMember(JsGetterClassMember),
	JsSetterClassMember(JsSetterClassMember),
	JsUnknownMember(JsUnknownMember),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsClassMemberName {
	JsObjectMemberKey(JsObjectMemberKey),
	JsPrivateClassMemberName(JsPrivateClassMemberName),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsObjectMemberKey {
	JsStaticMemberName(JsStaticMemberName),
	JsComputedMember(JsComputedMember),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsObjectMember {
	JsMethodObjectMember(JsMethodObjectMember),
	JsGetterObjectMember(JsGetterObjectMember),
	JsSetterObjectMember(JsSetterObjectMember),
	JsPropertyObjectMember(JsPropertyObjectMember),
	JsSpreadObjectMember(JsSpreadObjectMember),
	JsUnknownMember(JsUnknownMember),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsExportDeclaration {
	JsFunctionDeclaration(JsFunctionDeclaration),
	JsClassDeclaration(JsClassDeclaration),
	JsVariableDeclarationStatement(JsVariableDeclarationStatement),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsExportDefaultArgument {
	JsClassDeclaration(JsClassDeclaration),
	JsFunctionDeclaration(JsFunctionDeclaration),
	JsExportDefaultExpressionArgument(JsExportDefaultExpressionArgument),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsImportClause {
	JsImportDefaultBinding(JsImportDefaultBinding),
	JsNamespaceImportClause(JsNamespaceImportClause),
	JsNamedImportClause(JsNamedImportClause),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnyJsArrayBindingElement {
	JsArrayHole(JsArrayHole),
	JsArrayBindingElement(JsArrayBindingElement),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnyJsPropertyBinding {
	JsShorthandPropertyBinding(JsShorthandPropertyBinding),
	JsPropertyBinding(JsPropertyBinding),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnyJsArrayAssignmentTargetElement {
	JsArrayHole(JsArrayHole),
	JsArrayAssignmentTargetElement(JsArrayAssignmentTargetElement),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsPropertyAssignmentTarget {
	JsShorthandPropertyAssignmentTarget(JsShorthandPropertyAssignmentTarget),
	JsObjectPropertyAssignmentTarget(JsObjectPropertyAssignmentTarget),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsFunction {
	JsFunctionDeclaration(JsFunctionDeclaration),
	JsFunctionExpression(JsFunctionExpression),
	JsArrowFunctionExpression(JsArrowFunctionExpression),
}
impl AstNode for JsScript {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SCRIPT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsModule {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_MODULE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SyntaxNode {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SYNTAX_NODE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SyntaxToken {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SYNTAX_TOKEN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsUnknownStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsUnknownExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsUnknownPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsUnknownMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsBlockStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BLOCK_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsBreakStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BREAK_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsClassDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CLASS_DECLARATION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsDebuggerStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DEBUGGER_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsEmptyStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EMPTY_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsExpressionStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EXPRESSION_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsForStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FOR_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsForInStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FOR_IN_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsForOfStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FOR_OF_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsFunctionDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FUNCTION_DECLARATION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsIfStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IF_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsLabeledStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_LABELED_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsReturnStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_RETURN_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsSwitchStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SWITCH_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsTryStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_TRY_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsVariableDeclarationStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_VARIABLE_DECLARATION_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsWhileStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_WHILE_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsWithStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_WITH_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsExport {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EXPORT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsExportDefault {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EXPORT_DEFAULT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsExportFrom {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EXPORT_FROM }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsExportAllFrom {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EXPORT_ALL_FROM }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsImport {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsImportModule {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_MODULE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsContinueStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CONTINUE_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsDoWhileStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DO_WHILE_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsVariableDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_VARIABLE_DECLARATION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsBindingIdentifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BINDING_IDENTIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsParameterList {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PARAMETER_LIST }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsFunctionBody {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FUNCTION_BODY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsElseClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ELSE_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsCaseClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CASE_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsDefaultClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DEFAULT_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsThrowStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_THROW_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsCatchClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CATCH_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsTryFinallyStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_TRY_FINALLY_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsFinallyClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FINALLY_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsCatchDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CATCH_DECLARATION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsVariableDeclarator {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_VARIABLE_DECLARATOR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsArrayExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsArrowFunctionExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARROW_FUNCTION_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsAssignmentExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ASSIGNMENT_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsAwaitExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_AWAIT_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsBinaryExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BINARY_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsCallExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CALL_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsClassExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CLASS_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsConditionalExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CONDITIONAL_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsDoExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DO_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsFunctionExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FUNCTION_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsImportCall {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_CALL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsLogicalExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_LOGICAL_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsMemberExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_MEMBER_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsNewExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NEW_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsObjectExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsOptionalCallExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OPTIONAL_CALL_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsParenthesizedExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PARENTHESIZED_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsReferenceIdentifierExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_REFERENCE_IDENTIFIER_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsSequenceExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SEQUENCE_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsSuperExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SUPER_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsTaggedTemplateExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_TAGGED_TEMPLATE_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsThisExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_THIS_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsUnaryExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNARY_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsPreUpdateExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PRE_UPDATE_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsPostUpdateExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_POST_UPDATE_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsYieldExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_YIELD_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsBooleanLiteral {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BOOLEAN_LITERAL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsBigIntLiteral {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BIG_INT_LITERAL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsNullLiteral {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NULL_LITERAL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsNumberLiteral {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NUMBER_LITERAL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsStringLiteral {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_STRING_LITERAL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsTemplateLiteral {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_TEMPLATE_LITERAL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsArraySpreadElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_SPREAD_ELEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsArrayElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_ELEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsArrayHole {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_HOLE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsParameter {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PARAMETER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsCallArgument {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CALL_ARGUMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsSpread {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SPREAD }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsStaticMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_STATIC_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsComputedMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_COMPUTED_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsIdentifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IDENTIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsStringTemplateElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_STRING_TEMPLATE_ELEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsExtendsClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EXTENDS_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsConstructorClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CONSTRUCTOR_CLASS_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsPropertyClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PROPERTY_CLASS_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsPrivatePropertyClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PRIVATE_PROPERTY_CLASS_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsMethodClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_METHOD_CLASS_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsGetterClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_GETTER_CLASS_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsSetterClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SETTER_CLASS_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsPrivateClassMemberName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PRIVATE_CLASS_MEMBER_NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsPropertyClassMemberInitializer {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PROPERTY_CLASS_MEMBER_INITIALIZER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsPrivateMethodClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PRIVATE_METHOD_CLASS_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsMethodObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_METHOD_OBJECT_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsGetterObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_GETTER_OBJECT_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsSetterObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SETTER_OBJECT_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsPropertyObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PROPERTY_OBJECT_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsSpreadObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SPREAD_OBJECT_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsExportFromSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EXPORT_FROM_SPECIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsExportName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EXPORT_NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsExportDefaultExpressionArgument {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EXPORT_DEFAULT_EXPRESSION_ARGUMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsImportDefaultBinding {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_DEFAULT_BINDING }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsNamespaceImportClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NAMESPACE_IMPORT_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsNamedImportClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NAMED_IMPORT_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsImportSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_SPECIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsImportBinding {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_BINDING }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsObjectBinding {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_BINDING }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsArrayBinding {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_BINDING }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsDefaultValueClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DEFAULT_VALUE_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsArrayRestBinding {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_REST_BINDING }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsArrayBindingElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_BINDING_ELEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsObjectRestBinding {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_REST_BINDING }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsShorthandPropertyBinding {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SHORTHAND_PROPERTY_BINDING }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsPropertyBinding {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PROPERTY_BINDING }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsMemberAssignmentTarget {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_MEMBER_ASSIGNMENT_TARGET }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsIdentifierAssignmentTarget {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IDENTIFIER_ASSIGNMENT_TARGET }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsArrayAssignmentTarget {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_ASSIGNMENT_TARGET }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsObjectAssignmentTarget {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_ASSIGNMENT_TARGET }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsArrayAssignmentRest {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_ASSIGNMENT_REST }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsArrayAssignmentTargetElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_ASSIGNMENT_TARGET_ELEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsObjectRestAssignmentTarget {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_REST_ASSIGNMENT_TARGET }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsShorthandPropertyAssignmentTarget {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SHORTHAND_PROPERTY_ASSIGNMENT_TARGET }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsObjectPropertyAssignmentTarget {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_PROPERTY_ASSIGNMENT_TARGET }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsRestParameter {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_REST_PARAMETER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl From<JsBlockStatement> for JsStatement {
	fn from(node: JsBlockStatement) -> JsStatement { JsStatement::JsBlockStatement(node) }
}
impl From<JsBreakStatement> for JsStatement {
	fn from(node: JsBreakStatement) -> JsStatement { JsStatement::JsBreakStatement(node) }
}
impl From<JsClassDeclaration> for JsStatement {
	fn from(node: JsClassDeclaration) -> JsStatement { JsStatement::JsClassDeclaration(node) }
}
impl From<JsDebuggerStatement> for JsStatement {
	fn from(node: JsDebuggerStatement) -> JsStatement { JsStatement::JsDebuggerStatement(node) }
}
impl From<JsEmptyStatement> for JsStatement {
	fn from(node: JsEmptyStatement) -> JsStatement { JsStatement::JsEmptyStatement(node) }
}
impl From<JsExpressionStatement> for JsStatement {
	fn from(node: JsExpressionStatement) -> JsStatement { JsStatement::JsExpressionStatement(node) }
}
impl From<JsForStatement> for JsStatement {
	fn from(node: JsForStatement) -> JsStatement { JsStatement::JsForStatement(node) }
}
impl From<JsForInStatement> for JsStatement {
	fn from(node: JsForInStatement) -> JsStatement { JsStatement::JsForInStatement(node) }
}
impl From<JsForOfStatement> for JsStatement {
	fn from(node: JsForOfStatement) -> JsStatement { JsStatement::JsForOfStatement(node) }
}
impl From<JsFunctionDeclaration> for JsStatement {
	fn from(node: JsFunctionDeclaration) -> JsStatement { JsStatement::JsFunctionDeclaration(node) }
}
impl From<JsIfStatement> for JsStatement {
	fn from(node: JsIfStatement) -> JsStatement { JsStatement::JsIfStatement(node) }
}
impl From<JsLabeledStatement> for JsStatement {
	fn from(node: JsLabeledStatement) -> JsStatement { JsStatement::JsLabeledStatement(node) }
}
impl From<JsReturnStatement> for JsStatement {
	fn from(node: JsReturnStatement) -> JsStatement { JsStatement::JsReturnStatement(node) }
}
impl From<JsSwitchStatement> for JsStatement {
	fn from(node: JsSwitchStatement) -> JsStatement { JsStatement::JsSwitchStatement(node) }
}
impl From<JsTryStatement> for JsStatement {
	fn from(node: JsTryStatement) -> JsStatement { JsStatement::JsTryStatement(node) }
}
impl From<JsVariableDeclarationStatement> for JsStatement {
	fn from(node: JsVariableDeclarationStatement) -> JsStatement {
		JsStatement::JsVariableDeclarationStatement(node)
	}
}
impl From<JsWhileStatement> for JsStatement {
	fn from(node: JsWhileStatement) -> JsStatement { JsStatement::JsWhileStatement(node) }
}
impl From<JsWithStatement> for JsStatement {
	fn from(node: JsWithStatement) -> JsStatement { JsStatement::JsWithStatement(node) }
}
impl From<JsExport> for JsStatement {
	fn from(node: JsExport) -> JsStatement { JsStatement::JsExport(node) }
}
impl From<JsExportDefault> for JsStatement {
	fn from(node: JsExportDefault) -> JsStatement { JsStatement::JsExportDefault(node) }
}
impl From<JsExportFrom> for JsStatement {
	fn from(node: JsExportFrom) -> JsStatement { JsStatement::JsExportFrom(node) }
}
impl From<JsExportAllFrom> for JsStatement {
	fn from(node: JsExportAllFrom) -> JsStatement { JsStatement::JsExportAllFrom(node) }
}
impl From<JsImport> for JsStatement {
	fn from(node: JsImport) -> JsStatement { JsStatement::JsImport(node) }
}
impl From<JsImportModule> for JsStatement {
	fn from(node: JsImportModule) -> JsStatement { JsStatement::JsImportModule(node) }
}
impl From<JsUnknownStatement> for JsStatement {
	fn from(node: JsUnknownStatement) -> JsStatement { JsStatement::JsUnknownStatement(node) }
}
impl AstNode for JsStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_BLOCK_STATEMENT
				| JS_BREAK_STATEMENT
				| JS_CLASS_DECLARATION
				| JS_DEBUGGER_STATEMENT
				| JS_EMPTY_STATEMENT
				| JS_EXPRESSION_STATEMENT
				| JS_FOR_STATEMENT
				| JS_FOR_IN_STATEMENT
				| JS_FOR_OF_STATEMENT
				| JS_FUNCTION_DECLARATION
				| JS_IF_STATEMENT
				| JS_LABELED_STATEMENT
				| JS_RETURN_STATEMENT
				| JS_SWITCH_STATEMENT
				| JS_TRY_STATEMENT
				| JS_VARIABLE_DECLARATION_STATEMENT
				| JS_WHILE_STATEMENT
				| JS_WITH_STATEMENT
				| JS_EXPORT | JS_EXPORT_DEFAULT
				| JS_EXPORT_FROM | JS_EXPORT_ALL_FROM
				| JS_IMPORT | JS_IMPORT_MODULE
				| JS_UNKNOWN_STATEMENT
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_BLOCK_STATEMENT => JsStatement::JsBlockStatement(JsBlockStatement { syntax }),
			JS_BREAK_STATEMENT => JsStatement::JsBreakStatement(JsBreakStatement { syntax }),
			JS_CLASS_DECLARATION => JsStatement::JsClassDeclaration(JsClassDeclaration { syntax }),
			JS_DEBUGGER_STATEMENT => {
				JsStatement::JsDebuggerStatement(JsDebuggerStatement { syntax })
			}
			JS_EMPTY_STATEMENT => JsStatement::JsEmptyStatement(JsEmptyStatement { syntax }),
			JS_EXPRESSION_STATEMENT => {
				JsStatement::JsExpressionStatement(JsExpressionStatement { syntax })
			}
			JS_FOR_STATEMENT => JsStatement::JsForStatement(JsForStatement { syntax }),
			JS_FOR_IN_STATEMENT => JsStatement::JsForInStatement(JsForInStatement { syntax }),
			JS_FOR_OF_STATEMENT => JsStatement::JsForOfStatement(JsForOfStatement { syntax }),
			JS_FUNCTION_DECLARATION => {
				JsStatement::JsFunctionDeclaration(JsFunctionDeclaration { syntax })
			}
			JS_IF_STATEMENT => JsStatement::JsIfStatement(JsIfStatement { syntax }),
			JS_LABELED_STATEMENT => JsStatement::JsLabeledStatement(JsLabeledStatement { syntax }),
			JS_RETURN_STATEMENT => JsStatement::JsReturnStatement(JsReturnStatement { syntax }),
			JS_SWITCH_STATEMENT => JsStatement::JsSwitchStatement(JsSwitchStatement { syntax }),
			JS_TRY_STATEMENT => JsStatement::JsTryStatement(JsTryStatement { syntax }),
			JS_VARIABLE_DECLARATION_STATEMENT => {
				JsStatement::JsVariableDeclarationStatement(JsVariableDeclarationStatement {
					syntax,
				})
			}
			JS_WHILE_STATEMENT => JsStatement::JsWhileStatement(JsWhileStatement { syntax }),
			JS_WITH_STATEMENT => JsStatement::JsWithStatement(JsWithStatement { syntax }),
			JS_EXPORT => JsStatement::JsExport(JsExport { syntax }),
			JS_EXPORT_DEFAULT => JsStatement::JsExportDefault(JsExportDefault { syntax }),
			JS_EXPORT_FROM => JsStatement::JsExportFrom(JsExportFrom { syntax }),
			JS_EXPORT_ALL_FROM => JsStatement::JsExportAllFrom(JsExportAllFrom { syntax }),
			JS_IMPORT => JsStatement::JsImport(JsImport { syntax }),
			JS_IMPORT_MODULE => JsStatement::JsImportModule(JsImportModule { syntax }),
			JS_UNKNOWN_STATEMENT => JsStatement::JsUnknownStatement(JsUnknownStatement { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsStatement::JsBlockStatement(it) => &it.syntax,
			JsStatement::JsBreakStatement(it) => &it.syntax,
			JsStatement::JsClassDeclaration(it) => &it.syntax,
			JsStatement::JsDebuggerStatement(it) => &it.syntax,
			JsStatement::JsEmptyStatement(it) => &it.syntax,
			JsStatement::JsExpressionStatement(it) => &it.syntax,
			JsStatement::JsForStatement(it) => &it.syntax,
			JsStatement::JsForInStatement(it) => &it.syntax,
			JsStatement::JsForOfStatement(it) => &it.syntax,
			JsStatement::JsFunctionDeclaration(it) => &it.syntax,
			JsStatement::JsIfStatement(it) => &it.syntax,
			JsStatement::JsLabeledStatement(it) => &it.syntax,
			JsStatement::JsReturnStatement(it) => &it.syntax,
			JsStatement::JsSwitchStatement(it) => &it.syntax,
			JsStatement::JsTryStatement(it) => &it.syntax,
			JsStatement::JsVariableDeclarationStatement(it) => &it.syntax,
			JsStatement::JsWhileStatement(it) => &it.syntax,
			JsStatement::JsWithStatement(it) => &it.syntax,
			JsStatement::JsExport(it) => &it.syntax,
			JsStatement::JsExportDefault(it) => &it.syntax,
			JsStatement::JsExportFrom(it) => &it.syntax,
			JsStatement::JsExportAllFrom(it) => &it.syntax,
			JsStatement::JsImport(it) => &it.syntax,
			JsStatement::JsImportModule(it) => &it.syntax,
			JsStatement::JsUnknownStatement(it) => &it.syntax,
		}
	}
}
impl From<SyntaxNode> for SyntaxElement {
	fn from(node: SyntaxNode) -> SyntaxElement { SyntaxElement::SyntaxNode(node) }
}
impl From<SyntaxToken> for SyntaxElement {
	fn from(node: SyntaxToken) -> SyntaxElement { SyntaxElement::SyntaxToken(node) }
}
impl AstNode for SyntaxElement {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, SYNTAX_NODE | SYNTAX_TOKEN) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SYNTAX_NODE => SyntaxElement::SyntaxNode(SyntaxNode { syntax }),
			SYNTAX_TOKEN => SyntaxElement::SyntaxToken(SyntaxToken { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			SyntaxElement::SyntaxNode(it) => &it.syntax,
			SyntaxElement::SyntaxToken(it) => &it.syntax,
		}
	}
}
impl From<JsArrayExpression> for JsExpression {
	fn from(node: JsArrayExpression) -> JsExpression { JsExpression::JsArrayExpression(node) }
}
impl From<JsArrowFunctionExpression> for JsExpression {
	fn from(node: JsArrowFunctionExpression) -> JsExpression {
		JsExpression::JsArrowFunctionExpression(node)
	}
}
impl From<JsAssignmentExpression> for JsExpression {
	fn from(node: JsAssignmentExpression) -> JsExpression {
		JsExpression::JsAssignmentExpression(node)
	}
}
impl From<JsAwaitExpression> for JsExpression {
	fn from(node: JsAwaitExpression) -> JsExpression { JsExpression::JsAwaitExpression(node) }
}
impl From<JsBinaryExpression> for JsExpression {
	fn from(node: JsBinaryExpression) -> JsExpression { JsExpression::JsBinaryExpression(node) }
}
impl From<JsCallExpression> for JsExpression {
	fn from(node: JsCallExpression) -> JsExpression { JsExpression::JsCallExpression(node) }
}
impl From<JsClassExpression> for JsExpression {
	fn from(node: JsClassExpression) -> JsExpression { JsExpression::JsClassExpression(node) }
}
impl From<JsConditionalExpression> for JsExpression {
	fn from(node: JsConditionalExpression) -> JsExpression {
		JsExpression::JsConditionalExpression(node)
	}
}
impl From<JsDoExpression> for JsExpression {
	fn from(node: JsDoExpression) -> JsExpression { JsExpression::JsDoExpression(node) }
}
impl From<JsFunctionExpression> for JsExpression {
	fn from(node: JsFunctionExpression) -> JsExpression { JsExpression::JsFunctionExpression(node) }
}
impl From<JsImportCall> for JsExpression {
	fn from(node: JsImportCall) -> JsExpression { JsExpression::JsImportCall(node) }
}
impl From<JsLogicalExpression> for JsExpression {
	fn from(node: JsLogicalExpression) -> JsExpression { JsExpression::JsLogicalExpression(node) }
}
impl From<JsMemberExpression> for JsExpression {
	fn from(node: JsMemberExpression) -> JsExpression { JsExpression::JsMemberExpression(node) }
}
impl From<JsNewExpression> for JsExpression {
	fn from(node: JsNewExpression) -> JsExpression { JsExpression::JsNewExpression(node) }
}
impl From<JsObjectExpression> for JsExpression {
	fn from(node: JsObjectExpression) -> JsExpression { JsExpression::JsObjectExpression(node) }
}
impl From<JsOptionalCallExpression> for JsExpression {
	fn from(node: JsOptionalCallExpression) -> JsExpression {
		JsExpression::JsOptionalCallExpression(node)
	}
}
impl From<JsParenthesizedExpression> for JsExpression {
	fn from(node: JsParenthesizedExpression) -> JsExpression {
		JsExpression::JsParenthesizedExpression(node)
	}
}
impl From<JsReferenceIdentifierExpression> for JsExpression {
	fn from(node: JsReferenceIdentifierExpression) -> JsExpression {
		JsExpression::JsReferenceIdentifierExpression(node)
	}
}
impl From<JsSequenceExpression> for JsExpression {
	fn from(node: JsSequenceExpression) -> JsExpression { JsExpression::JsSequenceExpression(node) }
}
impl From<JsSuperExpression> for JsExpression {
	fn from(node: JsSuperExpression) -> JsExpression { JsExpression::JsSuperExpression(node) }
}
impl From<JsTaggedTemplateExpression> for JsExpression {
	fn from(node: JsTaggedTemplateExpression) -> JsExpression {
		JsExpression::JsTaggedTemplateExpression(node)
	}
}
impl From<JsThisExpression> for JsExpression {
	fn from(node: JsThisExpression) -> JsExpression { JsExpression::JsThisExpression(node) }
}
impl From<JsUnaryExpression> for JsExpression {
	fn from(node: JsUnaryExpression) -> JsExpression { JsExpression::JsUnaryExpression(node) }
}
impl From<JsPreUpdateExpression> for JsExpression {
	fn from(node: JsPreUpdateExpression) -> JsExpression {
		JsExpression::JsPreUpdateExpression(node)
	}
}
impl From<JsPostUpdateExpression> for JsExpression {
	fn from(node: JsPostUpdateExpression) -> JsExpression {
		JsExpression::JsPostUpdateExpression(node)
	}
}
impl From<JsYieldExpression> for JsExpression {
	fn from(node: JsYieldExpression) -> JsExpression { JsExpression::JsYieldExpression(node) }
}
impl From<JsBooleanLiteral> for JsExpression {
	fn from(node: JsBooleanLiteral) -> JsExpression { JsExpression::JsBooleanLiteral(node) }
}
impl From<JsBigIntLiteral> for JsExpression {
	fn from(node: JsBigIntLiteral) -> JsExpression { JsExpression::JsBigIntLiteral(node) }
}
impl From<JsNullLiteral> for JsExpression {
	fn from(node: JsNullLiteral) -> JsExpression { JsExpression::JsNullLiteral(node) }
}
impl From<JsNumberLiteral> for JsExpression {
	fn from(node: JsNumberLiteral) -> JsExpression { JsExpression::JsNumberLiteral(node) }
}
impl From<JsStringLiteral> for JsExpression {
	fn from(node: JsStringLiteral) -> JsExpression { JsExpression::JsStringLiteral(node) }
}
impl From<JsTemplateLiteral> for JsExpression {
	fn from(node: JsTemplateLiteral) -> JsExpression { JsExpression::JsTemplateLiteral(node) }
}
impl From<JsUnknownExpression> for JsExpression {
	fn from(node: JsUnknownExpression) -> JsExpression { JsExpression::JsUnknownExpression(node) }
}
impl AstNode for JsExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_ARRAY_EXPRESSION
				| JS_ARROW_FUNCTION_EXPRESSION
				| JS_ASSIGNMENT_EXPRESSION
				| JS_AWAIT_EXPRESSION
				| JS_BINARY_EXPRESSION
				| JS_CALL_EXPRESSION
				| JS_CLASS_EXPRESSION
				| JS_CONDITIONAL_EXPRESSION
				| JS_DO_EXPRESSION
				| JS_FUNCTION_EXPRESSION
				| JS_IMPORT_CALL | JS_LOGICAL_EXPRESSION
				| JS_MEMBER_EXPRESSION
				| JS_NEW_EXPRESSION
				| JS_OBJECT_EXPRESSION
				| JS_OPTIONAL_CALL_EXPRESSION
				| JS_PARENTHESIZED_EXPRESSION
				| JS_REFERENCE_IDENTIFIER_EXPRESSION
				| JS_SEQUENCE_EXPRESSION
				| JS_SUPER_EXPRESSION
				| JS_TAGGED_TEMPLATE_EXPRESSION
				| JS_THIS_EXPRESSION
				| JS_UNARY_EXPRESSION
				| JS_PRE_UPDATE_EXPRESSION
				| JS_POST_UPDATE_EXPRESSION
				| JS_YIELD_EXPRESSION
				| JS_BOOLEAN_LITERAL
				| JS_BIG_INT_LITERAL
				| JS_NULL_LITERAL
				| JS_NUMBER_LITERAL
				| JS_STRING_LITERAL
				| JS_TEMPLATE_LITERAL
				| JS_UNKNOWN_EXPRESSION
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_ARRAY_EXPRESSION => JsExpression::JsArrayExpression(JsArrayExpression { syntax }),
			JS_ARROW_FUNCTION_EXPRESSION => {
				JsExpression::JsArrowFunctionExpression(JsArrowFunctionExpression { syntax })
			}
			JS_ASSIGNMENT_EXPRESSION => {
				JsExpression::JsAssignmentExpression(JsAssignmentExpression { syntax })
			}
			JS_AWAIT_EXPRESSION => JsExpression::JsAwaitExpression(JsAwaitExpression { syntax }),
			JS_BINARY_EXPRESSION => JsExpression::JsBinaryExpression(JsBinaryExpression { syntax }),
			JS_CALL_EXPRESSION => JsExpression::JsCallExpression(JsCallExpression { syntax }),
			JS_CLASS_EXPRESSION => JsExpression::JsClassExpression(JsClassExpression { syntax }),
			JS_CONDITIONAL_EXPRESSION => {
				JsExpression::JsConditionalExpression(JsConditionalExpression { syntax })
			}
			JS_DO_EXPRESSION => JsExpression::JsDoExpression(JsDoExpression { syntax }),
			JS_FUNCTION_EXPRESSION => {
				JsExpression::JsFunctionExpression(JsFunctionExpression { syntax })
			}
			JS_IMPORT_CALL => JsExpression::JsImportCall(JsImportCall { syntax }),
			JS_LOGICAL_EXPRESSION => {
				JsExpression::JsLogicalExpression(JsLogicalExpression { syntax })
			}
			JS_MEMBER_EXPRESSION => JsExpression::JsMemberExpression(JsMemberExpression { syntax }),
			JS_NEW_EXPRESSION => JsExpression::JsNewExpression(JsNewExpression { syntax }),
			JS_OBJECT_EXPRESSION => JsExpression::JsObjectExpression(JsObjectExpression { syntax }),
			JS_OPTIONAL_CALL_EXPRESSION => {
				JsExpression::JsOptionalCallExpression(JsOptionalCallExpression { syntax })
			}
			JS_PARENTHESIZED_EXPRESSION => {
				JsExpression::JsParenthesizedExpression(JsParenthesizedExpression { syntax })
			}
			JS_REFERENCE_IDENTIFIER_EXPRESSION => {
				JsExpression::JsReferenceIdentifierExpression(JsReferenceIdentifierExpression {
					syntax,
				})
			}
			JS_SEQUENCE_EXPRESSION => {
				JsExpression::JsSequenceExpression(JsSequenceExpression { syntax })
			}
			JS_SUPER_EXPRESSION => JsExpression::JsSuperExpression(JsSuperExpression { syntax }),
			JS_TAGGED_TEMPLATE_EXPRESSION => {
				JsExpression::JsTaggedTemplateExpression(JsTaggedTemplateExpression { syntax })
			}
			JS_THIS_EXPRESSION => JsExpression::JsThisExpression(JsThisExpression { syntax }),
			JS_UNARY_EXPRESSION => JsExpression::JsUnaryExpression(JsUnaryExpression { syntax }),
			JS_PRE_UPDATE_EXPRESSION => {
				JsExpression::JsPreUpdateExpression(JsPreUpdateExpression { syntax })
			}
			JS_POST_UPDATE_EXPRESSION => {
				JsExpression::JsPostUpdateExpression(JsPostUpdateExpression { syntax })
			}
			JS_YIELD_EXPRESSION => JsExpression::JsYieldExpression(JsYieldExpression { syntax }),
			JS_BOOLEAN_LITERAL => JsExpression::JsBooleanLiteral(JsBooleanLiteral { syntax }),
			JS_BIG_INT_LITERAL => JsExpression::JsBigIntLiteral(JsBigIntLiteral { syntax }),
			JS_NULL_LITERAL => JsExpression::JsNullLiteral(JsNullLiteral { syntax }),
			JS_NUMBER_LITERAL => JsExpression::JsNumberLiteral(JsNumberLiteral { syntax }),
			JS_STRING_LITERAL => JsExpression::JsStringLiteral(JsStringLiteral { syntax }),
			JS_TEMPLATE_LITERAL => JsExpression::JsTemplateLiteral(JsTemplateLiteral { syntax }),
			JS_UNKNOWN_EXPRESSION => {
				JsExpression::JsUnknownExpression(JsUnknownExpression { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsExpression::JsArrayExpression(it) => &it.syntax,
			JsExpression::JsArrowFunctionExpression(it) => &it.syntax,
			JsExpression::JsAssignmentExpression(it) => &it.syntax,
			JsExpression::JsAwaitExpression(it) => &it.syntax,
			JsExpression::JsBinaryExpression(it) => &it.syntax,
			JsExpression::JsCallExpression(it) => &it.syntax,
			JsExpression::JsClassExpression(it) => &it.syntax,
			JsExpression::JsConditionalExpression(it) => &it.syntax,
			JsExpression::JsDoExpression(it) => &it.syntax,
			JsExpression::JsFunctionExpression(it) => &it.syntax,
			JsExpression::JsImportCall(it) => &it.syntax,
			JsExpression::JsLogicalExpression(it) => &it.syntax,
			JsExpression::JsMemberExpression(it) => &it.syntax,
			JsExpression::JsNewExpression(it) => &it.syntax,
			JsExpression::JsObjectExpression(it) => &it.syntax,
			JsExpression::JsOptionalCallExpression(it) => &it.syntax,
			JsExpression::JsParenthesizedExpression(it) => &it.syntax,
			JsExpression::JsReferenceIdentifierExpression(it) => &it.syntax,
			JsExpression::JsSequenceExpression(it) => &it.syntax,
			JsExpression::JsSuperExpression(it) => &it.syntax,
			JsExpression::JsTaggedTemplateExpression(it) => &it.syntax,
			JsExpression::JsThisExpression(it) => &it.syntax,
			JsExpression::JsUnaryExpression(it) => &it.syntax,
			JsExpression::JsPreUpdateExpression(it) => &it.syntax,
			JsExpression::JsPostUpdateExpression(it) => &it.syntax,
			JsExpression::JsYieldExpression(it) => &it.syntax,
			JsExpression::JsBooleanLiteral(it) => &it.syntax,
			JsExpression::JsBigIntLiteral(it) => &it.syntax,
			JsExpression::JsNullLiteral(it) => &it.syntax,
			JsExpression::JsNumberLiteral(it) => &it.syntax,
			JsExpression::JsStringLiteral(it) => &it.syntax,
			JsExpression::JsTemplateLiteral(it) => &it.syntax,
			JsExpression::JsUnknownExpression(it) => &it.syntax,
		}
	}
}
impl From<JsExpression> for JsForInit {
	fn from(node: JsExpression) -> JsForInit { JsForInit::JsExpression(node) }
}
impl From<JsVariableDeclaration> for JsForInit {
	fn from(node: JsVariableDeclaration) -> JsForInit { JsForInit::JsVariableDeclaration(node) }
}
impl AstNode for JsForInit {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(kind, JS_EXPRESSION | JS_VARIABLE_DECLARATION)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_EXPRESSION => JsForInit::JsExpression(JsExpression { syntax }),
			JS_VARIABLE_DECLARATION => {
				JsForInit::JsVariableDeclaration(JsVariableDeclaration { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsForInit::JsExpression(it) => &it.syntax,
			JsForInit::JsVariableDeclaration(it) => &it.syntax,
		}
	}
}
impl From<JsVariableDeclaration> for JsForLeft {
	fn from(node: JsVariableDeclaration) -> JsForLeft { JsForLeft::JsVariableDeclaration(node) }
}
impl From<JsAssignmentTarget> for JsForLeft {
	fn from(node: JsAssignmentTarget) -> JsForLeft { JsForLeft::JsAssignmentTarget(node) }
}
impl AstNode for JsForLeft {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(kind, JS_VARIABLE_DECLARATION | JS_ASSIGNMENT_TARGET)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_VARIABLE_DECLARATION => {
				JsForLeft::JsVariableDeclaration(JsVariableDeclaration { syntax })
			}
			JS_ASSIGNMENT_TARGET => JsForLeft::JsAssignmentTarget(JsAssignmentTarget { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsForLeft::JsVariableDeclaration(it) => &it.syntax,
			JsForLeft::JsAssignmentTarget(it) => &it.syntax,
		}
	}
}
impl From<JsSimpleAssignmentTarget> for JsAssignmentTarget {
	fn from(node: JsSimpleAssignmentTarget) -> JsAssignmentTarget {
		JsAssignmentTarget::JsSimpleAssignmentTarget(node)
	}
}
impl From<JsArrayAssignmentTarget> for JsAssignmentTarget {
	fn from(node: JsArrayAssignmentTarget) -> JsAssignmentTarget {
		JsAssignmentTarget::JsArrayAssignmentTarget(node)
	}
}
impl From<JsObjectAssignmentTarget> for JsAssignmentTarget {
	fn from(node: JsObjectAssignmentTarget) -> JsAssignmentTarget {
		JsAssignmentTarget::JsObjectAssignmentTarget(node)
	}
}
impl AstNode for JsAssignmentTarget {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_SIMPLE_ASSIGNMENT_TARGET | JS_ARRAY_ASSIGNMENT_TARGET | JS_OBJECT_ASSIGNMENT_TARGET
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_SIMPLE_ASSIGNMENT_TARGET => {
				JsAssignmentTarget::JsSimpleAssignmentTarget(JsSimpleAssignmentTarget { syntax })
			}
			JS_ARRAY_ASSIGNMENT_TARGET => {
				JsAssignmentTarget::JsArrayAssignmentTarget(JsArrayAssignmentTarget { syntax })
			}
			JS_OBJECT_ASSIGNMENT_TARGET => {
				JsAssignmentTarget::JsObjectAssignmentTarget(JsObjectAssignmentTarget { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAssignmentTarget::JsSimpleAssignmentTarget(it) => &it.syntax,
			JsAssignmentTarget::JsArrayAssignmentTarget(it) => &it.syntax,
			JsAssignmentTarget::JsObjectAssignmentTarget(it) => &it.syntax,
		}
	}
}
impl From<JsCaseClause> for JsSwitchCase {
	fn from(node: JsCaseClause) -> JsSwitchCase { JsSwitchCase::JsCaseClause(node) }
}
impl From<JsDefaultClause> for JsSwitchCase {
	fn from(node: JsDefaultClause) -> JsSwitchCase { JsSwitchCase::JsDefaultClause(node) }
}
impl AstNode for JsSwitchCase {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, JS_CASE_CLAUSE | JS_DEFAULT_CLAUSE) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_CASE_CLAUSE => JsSwitchCase::JsCaseClause(JsCaseClause { syntax }),
			JS_DEFAULT_CLAUSE => JsSwitchCase::JsDefaultClause(JsDefaultClause { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsSwitchCase::JsCaseClause(it) => &it.syntax,
			JsSwitchCase::JsDefaultClause(it) => &it.syntax,
		}
	}
}
impl From<JsObjectBinding> for JsBinding {
	fn from(node: JsObjectBinding) -> JsBinding { JsBinding::JsObjectBinding(node) }
}
impl From<JsArrayBinding> for JsBinding {
	fn from(node: JsArrayBinding) -> JsBinding { JsBinding::JsArrayBinding(node) }
}
impl From<JsBindingIdentifier> for JsBinding {
	fn from(node: JsBindingIdentifier) -> JsBinding { JsBinding::JsBindingIdentifier(node) }
}
impl AstNode for JsBinding {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_OBJECT_BINDING | JS_ARRAY_BINDING | JS_BINDING_IDENTIFIER
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_OBJECT_BINDING => JsBinding::JsObjectBinding(JsObjectBinding { syntax }),
			JS_ARRAY_BINDING => JsBinding::JsArrayBinding(JsArrayBinding { syntax }),
			JS_BINDING_IDENTIFIER => JsBinding::JsBindingIdentifier(JsBindingIdentifier { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsBinding::JsObjectBinding(it) => &it.syntax,
			JsBinding::JsArrayBinding(it) => &it.syntax,
			JsBinding::JsBindingIdentifier(it) => &it.syntax,
		}
	}
}
impl From<JsArraySpreadElement> for AnyJsArrayElement {
	fn from(node: JsArraySpreadElement) -> AnyJsArrayElement {
		AnyJsArrayElement::JsArraySpreadElement(node)
	}
}
impl From<JsArrayElement> for AnyJsArrayElement {
	fn from(node: JsArrayElement) -> AnyJsArrayElement { AnyJsArrayElement::JsArrayElement(node) }
}
impl From<JsArrayHole> for AnyJsArrayElement {
	fn from(node: JsArrayHole) -> AnyJsArrayElement { AnyJsArrayElement::JsArrayHole(node) }
}
impl AstNode for AnyJsArrayElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_ARRAY_SPREAD_ELEMENT | JS_ARRAY_ELEMENT | JS_ARRAY_HOLE
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_ARRAY_SPREAD_ELEMENT => {
				AnyJsArrayElement::JsArraySpreadElement(JsArraySpreadElement { syntax })
			}
			JS_ARRAY_ELEMENT => AnyJsArrayElement::JsArrayElement(JsArrayElement { syntax }),
			JS_ARRAY_HOLE => AnyJsArrayElement::JsArrayHole(JsArrayHole { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			AnyJsArrayElement::JsArraySpreadElement(it) => &it.syntax,
			AnyJsArrayElement::JsArrayElement(it) => &it.syntax,
			AnyJsArrayElement::JsArrayHole(it) => &it.syntax,
		}
	}
}
impl From<JsParameterList> for JsArrowFunctionParameters {
	fn from(node: JsParameterList) -> JsArrowFunctionParameters {
		JsArrowFunctionParameters::JsParameterList(node)
	}
}
impl From<JsParameter> for JsArrowFunctionParameters {
	fn from(node: JsParameter) -> JsArrowFunctionParameters {
		JsArrowFunctionParameters::JsParameter(node)
	}
}
impl AstNode for JsArrowFunctionParameters {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, JS_PARAMETER_LIST | JS_PARAMETER) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_PARAMETER_LIST => {
				JsArrowFunctionParameters::JsParameterList(JsParameterList { syntax })
			}
			JS_PARAMETER => JsArrowFunctionParameters::JsParameter(JsParameter { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsArrowFunctionParameters::JsParameterList(it) => &it.syntax,
			JsArrowFunctionParameters::JsParameter(it) => &it.syntax,
		}
	}
}
impl From<JsStatement> for JsStatementOrExpression {
	fn from(node: JsStatement) -> JsStatementOrExpression {
		JsStatementOrExpression::JsStatement(node)
	}
}
impl From<JsExpression> for JsStatementOrExpression {
	fn from(node: JsExpression) -> JsStatementOrExpression {
		JsStatementOrExpression::JsExpression(node)
	}
}
impl AstNode for JsStatementOrExpression {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, JS_STATEMENT | JS_EXPRESSION) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_STATEMENT => JsStatementOrExpression::JsStatement(JsStatement { syntax }),
			JS_EXPRESSION => JsStatementOrExpression::JsExpression(JsExpression { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsStatementOrExpression::JsStatement(it) => &it.syntax,
			JsStatementOrExpression::JsExpression(it) => &it.syntax,
		}
	}
}
impl From<JsExpression> for JsExpressionOrSpread {
	fn from(node: JsExpression) -> JsExpressionOrSpread { JsExpressionOrSpread::JsExpression(node) }
}
impl From<JsSpread> for JsExpressionOrSpread {
	fn from(node: JsSpread) -> JsExpressionOrSpread { JsExpressionOrSpread::JsSpread(node) }
}
impl AstNode for JsExpressionOrSpread {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, JS_EXPRESSION | JS_SPREAD) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_EXPRESSION => JsExpressionOrSpread::JsExpression(JsExpression { syntax }),
			JS_SPREAD => JsExpressionOrSpread::JsSpread(JsSpread { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsExpressionOrSpread::JsExpression(it) => &it.syntax,
			JsExpressionOrSpread::JsSpread(it) => &it.syntax,
		}
	}
}
impl From<JsStaticMember> for JsMember {
	fn from(node: JsStaticMember) -> JsMember { JsMember::JsStaticMember(node) }
}
impl From<JsComputedMember> for JsMember {
	fn from(node: JsComputedMember) -> JsMember { JsMember::JsComputedMember(node) }
}
impl AstNode for JsMember {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, JS_STATIC_MEMBER | JS_COMPUTED_MEMBER) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_STATIC_MEMBER => JsMember::JsStaticMember(JsStaticMember { syntax }),
			JS_COMPUTED_MEMBER => JsMember::JsComputedMember(JsComputedMember { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsMember::JsStaticMember(it) => &it.syntax,
			JsMember::JsComputedMember(it) => &it.syntax,
		}
	}
}
impl From<JsIdentifier> for JsStaticMemberName {
	fn from(node: JsIdentifier) -> JsStaticMemberName { JsStaticMemberName::JsIdentifier(node) }
}
impl From<JsStringLiteral> for JsStaticMemberName {
	fn from(node: JsStringLiteral) -> JsStaticMemberName {
		JsStaticMemberName::JsStringLiteral(node)
	}
}
impl From<JsNumberLiteral> for JsStaticMemberName {
	fn from(node: JsNumberLiteral) -> JsStaticMemberName {
		JsStaticMemberName::JsNumberLiteral(node)
	}
}
impl AstNode for JsStaticMemberName {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(kind, JS_IDENTIFIER | JS_STRING_LITERAL | JS_NUMBER_LITERAL)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_IDENTIFIER => JsStaticMemberName::JsIdentifier(JsIdentifier { syntax }),
			JS_STRING_LITERAL => JsStaticMemberName::JsStringLiteral(JsStringLiteral { syntax }),
			JS_NUMBER_LITERAL => JsStaticMemberName::JsNumberLiteral(JsNumberLiteral { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsStaticMemberName::JsIdentifier(it) => &it.syntax,
			JsStaticMemberName::JsStringLiteral(it) => &it.syntax,
			JsStaticMemberName::JsNumberLiteral(it) => &it.syntax,
		}
	}
}
impl From<JsMemberAssignmentTarget> for JsSimpleAssignmentTarget {
	fn from(node: JsMemberAssignmentTarget) -> JsSimpleAssignmentTarget {
		JsSimpleAssignmentTarget::JsMemberAssignmentTarget(node)
	}
}
impl From<JsIdentifierAssignmentTarget> for JsSimpleAssignmentTarget {
	fn from(node: JsIdentifierAssignmentTarget) -> JsSimpleAssignmentTarget {
		JsSimpleAssignmentTarget::JsIdentifierAssignmentTarget(node)
	}
}
impl AstNode for JsSimpleAssignmentTarget {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_MEMBER_ASSIGNMENT_TARGET | JS_IDENTIFIER_ASSIGNMENT_TARGET
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_MEMBER_ASSIGNMENT_TARGET => {
				JsSimpleAssignmentTarget::JsMemberAssignmentTarget(JsMemberAssignmentTarget {
					syntax,
				})
			}
			JS_IDENTIFIER_ASSIGNMENT_TARGET => {
				JsSimpleAssignmentTarget::JsIdentifierAssignmentTarget(
					JsIdentifierAssignmentTarget { syntax },
				)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsSimpleAssignmentTarget::JsMemberAssignmentTarget(it) => &it.syntax,
			JsSimpleAssignmentTarget::JsIdentifierAssignmentTarget(it) => &it.syntax,
		}
	}
}
impl From<JsStringTemplateElement> for JsTemplateElement {
	fn from(node: JsStringTemplateElement) -> JsTemplateElement {
		JsTemplateElement::JsStringTemplateElement(node)
	}
}
impl From<JsExpression> for JsTemplateElement {
	fn from(node: JsExpression) -> JsTemplateElement { JsTemplateElement::JsExpression(node) }
}
impl AstNode for JsTemplateElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(kind, JS_STRING_TEMPLATE_ELEMENT | JS_EXPRESSION)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_STRING_TEMPLATE_ELEMENT => {
				JsTemplateElement::JsStringTemplateElement(JsStringTemplateElement { syntax })
			}
			JS_EXPRESSION => JsTemplateElement::JsExpression(JsExpression { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsTemplateElement::JsStringTemplateElement(it) => &it.syntax,
			JsTemplateElement::JsExpression(it) => &it.syntax,
		}
	}
}
impl From<JsConstructorClassMember> for JsClassMember {
	fn from(node: JsConstructorClassMember) -> JsClassMember {
		JsClassMember::JsConstructorClassMember(node)
	}
}
impl From<JsPropertyClassMember> for JsClassMember {
	fn from(node: JsPropertyClassMember) -> JsClassMember {
		JsClassMember::JsPropertyClassMember(node)
	}
}
impl From<JsPrivatePropertyClassMember> for JsClassMember {
	fn from(node: JsPrivatePropertyClassMember) -> JsClassMember {
		JsClassMember::JsPrivatePropertyClassMember(node)
	}
}
impl From<JsMethodClassMember> for JsClassMember {
	fn from(node: JsMethodClassMember) -> JsClassMember { JsClassMember::JsMethodClassMember(node) }
}
impl From<JsGetterClassMember> for JsClassMember {
	fn from(node: JsGetterClassMember) -> JsClassMember { JsClassMember::JsGetterClassMember(node) }
}
impl From<JsSetterClassMember> for JsClassMember {
	fn from(node: JsSetterClassMember) -> JsClassMember { JsClassMember::JsSetterClassMember(node) }
}
impl From<JsUnknownMember> for JsClassMember {
	fn from(node: JsUnknownMember) -> JsClassMember { JsClassMember::JsUnknownMember(node) }
}
impl AstNode for JsClassMember {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_CONSTRUCTOR_CLASS_MEMBER
				| JS_PROPERTY_CLASS_MEMBER
				| JS_PRIVATE_PROPERTY_CLASS_MEMBER
				| JS_METHOD_CLASS_MEMBER
				| JS_GETTER_CLASS_MEMBER
				| JS_SETTER_CLASS_MEMBER
				| JS_UNKNOWN_MEMBER
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_CONSTRUCTOR_CLASS_MEMBER => {
				JsClassMember::JsConstructorClassMember(JsConstructorClassMember { syntax })
			}
			JS_PROPERTY_CLASS_MEMBER => {
				JsClassMember::JsPropertyClassMember(JsPropertyClassMember { syntax })
			}
			JS_PRIVATE_PROPERTY_CLASS_MEMBER => {
				JsClassMember::JsPrivatePropertyClassMember(JsPrivatePropertyClassMember { syntax })
			}
			JS_METHOD_CLASS_MEMBER => {
				JsClassMember::JsMethodClassMember(JsMethodClassMember { syntax })
			}
			JS_GETTER_CLASS_MEMBER => {
				JsClassMember::JsGetterClassMember(JsGetterClassMember { syntax })
			}
			JS_SETTER_CLASS_MEMBER => {
				JsClassMember::JsSetterClassMember(JsSetterClassMember { syntax })
			}
			JS_UNKNOWN_MEMBER => JsClassMember::JsUnknownMember(JsUnknownMember { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsClassMember::JsConstructorClassMember(it) => &it.syntax,
			JsClassMember::JsPropertyClassMember(it) => &it.syntax,
			JsClassMember::JsPrivatePropertyClassMember(it) => &it.syntax,
			JsClassMember::JsMethodClassMember(it) => &it.syntax,
			JsClassMember::JsGetterClassMember(it) => &it.syntax,
			JsClassMember::JsSetterClassMember(it) => &it.syntax,
			JsClassMember::JsUnknownMember(it) => &it.syntax,
		}
	}
}
impl From<JsObjectMemberKey> for JsClassMemberName {
	fn from(node: JsObjectMemberKey) -> JsClassMemberName {
		JsClassMemberName::JsObjectMemberKey(node)
	}
}
impl From<JsPrivateClassMemberName> for JsClassMemberName {
	fn from(node: JsPrivateClassMemberName) -> JsClassMemberName {
		JsClassMemberName::JsPrivateClassMemberName(node)
	}
}
impl AstNode for JsClassMemberName {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(kind, JS_OBJECT_MEMBER_KEY | JS_PRIVATE_CLASS_MEMBER_NAME)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_OBJECT_MEMBER_KEY => {
				JsClassMemberName::JsObjectMemberKey(JsObjectMemberKey { syntax })
			}
			JS_PRIVATE_CLASS_MEMBER_NAME => {
				JsClassMemberName::JsPrivateClassMemberName(JsPrivateClassMemberName { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsClassMemberName::JsObjectMemberKey(it) => &it.syntax,
			JsClassMemberName::JsPrivateClassMemberName(it) => &it.syntax,
		}
	}
}
impl From<JsStaticMemberName> for JsObjectMemberKey {
	fn from(node: JsStaticMemberName) -> JsObjectMemberKey {
		JsObjectMemberKey::JsStaticMemberName(node)
	}
}
impl From<JsComputedMember> for JsObjectMemberKey {
	fn from(node: JsComputedMember) -> JsObjectMemberKey {
		JsObjectMemberKey::JsComputedMember(node)
	}
}
impl AstNode for JsObjectMemberKey {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(kind, JS_STATIC_MEMBER_NAME | JS_COMPUTED_MEMBER)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_STATIC_MEMBER_NAME => {
				JsObjectMemberKey::JsStaticMemberName(JsStaticMemberName { syntax })
			}
			JS_COMPUTED_MEMBER => JsObjectMemberKey::JsComputedMember(JsComputedMember { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsObjectMemberKey::JsStaticMemberName(it) => &it.syntax,
			JsObjectMemberKey::JsComputedMember(it) => &it.syntax,
		}
	}
}
impl From<JsMethodObjectMember> for JsObjectMember {
	fn from(node: JsMethodObjectMember) -> JsObjectMember {
		JsObjectMember::JsMethodObjectMember(node)
	}
}
impl From<JsGetterObjectMember> for JsObjectMember {
	fn from(node: JsGetterObjectMember) -> JsObjectMember {
		JsObjectMember::JsGetterObjectMember(node)
	}
}
impl From<JsSetterObjectMember> for JsObjectMember {
	fn from(node: JsSetterObjectMember) -> JsObjectMember {
		JsObjectMember::JsSetterObjectMember(node)
	}
}
impl From<JsPropertyObjectMember> for JsObjectMember {
	fn from(node: JsPropertyObjectMember) -> JsObjectMember {
		JsObjectMember::JsPropertyObjectMember(node)
	}
}
impl From<JsSpreadObjectMember> for JsObjectMember {
	fn from(node: JsSpreadObjectMember) -> JsObjectMember {
		JsObjectMember::JsSpreadObjectMember(node)
	}
}
impl From<JsUnknownMember> for JsObjectMember {
	fn from(node: JsUnknownMember) -> JsObjectMember { JsObjectMember::JsUnknownMember(node) }
}
impl AstNode for JsObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_METHOD_OBJECT_MEMBER
				| JS_GETTER_OBJECT_MEMBER
				| JS_SETTER_OBJECT_MEMBER
				| JS_PROPERTY_OBJECT_MEMBER
				| JS_SPREAD_OBJECT_MEMBER
				| JS_UNKNOWN_MEMBER
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_METHOD_OBJECT_MEMBER => {
				JsObjectMember::JsMethodObjectMember(JsMethodObjectMember { syntax })
			}
			JS_GETTER_OBJECT_MEMBER => {
				JsObjectMember::JsGetterObjectMember(JsGetterObjectMember { syntax })
			}
			JS_SETTER_OBJECT_MEMBER => {
				JsObjectMember::JsSetterObjectMember(JsSetterObjectMember { syntax })
			}
			JS_PROPERTY_OBJECT_MEMBER => {
				JsObjectMember::JsPropertyObjectMember(JsPropertyObjectMember { syntax })
			}
			JS_SPREAD_OBJECT_MEMBER => {
				JsObjectMember::JsSpreadObjectMember(JsSpreadObjectMember { syntax })
			}
			JS_UNKNOWN_MEMBER => JsObjectMember::JsUnknownMember(JsUnknownMember { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsObjectMember::JsMethodObjectMember(it) => &it.syntax,
			JsObjectMember::JsGetterObjectMember(it) => &it.syntax,
			JsObjectMember::JsSetterObjectMember(it) => &it.syntax,
			JsObjectMember::JsPropertyObjectMember(it) => &it.syntax,
			JsObjectMember::JsSpreadObjectMember(it) => &it.syntax,
			JsObjectMember::JsUnknownMember(it) => &it.syntax,
		}
	}
}
impl From<JsFunctionDeclaration> for JsExportDeclaration {
	fn from(node: JsFunctionDeclaration) -> JsExportDeclaration {
		JsExportDeclaration::JsFunctionDeclaration(node)
	}
}
impl From<JsClassDeclaration> for JsExportDeclaration {
	fn from(node: JsClassDeclaration) -> JsExportDeclaration {
		JsExportDeclaration::JsClassDeclaration(node)
	}
}
impl From<JsVariableDeclarationStatement> for JsExportDeclaration {
	fn from(node: JsVariableDeclarationStatement) -> JsExportDeclaration {
		JsExportDeclaration::JsVariableDeclarationStatement(node)
	}
}
impl AstNode for JsExportDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_FUNCTION_DECLARATION | JS_CLASS_DECLARATION | JS_VARIABLE_DECLARATION_STATEMENT
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_FUNCTION_DECLARATION => {
				JsExportDeclaration::JsFunctionDeclaration(JsFunctionDeclaration { syntax })
			}
			JS_CLASS_DECLARATION => {
				JsExportDeclaration::JsClassDeclaration(JsClassDeclaration { syntax })
			}
			JS_VARIABLE_DECLARATION_STATEMENT => {
				JsExportDeclaration::JsVariableDeclarationStatement(
					JsVariableDeclarationStatement { syntax },
				)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsExportDeclaration::JsFunctionDeclaration(it) => &it.syntax,
			JsExportDeclaration::JsClassDeclaration(it) => &it.syntax,
			JsExportDeclaration::JsVariableDeclarationStatement(it) => &it.syntax,
		}
	}
}
impl From<JsClassDeclaration> for JsExportDefaultArgument {
	fn from(node: JsClassDeclaration) -> JsExportDefaultArgument {
		JsExportDefaultArgument::JsClassDeclaration(node)
	}
}
impl From<JsFunctionDeclaration> for JsExportDefaultArgument {
	fn from(node: JsFunctionDeclaration) -> JsExportDefaultArgument {
		JsExportDefaultArgument::JsFunctionDeclaration(node)
	}
}
impl From<JsExportDefaultExpressionArgument> for JsExportDefaultArgument {
	fn from(node: JsExportDefaultExpressionArgument) -> JsExportDefaultArgument {
		JsExportDefaultArgument::JsExportDefaultExpressionArgument(node)
	}
}
impl AstNode for JsExportDefaultArgument {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_CLASS_DECLARATION | JS_FUNCTION_DECLARATION | JS_EXPORT_DEFAULT_EXPRESSION_ARGUMENT
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_CLASS_DECLARATION => {
				JsExportDefaultArgument::JsClassDeclaration(JsClassDeclaration { syntax })
			}
			JS_FUNCTION_DECLARATION => {
				JsExportDefaultArgument::JsFunctionDeclaration(JsFunctionDeclaration { syntax })
			}
			JS_EXPORT_DEFAULT_EXPRESSION_ARGUMENT => {
				JsExportDefaultArgument::JsExportDefaultExpressionArgument(
					JsExportDefaultExpressionArgument { syntax },
				)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsExportDefaultArgument::JsClassDeclaration(it) => &it.syntax,
			JsExportDefaultArgument::JsFunctionDeclaration(it) => &it.syntax,
			JsExportDefaultArgument::JsExportDefaultExpressionArgument(it) => &it.syntax,
		}
	}
}
impl From<JsImportDefaultBinding> for JsImportClause {
	fn from(node: JsImportDefaultBinding) -> JsImportClause {
		JsImportClause::JsImportDefaultBinding(node)
	}
}
impl From<JsNamespaceImportClause> for JsImportClause {
	fn from(node: JsNamespaceImportClause) -> JsImportClause {
		JsImportClause::JsNamespaceImportClause(node)
	}
}
impl From<JsNamedImportClause> for JsImportClause {
	fn from(node: JsNamedImportClause) -> JsImportClause {
		JsImportClause::JsNamedImportClause(node)
	}
}
impl AstNode for JsImportClause {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_IMPORT_DEFAULT_BINDING | JS_NAMESPACE_IMPORT_CLAUSE | JS_NAMED_IMPORT_CLAUSE
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_IMPORT_DEFAULT_BINDING => {
				JsImportClause::JsImportDefaultBinding(JsImportDefaultBinding { syntax })
			}
			JS_NAMESPACE_IMPORT_CLAUSE => {
				JsImportClause::JsNamespaceImportClause(JsNamespaceImportClause { syntax })
			}
			JS_NAMED_IMPORT_CLAUSE => {
				JsImportClause::JsNamedImportClause(JsNamedImportClause { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsImportClause::JsImportDefaultBinding(it) => &it.syntax,
			JsImportClause::JsNamespaceImportClause(it) => &it.syntax,
			JsImportClause::JsNamedImportClause(it) => &it.syntax,
		}
	}
}
impl From<JsArrayHole> for AnyJsArrayBindingElement {
	fn from(node: JsArrayHole) -> AnyJsArrayBindingElement {
		AnyJsArrayBindingElement::JsArrayHole(node)
	}
}
impl From<JsArrayBindingElement> for AnyJsArrayBindingElement {
	fn from(node: JsArrayBindingElement) -> AnyJsArrayBindingElement {
		AnyJsArrayBindingElement::JsArrayBindingElement(node)
	}
}
impl AstNode for AnyJsArrayBindingElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(kind, JS_ARRAY_HOLE | JS_ARRAY_BINDING_ELEMENT)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_ARRAY_HOLE => AnyJsArrayBindingElement::JsArrayHole(JsArrayHole { syntax }),
			JS_ARRAY_BINDING_ELEMENT => {
				AnyJsArrayBindingElement::JsArrayBindingElement(JsArrayBindingElement { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			AnyJsArrayBindingElement::JsArrayHole(it) => &it.syntax,
			AnyJsArrayBindingElement::JsArrayBindingElement(it) => &it.syntax,
		}
	}
}
impl From<JsShorthandPropertyBinding> for AnyJsPropertyBinding {
	fn from(node: JsShorthandPropertyBinding) -> AnyJsPropertyBinding {
		AnyJsPropertyBinding::JsShorthandPropertyBinding(node)
	}
}
impl From<JsPropertyBinding> for AnyJsPropertyBinding {
	fn from(node: JsPropertyBinding) -> AnyJsPropertyBinding {
		AnyJsPropertyBinding::JsPropertyBinding(node)
	}
}
impl AstNode for AnyJsPropertyBinding {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(kind, JS_SHORTHAND_PROPERTY_BINDING | JS_PROPERTY_BINDING)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_SHORTHAND_PROPERTY_BINDING => {
				AnyJsPropertyBinding::JsShorthandPropertyBinding(JsShorthandPropertyBinding {
					syntax,
				})
			}
			JS_PROPERTY_BINDING => {
				AnyJsPropertyBinding::JsPropertyBinding(JsPropertyBinding { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			AnyJsPropertyBinding::JsShorthandPropertyBinding(it) => &it.syntax,
			AnyJsPropertyBinding::JsPropertyBinding(it) => &it.syntax,
		}
	}
}
impl From<JsArrayHole> for AnyJsArrayAssignmentTargetElement {
	fn from(node: JsArrayHole) -> AnyJsArrayAssignmentTargetElement {
		AnyJsArrayAssignmentTargetElement::JsArrayHole(node)
	}
}
impl From<JsArrayAssignmentTargetElement> for AnyJsArrayAssignmentTargetElement {
	fn from(node: JsArrayAssignmentTargetElement) -> AnyJsArrayAssignmentTargetElement {
		AnyJsArrayAssignmentTargetElement::JsArrayAssignmentTargetElement(node)
	}
}
impl AstNode for AnyJsArrayAssignmentTargetElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(kind, JS_ARRAY_HOLE | JS_ARRAY_ASSIGNMENT_TARGET_ELEMENT)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_ARRAY_HOLE => AnyJsArrayAssignmentTargetElement::JsArrayHole(JsArrayHole { syntax }),
			JS_ARRAY_ASSIGNMENT_TARGET_ELEMENT => {
				AnyJsArrayAssignmentTargetElement::JsArrayAssignmentTargetElement(
					JsArrayAssignmentTargetElement { syntax },
				)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			AnyJsArrayAssignmentTargetElement::JsArrayHole(it) => &it.syntax,
			AnyJsArrayAssignmentTargetElement::JsArrayAssignmentTargetElement(it) => &it.syntax,
		}
	}
}
impl From<JsShorthandPropertyAssignmentTarget> for JsPropertyAssignmentTarget {
	fn from(node: JsShorthandPropertyAssignmentTarget) -> JsPropertyAssignmentTarget {
		JsPropertyAssignmentTarget::JsShorthandPropertyAssignmentTarget(node)
	}
}
impl From<JsObjectPropertyAssignmentTarget> for JsPropertyAssignmentTarget {
	fn from(node: JsObjectPropertyAssignmentTarget) -> JsPropertyAssignmentTarget {
		JsPropertyAssignmentTarget::JsObjectPropertyAssignmentTarget(node)
	}
}
impl AstNode for JsPropertyAssignmentTarget {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_SHORTHAND_PROPERTY_ASSIGNMENT_TARGET | JS_OBJECT_PROPERTY_ASSIGNMENT_TARGET
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_SHORTHAND_PROPERTY_ASSIGNMENT_TARGET => {
				JsPropertyAssignmentTarget::JsShorthandPropertyAssignmentTarget(
					JsShorthandPropertyAssignmentTarget { syntax },
				)
			}
			JS_OBJECT_PROPERTY_ASSIGNMENT_TARGET => {
				JsPropertyAssignmentTarget::JsObjectPropertyAssignmentTarget(
					JsObjectPropertyAssignmentTarget { syntax },
				)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsPropertyAssignmentTarget::JsShorthandPropertyAssignmentTarget(it) => &it.syntax,
			JsPropertyAssignmentTarget::JsObjectPropertyAssignmentTarget(it) => &it.syntax,
		}
	}
}
impl From<JsFunctionDeclaration> for JsFunction {
	fn from(node: JsFunctionDeclaration) -> JsFunction { JsFunction::JsFunctionDeclaration(node) }
}
impl From<JsFunctionExpression> for JsFunction {
	fn from(node: JsFunctionExpression) -> JsFunction { JsFunction::JsFunctionExpression(node) }
}
impl From<JsArrowFunctionExpression> for JsFunction {
	fn from(node: JsArrowFunctionExpression) -> JsFunction {
		JsFunction::JsArrowFunctionExpression(node)
	}
}
impl AstNode for JsFunction {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_FUNCTION_DECLARATION | JS_FUNCTION_EXPRESSION | JS_ARROW_FUNCTION_EXPRESSION
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_FUNCTION_DECLARATION => {
				JsFunction::JsFunctionDeclaration(JsFunctionDeclaration { syntax })
			}
			JS_FUNCTION_EXPRESSION => {
				JsFunction::JsFunctionExpression(JsFunctionExpression { syntax })
			}
			JS_ARROW_FUNCTION_EXPRESSION => {
				JsFunction::JsArrowFunctionExpression(JsArrowFunctionExpression { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsFunction::JsFunctionDeclaration(it) => &it.syntax,
			JsFunction::JsFunctionExpression(it) => &it.syntax,
			JsFunction::JsArrowFunctionExpression(it) => &it.syntax,
		}
	}
}
impl std::fmt::Display for JsStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SyntaxElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsForInit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsForLeft {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAssignmentTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsSwitchCase {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AnyJsArrayElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrowFunctionParameters {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsStatementOrExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsExpressionOrSpread {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsStaticMemberName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsSimpleAssignmentTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsTemplateElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsClassMemberName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectMemberKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsExportDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsExportDefaultArgument {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsImportClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AnyJsArrayBindingElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AnyJsPropertyBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AnyJsArrayAssignmentTargetElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPropertyAssignmentTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsFunction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsScript {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsModule {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SyntaxNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SyntaxToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsBlockStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsBreakStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsClassDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsDebuggerStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsEmptyStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsExpressionStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsForStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsForInStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsForOfStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsFunctionDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsIfStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsLabeledStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsReturnStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsSwitchStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsTryStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsVariableDeclarationStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsWhileStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsWithStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsExport {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsExportDefault {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsExportFrom {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsExportAllFrom {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsImport {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsImportModule {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsContinueStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsDoWhileStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsVariableDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsBindingIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsParameterList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsFunctionBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsElseClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsCaseClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsDefaultClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsThrowStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsCatchClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsTryFinallyStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsFinallyClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsCatchDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsVariableDeclarator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrayExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrowFunctionExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAssignmentExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAwaitExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsBinaryExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsCallExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsClassExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsConditionalExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsDoExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsFunctionExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsImportCall {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsLogicalExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsMemberExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsNewExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsOptionalCallExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsParenthesizedExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsReferenceIdentifierExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsSequenceExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsSuperExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsTaggedTemplateExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsThisExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnaryExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPreUpdateExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPostUpdateExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsYieldExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsBooleanLiteral {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsBigIntLiteral {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsNullLiteral {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsNumberLiteral {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsStringLiteral {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsTemplateLiteral {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArraySpreadElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrayElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrayHole {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsCallArgument {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsSpread {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsStaticMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsComputedMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsStringTemplateElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsExtendsClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsConstructorClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPropertyClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPrivatePropertyClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsMethodClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsGetterClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsSetterClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPrivateClassMemberName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPropertyClassMemberInitializer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPrivateMethodClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsMethodObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsGetterObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsSetterObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPropertyObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsSpreadObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsExportFromSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsExportName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsExportDefaultExpressionArgument {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsImportDefaultBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsNamespaceImportClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsNamedImportClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsImportSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsImportBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrayBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsDefaultValueClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrayRestBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrayBindingElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectRestBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsShorthandPropertyBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPropertyBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsMemberAssignmentTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsIdentifierAssignmentTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrayAssignmentTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectAssignmentTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrayAssignmentRest {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrayAssignmentTargetElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectRestAssignmentTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsShorthandPropertyAssignmentTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectPropertyAssignmentTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsRestParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
