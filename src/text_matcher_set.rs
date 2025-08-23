use crate::{ TextMatchResult, TextMatcher, TextMatcherSource };
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

	/// Try to match any of the matchers to the given text. Returns MatchResult in case of a match.
	pub fn match_text(&self, text:&str) -> Option<TextMatchResult> {
		for (matcher_name, matcher) in &self.matchers {
			if let Some(mut match_result) = matcher.match_text(text) {
				match_result.type_name = matcher_name.to_string();
				return Some(match_result);
			}
		}
		None
	}

	/// Keep matching as much of the given text as possible. Returns a list of entries containing matcher name, .
	pub fn multi_match_text(&self, text:&str) -> TextMatchResult {
		let mut cursor:usize = 0;
		let mut remaining_text:&str = text;
		let mut results:Vec<TextMatchResult> = Vec::new();
		while let Some(match_result) = self.match_text(remaining_text) {
			results.push(match_result.clone());
			cursor += match_result.length;
			if match_result.length == 0 {
				panic!("Matched a {} with length 0 in multi_match_text. As this will increase the cursor, this would repeat indefinitely. Aborting rest of parsing.", match_result.type_name);
			}
			remaining_text = if text.len() > cursor { &text[cursor..] } else { "" };
		}
		TextMatchResult::new_with_sub_matches(cursor, text, results)
	}
}
impl Index<&str> for TextMatcherSet {
	type Output = (String, TextMatcher);

	fn index(&self, index:&str) -> &Self::Output {
		self.matcher_by_name(index).unwrap()
	}
}