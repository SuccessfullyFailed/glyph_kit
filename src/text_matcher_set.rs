use crate::{ MatchHit, MatchExpr, TextPredicate };
use std::ops::Index;



pub struct MatcherRegistry {
	matchers:Vec<(String, MatchExpr)>
}
impl MatcherRegistry {

	/* CONSTRUCTOR METHODS */

	/// Create a new registry.
	pub fn new() -> MatcherRegistry {
		MatcherRegistry {
			matchers: Vec::new()
		}
	}

	/// Return self with an additional matcher to the set.
	pub fn with_matcher<T:TextPredicate + 'static>(mut self, name:&str, matcher_source:T) -> Self {
		self.matchers.push((name.to_string(), MatchExpr::new(matcher_source)));
		self
	}

	/// Return self with multiple additional matchers to the set.
	pub fn with_matchers<T:TextPredicate + 'static>(mut self, sources:Vec<(&str, T)>) -> Self {
		for (name, matcher_source) in sources {
			self = self.with_matcher(name, matcher_source)
		}
		self
	}



	/* USAGE METHODS */

	/// Get a match-expression by name.
	pub fn matcher_by_name(&self, name:&str) -> Option<&(String, MatchExpr)> {
		self.matchers.iter().find(|(matcher_name, _)| matcher_name == name)
	}

	/// Try to match any of the match-expressions to the given text. Returns MatchResult in case of a match.
	pub fn match_text(&self, text:&str) -> Option<MatchHit> {
		for (matcher_name, matcher) in &self.matchers {
			if let Some(mut match_result) = matcher.match_text(text) {
				match_result.type_name = matcher_name.to_string();
				return Some(match_result);
			}
		}
		None
	}

	/// Keep matching as much of the given text as possible. Returns a list of MatchResults.
	pub fn multi_match_text(&self, text:&str) -> MatchHit {
		let mut cursor:usize = 0;
		let mut remaining_text:&str = text;
		let mut results:Vec<MatchHit> = Vec::new();
		while let Some(match_result) = self.match_text(remaining_text) {
			results.push(match_result.clone());
			cursor += match_result.length;
			if match_result.length == 0 {
				panic!("Matched a {} with length 0 in multi_match_text. As this will increase the cursor, this would repeat indefinitely. Aborting rest of parsing.", match_result.type_name);
			}
			remaining_text = if text.len() > cursor { &text[cursor..] } else { "" };
		}
		MatchHit::new_with_sub_matches(cursor, text, results)
	}

	/// Find any match anywhere in the given text. Returns the start index where it was found and MatchResult in case of a match.
	pub fn find_match(&self, text:&str) -> Option<(usize, MatchHit)> {
		for cursor in 0..text.len() {
			if let Some(match_result) = self.match_text(&text[cursor..]) {
				return Some((cursor, match_result));
			}
		}
		None
	}

	/// Find all possible matches anywhere in the given text. Returns the start index where it was found and MatchResult in case of a match.
	pub fn find_matches(&self, text:&str) -> Vec<(usize, MatchHit)> {
		let mut results:Vec<(usize, MatchHit)> = Vec::new();
		let text_end:usize = text.len();
		let mut cursor:usize = 0;
		while cursor < text_end {
			if let Some(match_result) = self.match_text(&text[cursor..]) {
				results.push((cursor, match_result.clone()));
				cursor += match_result.length;
			} else {
				cursor += 1;
			}
		}
		results
	}
}
impl Index<&str> for MatcherRegistry {
	type Output = (String, MatchExpr);

	fn index(&self, index:&str) -> &Self::Output {
		self.matcher_by_name(index).unwrap()
	}
}