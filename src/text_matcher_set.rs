use crate::{ TextMatcher, TextMatcherSource };
use std::ops::Index;



#[derive(Clone, PartialEq, Debug)]
pub struct MatchResult {
	pub match_type:String,
	pub match_length:usize,
	pub match_contents:String
}



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

	/// Try to match any of the matchers to the given text. Returns MatchResult in case of a match.
	pub fn match_text(&self, text:&str) -> Option<MatchResult> {
		for (matcher_name, matcher) in &self.matchers {
			if let Some(match_length) = matcher.match_text(text) {
				return Some(
					MatchResult {
						match_type: matcher_name.to_string(),
						match_length,
						match_contents: text[..match_length].to_string()
					}
				);
			}
		}
		None
	}

	/// Keep matching as much of the given text as possible. Returns a list of entries containing matcher name, .
	pub fn multi_match_text(&self, text:&str) -> Vec<MatchResult> {
		let mut cursor:usize = 0;
		let mut remaining_text:&str = text;
		let mut results:Vec<MatchResult> = Vec::new();
		while let Some(match_result) = self.match_text(remaining_text) {
			results.push(match_result.clone());
			cursor += match_result.match_length;
			remaining_text = if text.len() > cursor { &text[cursor..] } else { "" };
		}
		results
	}
}
impl Index<&str> for TextMatcherSet {
	type Output = (String, TextMatcher);

	fn index(&self, index:&str) -> &Self::Output {
		self.matcher_by_name(index).unwrap()
	}
}