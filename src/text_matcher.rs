


/* MAIN TRAIT */
pub trait TextMatcher {

	/// Try to match the given text. Returns the length of the match in case of a match.
	fn match_text(&self, text:&str) -> Option<usize>;
}



/* ATOM IMPLEMENTATIONS */
impl TextMatcher for char {
	fn match_text(&self, text:&str) -> Option<usize> {
		if let Some(first_char) = text.chars().next() {
			if first_char == *self {
				return Some(1);
			}
		}
		None
	}
}
impl TextMatcher for str {
	fn match_text(&self, text:&str) -> Option<usize> {
		if text.starts_with(self) {
			Some(self.len())
		} else {
			None
		}
	}
}
impl<T> TextMatcher for T where T:Fn(&str) -> Option<usize> {
	fn match_text(&self, text:&str) -> Option<usize> {
		self(text)
	}
}


/* LIST IMPLEMENTATIONS */
impl<T> TextMatcher for [T] where T:TextMatcher {
	fn match_text(&self, text:&str) -> Option<usize> {
		let mut cursor:usize = 0;
		let mut matcher_index:usize = 0;
		let text_len:usize = text.len();
		let matcher_count:usize = self.len();
		while cursor < text_len && matcher_index < matcher_count {
			if let Some(match_length) = self[matcher_index].match_text(&text[cursor..]) {
				cursor += match_length;
			} else {
				return None;
			};
			matcher_index += 1;
		}
		Some(cursor)
	}
}
impl<T> TextMatcher for Vec<T> where T:TextMatcher {
	fn match_text(&self, text:&str) -> Option<usize> {
		self[..].match_text(text)
	}
}