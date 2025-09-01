use std::{ rc::Rc, ops::{ Add, BitAnd, BitOr, Mul, Not } };
use crate::{ MatchHit, TextPredicate };



const LINE_BREAK_CHARS:&[char] = &['\n', '\r'];



#[derive(Clone)]
pub struct MatchExpr(Rc<dyn TextPredicate>);
impl MatchExpr {

	/// Create a new match-expression from a source.
	pub fn new<T:TextPredicate + 'static>(source:T) -> MatchExpr {
		MatchExpr(Rc::new(source))
	}



	/* NAME MODIFICATION MATCHER METHODS */

	/// Wrap any result of the sub-matcher in the given name.
	pub fn named<T:TextPredicate + 'static>(name:&str, sub_matcher:T) -> MatchExpr {
		let name:String = name.to_string();
		MatchExpr::new(move |text:&str| {
			match sub_matcher.match_text(text) {
				Some(mut result) => {
					result.type_name = name.to_string();
					Some(result)
				},
				None => None
			}
		})
	}



	/* REPETITION MATCHER METHODS */

	/// Repeat the given match-expression as many times as possible. Will return None when not matched once.
	pub fn repeat_max<T:TextPredicate + 'static>(sub_matcher:T) -> MatchExpr {
		MatchExpr::new(move |text:&str| {
			let mut matched_any:bool = false;
			let mut cursor:usize = 0;
			let mut sub_matches:Vec<MatchHit> = Vec::new();
			let mut remaining_text:&str = text;
			while let Some(match_result) = sub_matcher.match_text(remaining_text) {
				matched_any = true;
				cursor += match_result.length;
				sub_matches.push(match_result);
				remaining_text = if text.len() > cursor { &text[cursor..] } else { "" };
			}
			if matched_any {
				Some(MatchHit::new_with_sub_matches(cursor, text, sub_matches))
			} else {
				None
			}
		})
	}

	/// Repeat the given match-expression as many times as possible. Will return Some(0) when not matched once.
	pub fn optional_repeat_max<T:TextPredicate + 'static>(sub_matcher:T) -> MatchExpr {
		MatchExpr::new(move |text:&str| {
			let mut cursor:usize = 0;
			let mut sub_matches:Vec<MatchHit> = Vec::new();
			let mut remaining_text:&str = text;
			while let Some(match_result) = sub_matcher.match_text(remaining_text) {
				cursor += match_result.length;
				sub_matches.push(match_result);
				remaining_text = if text.len() > cursor { &text[cursor..] } else { "" };
			}
			Some(MatchHit::new_with_sub_matches(cursor, text, sub_matches))
		})
	}

	/// Create a match-expression that tries to match the given sub-matcher, but still returns Some(0) on mismatch.
	pub fn optional<T:TextPredicate + 'static>(sub_matcher:T) -> MatchExpr {
		MatchExpr::new(move |text:&str| {
			Some(sub_matcher.match_text(text).unwrap_or(MatchHit::new(0, text)))
		})
	}



	/* WHITE-SPACE MATCH-EXPRESSION METHODS */

	/// Create a match-expression that matches only white-space. Matches maximum one character.
	pub fn whitespace() -> MatchExpr {
		MatchExpr::on_first_char(|char| char.is_whitespace())
	}

	/// Create a match-expression that matches only linebreaks. Matches maximum one character.
	pub fn linebreak() ->  MatchExpr {
		MatchExpr::on_first_char(|char| LINE_BREAK_CHARS.contains(&char))
	}

	/// Create a match-expression that matches only non-linebreak whitespace. Matches maximum one character.
	pub fn inline_whitespace() ->  MatchExpr {
		MatchExpr::on_first_char(|char| char.is_whitespace() && !LINE_BREAK_CHARS.contains(&char))
	}



	/* NUMERIC MATCH-EXPRESSION METHODS */

	/// Create a match-expression that matches only digits. Matches maximum one character.
	pub fn digit() -> MatchExpr {
		MatchExpr::on_first_char(|char| *char >= '0' && *char <= '9')
	}

	/// Create a match-expression that matches unsigned integers. Matches as long as possible.
	pub fn unsigned_integer() -> MatchExpr {
		MatchExpr::repeat_max(MatchExpr::digit())
	}

	/// Create a match-expression that matches signed integers. Matches as long as possible.
	pub fn signed_integer() -> MatchExpr {
		MatchExpr::optional("-") + MatchExpr::unsigned_integer()
	}

	/// Create a match-expression that matches floating point numbers. Matches as long as possible.
	pub fn float() -> MatchExpr {
		MatchExpr::signed_integer() + MatchExpr::optional(MatchExpr::new(".") + MatchExpr::unsigned_integer())
	}

	



	/* HELPER METHODS */
	
	/// Create a match-expression that checks something on the first character.
	fn on_first_char<T:Fn(&char) -> bool + 'static>(compare_function:T) -> MatchExpr {
		MatchExpr::new(move |text:&str| {
			if !text.is_empty() {
				if let Some(first_char) = text[..1].chars().next() {
					if compare_function(&first_char) {
						return Some(MatchHit::new(1, text));
					}
				}
			}
			None
		})
	}
}
impl TextPredicate for MatchExpr {
	fn match_text(&self, text:&str) -> Option<MatchHit> {
		self.0.match_text(text)
	}
}
impl<T:TextPredicate + 'static> Add<T> for MatchExpr {
	type Output = MatchExpr;

	fn add(self, rhs:T) -> Self::Output {
		MatchExpr::new(move |text:&str| {
			if let Some(left_match) = self.match_text(text) {
				let remaining_text:&str = if text.len() > left_match.length { &text[left_match.length..] } else { "" };
				if let Some(right_match) = rhs.match_text(remaining_text) {
					return Some(MatchHit::new_with_sub_matches(left_match.length + right_match.length, text, vec![left_match, right_match]));
				}
			}
			None
		})
	}
}
impl Mul<usize> for MatchExpr {
	type Output = MatchExpr;

	fn mul(self, rhs:usize) -> Self::Output {
		MatchExpr::new(move |text:&str| {
			let mut cursor:usize = 0;
			let mut sub_results:Vec<MatchHit> = Vec::new();
			for _ in 0..rhs {
				let remaining_text:&str = if text.len() > cursor { &text[cursor..] } else { "" };
				match self.match_text(remaining_text) {
					Some(match_result) => {
						cursor += match_result.length;
						sub_results.push(match_result);
					},
					None => return None
				}
			}
			let type_name:String = sub_results.iter().next().map(|result| result.type_name.clone()).unwrap_or(String::new());
			Some(MatchHit::named_with_sub_matches(&type_name, cursor, text, sub_results))
		})
	}
}
impl<T:TextPredicate + 'static> BitAnd<T> for MatchExpr {
	type Output = MatchExpr;

	fn bitand(self, rhs:T) -> Self::Output {
		self + rhs
	}
}
impl<T:TextPredicate + 'static> BitOr<T> for MatchExpr {
	type Output = MatchExpr;

	fn bitor(self, rhs:T) -> Self::Output {
		MatchExpr::new(move |text:&str| {
			if let Some(match_result) = self.match_text(text) {
				Some(match_result)
			} else if let Some(match_result) = rhs.match_text(text) {
				Some(match_result)
			} else {
				None
			}
		})
	}
}
impl Not for MatchExpr {
	type Output = MatchExpr;

	fn not(self) -> Self::Output {
		MatchExpr::new(move |text:&str| {
			if text.is_empty() {
				None
			} else { 
				match self.match_text(text) {
					Some(_) => None,
					None => Some(MatchHit::new(1, text))
				}
			}
		})
	}
}