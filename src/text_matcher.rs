


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
impl TextMatcher for &str {
	fn match_text(&self, text:&str) -> Option<usize> {
		if text.starts_with(self) {
			Some(self.len())
		} else {
			None
		}
	}
}
impl TextMatcher for String {
	fn match_text(&self, text:&str) -> Option<usize> {
		self.as_str().match_text(text)
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
		let text_len:usize = text.len();
		for matcher in self {
			let text_remainder:&str = if text_len > cursor { &text[cursor..] } else { "" }; // Next matcher could match empty.
			if let Some(match_length) = matcher.match_text(&text_remainder) {
				cursor += match_length;
			} else {
				return None;
			};
		}
		Some(cursor)
	}
}
impl<T> TextMatcher for Vec<T> where T:TextMatcher {
	fn match_text(&self, text:&str) -> Option<usize> {
		self[..].match_text(text)
	}
}



/* TUPLE IMPLEMENTATION */
macro_rules! tuple_matcher {
	($($name:ident $idx:tt), +) => {
		impl<$($name: TextMatcher),+> TextMatcher for ($($name,)+) {
			fn match_text(&self, text:&str) -> Option<usize> {
				let mut cursor = 0;
				let text_len:usize = text.len();
				$(
					let text_remainder:&str = if text_len > cursor { &text[cursor..] } else { "" }; // Next matcher could match empty.
					if let Some(len) = self.$idx.match_text(&text_remainder) {
						cursor += len;
					} else {
						return None;
					}
				)+
				Some(cursor)
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