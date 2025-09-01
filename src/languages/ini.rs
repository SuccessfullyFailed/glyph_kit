use crate::{ MatchHit, MatchExpr, MatcherRegistry };



const CATEGORY_ID:&str = "category_name";
const VARIABLE_NAME_ID:&str = "name";
const VARIABLE_VALUE_ID:&str = "value";



type StringFormatter = &'static dyn Fn(&str) -> String;
pub struct IniParser {
	matcher_set:MatcherRegistry,
	formatter:Option<StringFormatter>
}
impl IniParser {

	/// Create a new ini parser.
	pub fn new() -> IniParser {
		IniParser {
			matcher_set: MatcherRegistry::new().with_matchers(vec![
				(
					"group",
					MatchExpr::new("[") +
					MatchExpr::named(CATEGORY_ID, MatchExpr::optional_repeat_max(!MatchExpr::new("]"))) +
					"]" +

					MatchExpr::optional_repeat_max(
						MatchExpr::named("whitespace", MatchExpr::optional_repeat_max(MatchExpr::whitespace())) +

						MatchExpr::named("variable_row", 
							MatchExpr::named(VARIABLE_NAME_ID, !(MatchExpr::whitespace() | "[") + MatchExpr::repeat_max(!MatchExpr::new("="))) +
							MatchExpr::new("=") +
							MatchExpr::named(VARIABLE_VALUE_ID, MatchExpr::optional_repeat_max(!MatchExpr::linebreak()))
						)
					)
				),
				(
					"whitespace",
					MatchExpr::repeat_max(MatchExpr::whitespace())
				)
			]),
			formatter: None
		}
	}

	/// Return self with a function that will format all values.
	pub fn with_value_formatter(mut self, formatter:StringFormatter) -> Self {
		self.formatter = Some(formatter);
		self
	}

	/// Parse some text.
	pub fn parse(&self, text:&str) -> MatchHit {
		let mut results = self.matcher_set.multi_match_text(text);
		if let Some(formatter) = self.formatter {
			results.execute_recursive_mut(|text_match| {
				if text_match.type_name == VARIABLE_VALUE_ID {
					text_match.contents = formatter(&text_match.contents);
				}
			});
		}
		results
	}
}