#[cfg(test)]
mod tests {
	use crate::{ MatchHit, TextPredicate };



	/* ATOM IMPLEMENTATION TESTS */
	#[test]
	fn test_text_predicate_atom_char() {
		assert_eq!('x'.match_text("xaba").unwrap().length, 1); // Partial match
		assert_eq!('a'.match_text("xaba"), None); // Full mismatch
		assert_eq!('x'.match_text("xxaba").unwrap().length, 1); // Non-repeating match
		assert_eq!('x'.match_text(""), None); // Empty text mismatch
	}

	#[test]
	fn test_text_predicate_atom_str() {
		assert_eq!("x".match_text("xaba").unwrap().length, 1); // Partial match
		assert_eq!("xaba".match_text("xaba").unwrap().length, 4); // Full match
		assert_eq!("a".match_text("xaba"), None); // Full mismatch
		assert_eq!("xxa".match_text("xxaba").unwrap().length, 3); // Partial lengthy match
		assert_eq!("xxa".match_text(""), None); // Empty text mismatch
	}

	#[test]
	fn test_text_predicate_atom_string() {
		assert_eq!("x".to_string().match_text("xaba").unwrap().length, 1); // Partial match
		assert_eq!("xaba".to_string().match_text("xaba").unwrap().length, 4); // Full match
		assert_eq!("a".to_string().match_text("xaba"), None); // Full mismatch
		assert_eq!("xxa".to_string().match_text("xxaba").unwrap().length, 3); // Partial lengthy match
		assert_eq!("xxa".to_string().match_text(""), None); // Empty text mismatch
	}

	#[test]
	fn test_text_predicate_atom_fn() {
		assert_eq!((|text:&str| if text == "xaba" { Some(MatchHit::new(3, text)) } else { None }).match_text("xaba").unwrap().length, 3);
		assert_eq!((|text:&str| if text == "daba" { Some(MatchHit::new(3, text)) } else { None }).match_text("xaba"), None);
		assert_eq!((|text:&str| if text == "daba" { Some(MatchHit::new(3, text)) } else { None }).match_text(""), None);
	}



	/* LIST IMPLEMENTATION TESTS */

	#[test]
	fn test_text_predicate_list_array() {
		assert_eq!(["x", "ab"].match_text("xaba").unwrap().length, 3); // Partial match
		assert_eq!(["x", "ab", "a"].match_text("xaba").unwrap().length, 4); // Full match
		assert_eq!(["a", "ab"].match_text("xaba"), None); // Full mismatch
		assert_eq!(["x", "x", "ab"].match_text("xxaba").unwrap().length, 4); // Partial lengthy match
		assert_eq!(["x", "ad"].match_text("xaba"), None); // Partial mismatch
		assert_eq!(["x", "ab", "a", ""].match_text("xaba").unwrap().length, 4); // Full match with trailing empty
		assert_eq!(["x", "ab", "a", ""].match_text(""), None); // Empty text mismatch
	}

	#[test]
	fn test_text_predicate_list_vec() {
		assert_eq!(vec!["x", "ab"].match_text("xaba").unwrap().length, 3); // Partial match
		assert_eq!(vec!["x", "ab", "a"].match_text("xaba").unwrap().length, 4); // Full match
		assert_eq!(vec!["a", "ab"].match_text("xaba"), None); // Full mismatch
		assert_eq!(vec!["x", "x", "ab"].match_text("xxaba").unwrap().length, 4); // Partial lengthy match
		assert_eq!(vec!["x", "ad"].match_text("xaba"), None); // Partial mismatch
		assert_eq!(vec!["x", "ab", "a", ""].match_text("xaba").unwrap().length, 4); // Full match with trailing empty
		assert_eq!(vec!["x", "ab", "a", ""].match_text(""), None); // Empty text mismatch
	}

	#[test]
	fn test_text_predicate_list_tuple() {
		assert_eq!(('x', "ab").match_text("xaba").unwrap().length, 3); // Partial match
		assert_eq!(('x', "ab", 'a').match_text("xaba").unwrap().length, 4); // Full match
		assert_eq!(('a', "ab").match_text("xaba"), None); // Full mismatch
		assert_eq!(('x', "xab").match_text("xxaba").unwrap().length, 4); // Partial lengthy match
		assert_eq!(('x', "ad").match_text("xaba"), None); // Partial mismatch
		assert_eq!(('x', "ab", 'a', "").match_text("xaba").unwrap().length, 4); // Full match with trailing empty
		assert_eq!(('x', "ab", 'a', 'x', "ab", 'a', 'x', "ab", 'a', 'x', "ab", 'a').match_text("xabaxabaxabaxabaxaba").unwrap().length, 16); // Full lengthy match
		assert_eq!(('x', "ab", 'a', "").match_text(""), None); // Empty text mismatch
	}



	/* MISCELLANEOUS IMPLEMENTATION TESTS */

	#[test]
	fn test_text_predicate_misc_range() {
		assert_eq!(("x".."b").match_text("xoba").unwrap().length, 3); // Partial lengthy match
		assert_eq!(("x".."a").match_text("xoba").unwrap().length, 4); // Full match
		assert_eq!(("b".."a").match_text("xoba"), None); // Full mismatch
		assert_eq!(("x".."b").match_text(""), None); // Empty text mismatch
	}
}