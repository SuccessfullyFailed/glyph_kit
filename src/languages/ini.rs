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
					"category",
					TextMatcher::new("[") +
					TextMatcher::optional_repeat_max(!TextMatcher::new("]")) +
					"]"
				),
				(
					"value",
					!TextMatcher::whitespace() + 
					TextMatcher::repeat_max(!TextMatcher::new("=")) +
					TextMatcher::new("=") +
					TextMatcher::optional_repeat_max(!TextMatcher::linebreak())
				),
				(
					"whitespace",
					TextMatcher::repeat_max(TextMatcher::whitespace())
				)
			])
		}
	}

	/// Parse some text.
	pub fn parse(&self, text:&str) -> Vec<TextMatchResult> {
		self.matcher_set.multi_match_text(text)
	}
}