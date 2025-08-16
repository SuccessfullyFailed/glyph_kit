#[cfg(test)]
mod tests {
	use crate::{TextMatcher, TextMatcherAdd};



	#[test]
	fn test_text_matcher_operation_add() {
		assert_eq!(TextMatcherAdd::new('x', "ab").match_text("xaba"), Some(3)); // Partial match
		
		assert_eq!(TextMatcherAdd::new('x', "ab").match_text("xaba"), Some(3)); // Partial match
		assert_eq!(TextMatcherAdd::new('x', "aba").match_text("xaba"), Some(4)); // Full match
		assert_eq!(TextMatcherAdd::new('a', "ab").match_text("xaba"), None); // Full mismatch
		assert_eq!(TextMatcherAdd::new('x', "xab").match_text("xxaba"), Some(4)); // Partial lengthy match
		assert_eq!(TextMatcherAdd::new('x', "ad").match_text("xaba"), None); // Partial mismatch
		assert_eq!(TextMatcherAdd::new("xaba", "").match_text("xaba"), Some(4)); // Full match with trailing empty
		assert_eq!(TextMatcherAdd::new(TextMatcherAdd::new('x', "xa"), "b").match_text("xxaba"), Some(4)); // Recursive adding.
	}
}