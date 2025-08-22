use crate::{ TextMatcher, TextMatcherSource };
use std::ops::Index;



pub struct TextMatcherSet {
	matchers:Vec<(String, TextMatcher)>
}
impl TextMatcherSet {

	/* CONSTRUCTOR METHODS */

	/// Create a new matcher set.
	pub fn new() -> TextMatcherSet {
		TextMatcherSet {
			matchers: Vec::new()
		}
	}

	/// Return self with an additional matcher to the set.
	pub fn with_matcher<T:TextMatcherSource + 'static>(mut self, name:&str, matcher_source:T) -> Self {
		self.matchers.push((name.to_string(), TextMatcher::new(matcher_source)));
		self
	}

	/// Return self with multiple additional matchers to the set.
	pub fn with_matchers<T:TextMatcherSource + 'static>(mut self, sources:Vec<(&str, T)>) -> Self {
		for (name, matcher_source) in sources {
			self = self.with_matcher(name, matcher_source)
		}
		self
	}



	/* USAGE METHODS */

	/// Get a matcher by name.
	pub fn matcher_by_name(&self, name:&str) -> Option<&(String, TextMatcher)> {
		self.matchers.iter().find(|(matcher_name, _)| matcher_name == name)
	}

	/// Try to match any of the matchers to the given text. Returns the name of the matcher and length of the match in case of a match.
	pub fn match_text(&self, text:&str) -> Option<(&str, usize)> {
		for (matcher_name, matcher) in &self.matchers {
			if let Some(match_length) = matcher.match_text(text) {
				return Some((matcher_name, match_length));
			}
		}
		None
	}
}
impl Index<&str> for TextMatcherSet {
	type Output = (String, TextMatcher);

	fn index(&self, index:&str) -> &Self::Output {
		self.matcher_by_name(index).unwrap()
	}
}