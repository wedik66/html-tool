use crate::{
	format_elements, group_elements, soft_indent, FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::Condition;

impl ToFormatElement for Condition {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		Some(group_elements(format_elements![
			formatter.format_token(&self.l_paren_token()?)?,
			soft_indent(formatter.format_node(self.condition()?)?),
			formatter.format_token(&self.r_paren_token()?)?
		]))
	}
}
