use crate::{ TextMatchResult, TextMatcher, TextMatcherSet };



const CATEGORY_ID:&str = "category_name";
const VARIABLE_NAME_ID:&str = "name";
const VARIABLE_VALUE_ID:&str = "value";



type StringFormatter = &'static dyn Fn(&str) -> String;
pub struct IniParser {
	matcher_set:TextMatcherSet,
	formatter:Option<StringFormatter>
}
impl IniParser {

	/// Create a new ini parser.
	pub fn new() -> IniParser {
		IniParser {
			matcher_set: TextMatcherSet::new().with_matchers(vec![
				(
					"group",
					TextMatcher::new("[") +
					TextMatcher::named(CATEGORY_ID, TextMatcher::optional_repeat_max(!TextMatcher::new("]"))) +
					"]" +

					TextMatcher::optional_repeat_max(
						TextMatcher::named("whitespace", TextMatcher::optional_repeat_max(TextMatcher::whitespace())) +

						TextMatcher::named("variable_row", 
							TextMatcher::named(VARIABLE_NAME_ID, !(TextMatcher::whitespace() | "[") + TextMatcher::repeat_max(!TextMatcher::new("="))) +
							TextMatcher::new("=") +
							TextMatcher::named(VARIABLE_VALUE_ID, TextMatcher::optional_repeat_max(!TextMatcher::linebreak()))
						)
					)
				),
				(
					"whitespace",
					TextMatcher::repeat_max(TextMatcher::whitespace())
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
	pub fn parse(&self, text:&str) -> TextMatchResult {
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