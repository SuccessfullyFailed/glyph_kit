use crate::TextMatchResult;



pub trait TextMatcherSource {

	/// Try to match the given text. Returns a TextMatchResult in case of a match.
	fn match_text(&self, text:&str) -> Option<TextMatchResult>;
}



/* ATOM IMPLEMENTATIONS */
impl TextMatcherSource for char {
	fn match_text(&self, text:&str) -> Option<TextMatchResult> {
		if let Some(first_char) = text.chars().next() {
			if first_char == *self {
				return Some(TextMatchResult::new(1, text));
			}
		}
		None
	}
}
impl TextMatcherSource for &str {
	fn match_text(&self, text:&str) -> Option<TextMatchResult> {
		if text.starts_with(self) {
			Some(TextMatchResult::new(self.len(), text))
		} else {
			None
		}
	}
}
impl TextMatcherSource for String {
	fn match_text(&self, text:&str) -> Option<TextMatchResult> {
		self.as_str().match_text(text)
	}
}
impl<T> TextMatcherSource for T where T:Fn(&str) -> Option<TextMatchResult> {
	fn match_text(&self, text:&str) -> Option<TextMatchResult> {
		self(text)
	}
}


/* LIST IMPLEMENTATIONS */
impl<T> TextMatcherSource for [T] where T:TextMatcherSource {
	fn match_text(&self, text:&str) -> Option<TextMatchResult> {
		let mut cursor:usize = 0;
		let mut sub_matches:Vec<TextMatchResult> = Vec::new();
		let text_len:usize = text.len();
		for matcher in self {
			let text_remainder:&str = if text_len > cursor { &text[cursor..] } else { "" }; // Next matcher could match empty.
			if let Some(match_result) = matcher.match_text(&text_remainder) {
				cursor += match_result.length;
				sub_matches.push(match_result);
			} else {
				return None;
			};
		}
		Some(TextMatchResult::new_with_sub_matches(cursor, text, sub_matches))
	}
}
impl<T> TextMatcherSource for Vec<T> where T:TextMatcherSource {
	fn match_text(&self, text:&str) -> Option<TextMatchResult> {
		self[..].match_text(text)
	}
}



/* TUPLE IMPLEMENTATION */
macro_rules! tuple_matcher {
	($($name:ident $idx:tt), +) => {
		impl<$($name:TextMatcherSource),+> TextMatcherSource for ($($name,)+) {
			fn match_text(&self, text:&str) -> Option<TextMatchResult> {
				let mut cursor = 0;
				let mut sub_matches:Vec<TextMatchResult> = Vec::new();
				let text_len:usize = text.len();
				$(
					let text_remainder:&str = if text_len > cursor { &text[cursor..] } else { "" }; // Next matcher could match empty.
					if let Some(match_result) = self.$idx.match_text(&text_remainder) {
						cursor += match_result.length;
						sub_matches.push(match_result);
					} else {
						return None;
					}
				)+
				Some(TextMatchResult::new(cursor, text))
			}
		}
	};
}
tuple_matcher!(A 0, B 1);
tuple_matcher!(A 0, B 1, C 2);
tuple_matcher!(A 0, B 1, C 2, D 3);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24, Z 25);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24, Z 25, AA 26);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24, Z 25, AA 26, AB 27);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24, Z 25, AA 26, AB 27, AC 28);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24, Z 25, AA 26, AB 27, AC 28, AD 29);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24, Z 25, AA 26, AB 27, AC 28, AD 29, AE 30);
tuple_matcher!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24, Z 25, AA 26, AB 27, AC 28, AD 29, AE 30, AF 31);