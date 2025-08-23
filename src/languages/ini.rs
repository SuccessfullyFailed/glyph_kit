use crate::{ TextMatchResult, TextMatcher, TextMatcherSet };



pub struct IniParser {
	matcher_set:TextMatcherSet
}
impl IniParser {

	/// Create a new ini parser.
	pub fn new() -> IniParser {
		IniParser {
			matcher_set: TextMatcherSet::new().with_matchers(vec![
				(
					"group",
					TextMatcher::new("[") +
					TextMatcher::named("category_name", TextMatcher::optional_repeat_max(!TextMatcher::new("]"))) +
					"]" +

					TextMatcher::optional_repeat_max(
						TextMatcher::named("whitespace", TextMatcher::optional_repeat_max(TextMatcher::whitespace())) +

						TextMatcher::named("variable_row", 
							TextMatcher::named("name", !(TextMatcher::whitespace() | "[") + TextMatcher::repeat_max(!TextMatcher::new("="))) +
							TextMatcher::new("=") +
							TextMatcher::named("value", TextMatcher::optional_repeat_max(!TextMatcher::linebreak()))
						)
					)
				),
				(
					"whitespace",
					TextMatcher::repeat_max(TextMatcher::whitespace())
				)
			])
		}
	}

	/// Parse some text.
	pub fn parse(&self, text:&str) -> TextMatchResult {
		self.matcher_set.multi_match_text(text)
	}
}