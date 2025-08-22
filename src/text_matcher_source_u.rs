#[cfg(test)]
mod tests {
	use crate::TextMatcherSource;



	/* ATOM IMPLEMENTATION TESTS */
	#[test]
	fn test_text_matcher_atom_char() {
		assert_eq!('x'.match_text_length("xaba"), Some(1)); // Partial match
		assert_eq!('a'.match_text_length("xaba"), None); // Full mismatch
		assert_eq!('x'.match_text_length("xxaba"), Some(1)); // Non-repeating match
		assert_eq!('x'.match_text_length(""), None); // Empty text mismatch
	}

	#[test]
	fn test_text_matcher_atom_str() {
		assert_eq!("x".match_text_length("xaba"), Some(1)); // Partial match
		assert_eq!("xaba".match_text_length("xaba"), Some(4)); // Full match
		assert_eq!("a".match_text_length("xaba"), None); // Full mismatch
		assert_eq!("xxa".match_text_length("xxaba"), Some(3)); // Partial lengthy match
		assert_eq!("xxa".match_text_length(""), None); // Empty text mismatch
	}

	#[test]
	fn test_text_matcher_atom_string() {
		assert_eq!("x".to_string().match_text_length("xaba"), Some(1)); // Partial match
		assert_eq!("xaba".to_string().match_text_length("xaba"), Some(4)); // Full match
		assert_eq!("a".to_string().match_text_length("xaba"), None); // Full mismatch
		assert_eq!("xxa".to_string().match_text_length("xxaba"), Some(3)); // Partial lengthy match
		assert_eq!("xxa".to_string().match_text_length(""), None); // Empty text mismatch
	}

	#[test]
	fn test_text_matcher_atom_fn() {
		assert_eq!((|text:&str| if text == "xaba" { Some(3) } else { None }).match_text_length("xaba"), Some(3));
		assert_eq!((|text:&str| if text == "daba" { Some(3) } else { None }).match_text_length("xaba"), None);
		assert_eq!((|text:&str| if text == "daba" { Some(3) } else { None }).match_text_length(""), None);
	}



	/* LIST IMPLEMENTATION TESTS */

	#[test]
	fn test_text_matcher_list_array() {
		assert_eq!(["x", "ab"].match_text_length("xaba"), Some(3)); // Partial match
		assert_eq!(["x", "ab", "a"].match_text_length("xaba"), Some(4)); // Full match
		assert_eq!(["a", "ab"].match_text_length("xaba"), None); // Full mismatch
		assert_eq!(["x", "x", "ab"].match_text_length("xxaba"), Some(4)); // Partial lengthy match
		assert_eq!(["x", "ad"].match_text_length("xaba"), None); // Partial mismatch
		assert_eq!(["x", "ab", "a", ""].match_text_length("xaba"), Some(4)); // Full match with trailing empty
		assert_eq!(["x", "ab", "a", ""].match_text_length(""), None); // Empty text mismatch
	}

	#[test]
	fn test_text_matcher_list_vec() {
		assert_eq!(vec!["x", "ab"].match_text_length("xaba"), Some(3)); // Partial match
		assert_eq!(vec!["x", "ab", "a"].match_text_length("xaba"), Some(4)); // Full match
		assert_eq!(vec!["a", "ab"].match_text_length("xaba"), None); // Full mismatch
		assert_eq!(vec!["x", "x", "ab"].match_text_length("xxaba"), Some(4)); // Partial lengthy match
		assert_eq!(vec!["x", "ad"].match_text_length("xaba"), None); // Partial mismatch
		assert_eq!(vec!["x", "ab", "a", ""].match_text_length("xaba"), Some(4)); // Full match with trailing empty
		assert_eq!(vec!["x", "ab", "a", ""].match_text_length(""), None); // Empty text mismatch
	}

	#[test]
	fn test_text_matcher_list_tuple() {
		assert_eq!(('x', "ab").match_text_length("xaba"), Some(3)); // Partial match
		assert_eq!(('x', "ab", 'a').match_text_length("xaba"), Some(4)); // Full match
		assert_eq!(('a', "ab").match_text_length("xaba"), None); // Full mismatch
		assert_eq!(('x', "xab").match_text_length("xxaba"), Some(4)); // Partial lengthy match
		assert_eq!(('x', "ad").match_text_length("xaba"), None); // Partial mismatch
		assert_eq!(('x', "ab", 'a', "").match_text_length("xaba"), Some(4)); // Full match with trailing empty
		assert_eq!(('x', "ab", 'a', 'x', "ab", 'a', 'x', "ab", 'a', 'x', "ab", 'a').match_text_length("xabaxabaxabaxabaxaba"), Some(16)); // Full lengthy match
		assert_eq!(('x', "ab", 'a', "").match_text_length(""), None); // Empty text mismatch
	}
}