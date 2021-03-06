use crate::{empty_element, format_elements, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::AssignPattern;

impl ToFormatElement for AssignPattern {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let key = if let Some(key) = self.key() {
			formatter.format_node(key)?
		} else {
			empty_element()
		};
		let assign = if let Some(eq_token) = self.eq_token() {
			formatter.format_token(&eq_token)?
		} else if let Some(colon_token) = self.colon_token() {
			formatter.format_token(&colon_token)?
		} else {
			empty_element()
		};

		let value = formatter.format_node(self.value()?)?;
		Some(format_elements![key, assign, value])
	}
}
