use crate::TextMatcher;

pub struct TextMatcherAdd<T, U> where T:TextMatcher, U:TextMatcher {
	left:T,
	right:U
}
impl<T, U> TextMatcherAdd<T, U> where T:TextMatcher, U:TextMatcher {

	/// Create a new TextMatcherAdd.
	pub fn new(left:T, right:U) -> TextMatcherAdd<T, U> {
		TextMatcherAdd { left, right }
	}
}
impl<T, U> TextMatcher for TextMatcherAdd<T, U> where T:TextMatcher, U:TextMatcher {
	fn match_text(&self, text:&str) -> Option<usize> {
		if let Some(left_length) = self.left.match_text(text) {
			let text_rest:&str = if text.len() > left_length { &text[left_length..] } else { "" };
			if let Some(right_length) = self.right.match_text(text_rest) {
				return Some(left_length + right_length);
			}
		}
		None
	}
}