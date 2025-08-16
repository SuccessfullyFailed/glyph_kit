#[cfg(test)]
mod tests {
	use crate::TextMatcher;



	/* ATOM IMPLEMENTATION TESTS */
	#[test]
	fn test_text_matcher_atom_char() {
		assert_eq!('x'.match_text("xaba"), Some(1));
		assert_eq!('a'.match_text("xaba"), None);
		assert_eq!('x'.match_text("xxaba"), Some(1));
	}

	#[test]
	fn test_text_matcher_atom_str() {
		assert_eq!("x".match_text("xaba"), Some(1));
		assert_eq!("a".match_text("xaba"), None);
		assert_eq!("xxa".match_text("xxaba"), Some(3));
	}

	#[test]
	fn test_text_matcher_atom_fn() {
		assert_eq!((|text:&str| if text == "xaba" { Some(3) } else { None }).match_text("xaba"), Some(3));
		assert_eq!((|text:&str| if text == "daba" { Some(3) } else { None }).match_text("xaba"), None);
	}



	/* LIST IMPLEMENTATION TESTS */

	#[test]
	fn test_text_matcher_list_array() {
		assert_eq!(['x', 'a', 'b'].match_text("xaba"), Some(3));
		assert_eq!(['a', 'a', 'b'].match_text("xaba"), None);
		assert_eq!(['x', 'x', 'a', 'b'].match_text("xxaba"), Some(4));
		assert_eq!(['x', 'a', 'd'].match_text("xaba"), None);
	}

	#[test]
	fn test_text_matcher_list_vec() {
		assert_eq!(vec!['x', 'a', 'b'].match_text("xaba"), Some(3));
		assert_eq!(vec!['a', 'a', 'b'].match_text("xaba"), None);
		assert_eq!(vec!['x', 'x', 'a', 'b'].match_text("xxaba"), Some(4));
		assert_eq!(vec!['x', 'a', 'd'].match_text("xaba"), None);
	}
}