use std::ops::{ Add, BitAnd, BitOr, Mul, Not };
use crate::TextMatcherSource;



const LINE_BREAK_CHARS:&[char] = &['\n', '\r'];



pub struct TextMatcher(Box<dyn TextMatcherSource>);
impl TextMatcher {

	/// Create a new text-matcher from a source.
	pub fn new<T:TextMatcherSource + 'static>(source:T) -> TextMatcher {
		TextMatcher(Box::new(source))
	}



	/* REPETITION MATCHER METHODS */

	/// Repeat the given matcher as many times as possible. Will return None when not matched once.
	pub fn repeat_max<T:TextMatcherSource + 'static>(sub_matcher:T) -> TextMatcher {
		TextMatcher::new(move |text:&str| {
			let mut matched_any:bool = false;
			let mut cursor:usize = 0;
			let mut remaining_text:&str = text;
			while let Some(match_length) = sub_matcher.match_text(remaining_text) {
				matched_any = true;
				cursor += match_length;
				remaining_text = if text.len() > cursor { &text[cursor..] } else { "" };
			}
			if matched_any {
				Some(cursor)
			} else {
				None
			}
		})
	}

	/// Create a matcher that tries to match the given sub-matcher, but still returns Some(0) on mismatch.
	pub fn optional<T:TextMatcherSource + 'static>(sub_matcher:T) -> TextMatcher {
		TextMatcher::new(move |text:&str| {
			Some(sub_matcher.match_text(text).unwrap_or(0))
		})
	}



	/* WHITE-SPACE MATCHER METHODS */

	/// Create a matcher that matches only white-space. Matches maximum one character.
	pub fn whitespace() -> TextMatcher {
		TextMatcher::on_first_char(|char| char.is_whitespace())
	}

	/// Create a matcher that matches only linebreaks. Matches maximum one character.
	pub fn linebreak() ->  TextMatcher {
		TextMatcher::on_first_char(|char| LINE_BREAK_CHARS.contains(&char))
	}

	/// Create a matcher that matches only non-linebreak whitespace. Matches maximum one character.
	pub fn inline_whitespace() ->  TextMatcher {
		TextMatcher::on_first_char(|char| char.is_whitespace() && !LINE_BREAK_CHARS.contains(&char))
	}



	/* NUMERIC MATCHER METHODS */

	/// Create a matcher that matches only digits. Matches maximum one character.
	pub fn digit() -> TextMatcher {
		TextMatcher::on_first_char(|char| *char >= '0' && *char <= '9')
	}

	/// Create a matcher that matches unsigned integers. Matches as long as possible.
	pub fn unsigned_integer() -> TextMatcher {
		TextMatcher::repeat_max(TextMatcher::digit())
	}

	/// Create a matcher that matches signed integers. Matches as long as possible.
	pub fn signed_integer() -> TextMatcher {
		TextMatcher::optional("-") + TextMatcher::unsigned_integer()
	}

	/// Create a matcher that matches floating point numbers. Matches as long as possible.
	pub fn float() -> TextMatcher {
		TextMatcher::signed_integer() + TextMatcher::optional(TextMatcher::new(".") + TextMatcher::unsigned_integer())
	}

	



	/* HELPER METHODS */
	
	/// Create a matcher that checks something on the first character.
	fn on_first_char<T:Fn(&char) -> bool + 'static>(compare_function:T) -> TextMatcher {
		TextMatcher::new(move |text:&str| {
			if !text.is_empty() {
				if let Some(first_char) = text[..1].chars().next() {
					if compare_function(&first_char) {
						return Some(1);
					}
				}
			}
			None
		})
	}
}
impl TextMatcherSource for TextMatcher {
	fn match_text(&self, text:&str) -> Option<usize> {
		self.0.match_text(text)
	}
}
impl<T:TextMatcherSource + 'static> Add<T> for TextMatcher {
	type Output = TextMatcher;

	fn add(self, rhs:T) -> Self::Output {
		TextMatcher::new(move |text:&str| {
			if let Some(left_length) = self.match_text(text) {
				let remaining_text:&str = if text.len() > left_length { &text[left_length..] } else { "" };
				if let Some(right_length) = rhs.match_text(remaining_text) {
					return Some(left_length + right_length);
				}
			}
			None
		})
	}
}
impl Mul<usize> for TextMatcher {
	type Output = TextMatcher;

	fn mul(self, rhs:usize) -> Self::Output {
		TextMatcher::new(move |text:&str| {
			let mut cursor:usize = 0;
			for _ in 0..rhs {
				let remaining_text:&str = if text.len() > cursor { &text[cursor..] } else { "" };
				match self.match_text(remaining_text) {
					Some(match_length) => cursor += match_length,
					None => return None
				}
			}
			Some(cursor)
		})
	}
}
impl<T:TextMatcherSource + 'static> BitAnd<T> for TextMatcher {
	type Output = TextMatcher;

	fn bitand(self, rhs:T) -> Self::Output {
		self + rhs
	}
}
impl<T:TextMatcherSource + 'static> BitOr<T> for TextMatcher {
	type Output = TextMatcher;

	fn bitor(self, rhs:T) -> Self::Output {
		TextMatcher::new(move |text:&str| {
			if let Some(match_length) = self.match_text(text) {
				Some(match_length)
			} else if let Some(match_length) = rhs.match_text(text) {
				Some(match_length)
			} else {
				None
			}
		})
	}
}
impl Not for TextMatcher {
	type Output = TextMatcher;

	fn not(self) -> Self::Output {
		TextMatcher::new(move |text:&str| {
			match self.match_text(text) {
				Some(_) => None,
				None => Some(0)
			}
		})
	}
}