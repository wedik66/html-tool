use crate::{
	empty_element, format_elements, space_token, token, FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::{CatchClause, Finalizer, TryStmt};

impl ToFormatElement for TryStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let try_token = formatter.format_token(&self.try_token()?)?;
		let test = formatter.format_node(self.test()?)?;
		let handler = if let Some(catch_clause) = self.handler() {
			format_elements![space_token(), formatter.format_node(catch_clause)?]
		} else {
			empty_element()
		};
		let finalizer = if let Some(finally_node) = self.finalizer() {
			format_elements![space_token(), formatter.format_node(finally_node)?]
		} else {
			empty_element()
		};
		if handler.is_empty() && finalizer.is_empty() {
			None
		} else {
			Some(format_elements![
				try_token,
				space_token(),
				test,
				handler,
				finalizer
			])
		}
	}
}

impl ToFormatElement for Finalizer {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let cons = formatter.format_node(self.cons()?)?;
		let finally = formatter.format_token(&self.finally_token()?)?;
		Some(format_elements![finally, space_token(), cons])
	}
}

impl ToFormatElement for CatchClause {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let l_paren = self.l_paren_token();
		let r_paren = self.r_paren_token();
		let error = self.error();
		let cons = formatter.format_node(self.cons()?)?;
		let catch_token = formatter.format_token(&self.catch_token()?)?;
		match (l_paren, r_paren, error) {
			(None, None, None) => Some(format_elements![token("catch"), space_token(), cons]),
			(Some(l_paren), Some(r_paren), Some(error)) => Some(format_elements![
				catch_token,
				space_token(),
				formatter.format_token(&l_paren)?,
				formatter.format_node(error)?,
				formatter.format_token(&r_paren)?,
				space_token(),
				cons
			]),
			_ => {
				// Here we return None, because a valid catch clause must have a condition or no condition at all:
				// - catch (e) {}
				// - catch {}
				//
				// Other cases should fail.
				None
			}
		}
	}
}
