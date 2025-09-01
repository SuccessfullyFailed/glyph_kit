#[cfg(test)]
mod tests {
	use crate::{ MatchHit, MatchExpr, TextPredicate };


	#[test]
	fn test_matcher_creation() {

		// Matchers should be able to be created from any source and tested without panicking.
		assert_eq!(MatchExpr::new('x').match_text("xaba").unwrap().length, 1);
		assert_eq!(MatchExpr::new("xaba").match_text("xaba").unwrap().length, 4);
		assert_eq!(MatchExpr::new("xaba".to_string()).match_text("xaba").unwrap().length, 4);
		assert_eq!(MatchExpr::new(|text:&str| if text == "xaba" { Some(MatchHit::new(3, text)) } else { None }).match_text("xaba").unwrap().length, 3);
		assert_eq!(MatchExpr::new(vec!["x", "aba", ""]).match_text("xaba").unwrap().length, 4);
		assert_eq!(MatchExpr::new(('x', "aba", "")).match_text("xaba").unwrap().length, 4);
		assert_eq!(MatchExpr::new('0').match_text("xaba"), None);
	}



	/* OPERAND TESTS */

	#[test]
	fn test_matcher_add() {
		let matcher:MatchExpr = MatchExpr::new("xa") + 'b';
		assert_eq!(matcher.match_text("xaba").unwrap().length, 3);

		let matcher:MatchExpr = MatchExpr::new("xa") + 'b' + 'a';
		assert_eq!(matcher.match_text("xaba").unwrap().length, 4);

		let matcher:MatchExpr = MatchExpr::new("xa") + 'b' + 'a' + 's';
		assert_eq!(matcher.match_text("xaba"), None);
	}

	#[test]
	fn test_matcher_mul() {
		let matcher:MatchExpr = MatchExpr::new("xa");
		assert_eq!(matcher.match_text("xaba").unwrap().length, 2);

		let matcher:MatchExpr = MatchExpr::new("xa") * 2;
		assert_eq!(matcher.match_text("xaxaba").unwrap().length, 4);

		let matcher:MatchExpr = MatchExpr::new("xa") * 3;
		assert_eq!(matcher.match_text("xaxaba"), None);
	}

	#[test]
	fn test_matcher_and() {
		let matcher:MatchExpr = MatchExpr::new("xa") & 'b';
		assert_eq!(matcher.match_text("xaba").unwrap().length, 3);
		let matcher:MatchExpr = matcher & 'a';
		assert_eq!(matcher.match_text("xaba").unwrap().length, 4);
		let matcher:MatchExpr = matcher & 'a';
		assert_eq!(matcher.match_text("xaba"), None);
	}

	#[test]
	fn test_matcher_or() {
		let matcher:MatchExpr = MatchExpr::new("xa") | 'b';
		assert_eq!(matcher.match_text("xaba").unwrap().length, 2);
		assert_eq!(matcher.match_text("baba").unwrap().length, 1);
		assert_eq!(matcher.match_text(""), None);

		let matcher:MatchExpr = MatchExpr::new("xa") | "xaba";
		assert_eq!(matcher.match_text("xaba").unwrap().length, 2);
		assert_eq!(matcher.match_text(""), None);

		let matcher:MatchExpr = MatchExpr::new("xa") | "daba";
		assert_eq!(matcher.match_text("daba").unwrap().length, 4);
		assert_eq!(matcher.match_text(""), None);

		let matcher:MatchExpr = MatchExpr::new("xa") | "ba";
		assert_eq!(matcher.match_text("haba"), None);
		assert_eq!(matcher.match_text(""), None);
	}

	#[test]
	fn test_matcher_not() {
		let matcher:MatchExpr = !MatchExpr::new("xa");
		assert_eq!(matcher.match_text("xaba"), None);
		assert_eq!(matcher.match_text("ababa").unwrap().length, 1);
		assert_eq!(matcher.match_text(""), None);

		let matcher:MatchExpr = !MatchExpr::new("xa") + 'b';
		assert_eq!(matcher.match_text("xaba"), None);
		assert_eq!(matcher.match_text("ababa").unwrap().length, 2);
		assert_eq!(matcher.match_text(""), None);

		let matcher:MatchExpr = !MatchExpr::new("xa") | "xaba";
		assert_eq!(matcher.match_text("xaba").unwrap().length, 4);
		assert_eq!(matcher.match_text(""), None);
	}



	/* REPEATING MATCHER TESTS */

	#[test]
	fn test_matcher_repeat_max() {
		let matcher:MatchExpr = MatchExpr::repeat_max("xa");
		assert_eq!(matcher.match_text("xaxaxaxaba").unwrap().length, 8);
		assert_eq!(matcher.match_text("baba"), None);
		assert_eq!(matcher.match_text(""), None);
	}

	#[test]
	fn test_matcher_optional_repeat_max() {
		let matcher:MatchExpr = MatchExpr::optional_repeat_max("xa");
		assert_eq!(matcher.match_text("xaxaxaxaba").unwrap().length, 8);
		assert_eq!(matcher.match_text("baba").unwrap().length, 0);
		assert_eq!(matcher.match_text("").unwrap().length, 0);
	}

	#[test]
	fn test_matcher_optional() {
		let matcher:MatchExpr = MatchExpr::optional("xa");
		assert_eq!(matcher.match_text("xaxaxaxaba").unwrap().length, 2);
		assert_eq!(matcher.match_text("baba").unwrap().length, 0);
		assert_eq!(matcher.match_text("").unwrap().length, 0);
	}



	/* WHITESPACE MATCHER TESTS */

	#[test]
	fn test_matcher_whitespace() {
		assert_eq!(MatchExpr::whitespace().match_text(" \nxaba").unwrap().length, 1);
		assert_eq!(MatchExpr::whitespace().match_text("\nxaba").unwrap().length, 1);
		assert_eq!(MatchExpr::whitespace().match_text("xaba"), None);
		assert_eq!(MatchExpr::whitespace().match_text(""), None);
	}

	#[test]
	fn test_matcher_linebreak() {
		assert_eq!(MatchExpr::linebreak().match_text(" \nxaba"), None);
		assert_eq!(MatchExpr::linebreak().match_text("\nxaba").unwrap().length, 1);
		assert_eq!(MatchExpr::linebreak().match_text("xaba"), None);
		assert_eq!(MatchExpr::linebreak().match_text(""), None);
	}



	/* NUMERIC MATCHER TEST */
	
	#[test]
	fn test_matcher_inline_whitespace() {
		assert_eq!(MatchExpr::inline_whitespace().match_text(" \nxaba").unwrap().length, 1);
		assert_eq!(MatchExpr::inline_whitespace().match_text("\nxaba"), None);
		assert_eq!(MatchExpr::inline_whitespace().match_text("xaba"), None);
		assert_eq!(MatchExpr::inline_whitespace().match_text(""), None);
	}

	#[test]
	fn test_matcher_digit() {
		assert_eq!(MatchExpr::digit().match_text("-19.0 xaba"), None);
		assert_eq!(MatchExpr::digit().match_text("19.0 xaba").unwrap().length, 1);
		assert_eq!(MatchExpr::digit().match_text("9.0 xaba").unwrap().length, 1);
		assert_eq!(MatchExpr::digit().match_text(".0 xaba"), None);
		assert_eq!(MatchExpr::digit().match_text("0 xaba").unwrap().length, 1);
		assert_eq!(MatchExpr::digit().match_text(" xaba"), None);
		assert_eq!(MatchExpr::digit().match_text(""), None);
	}

	#[test]
	fn test_matcher_unsigned_integer() {
		assert_eq!(MatchExpr::unsigned_integer().match_text("-19.0 xaba"), None);
		assert_eq!(MatchExpr::unsigned_integer().match_text("19.0 xaba").unwrap().length, 2);
		assert_eq!(MatchExpr::unsigned_integer().match_text("9.0 xaba").unwrap().length, 1);
		assert_eq!(MatchExpr::unsigned_integer().match_text(".0 xaba"), None);
		assert_eq!(MatchExpr::unsigned_integer().match_text("0 xaba").unwrap().length, 1);
		assert_eq!(MatchExpr::unsigned_integer().match_text(" xaba"), None);
		assert_eq!(MatchExpr::unsigned_integer().match_text(""), None);
	}

	#[test]
	fn test_matcher_signed_integer() {
		assert_eq!(MatchExpr::signed_integer().match_text("-19.0 xaba").unwrap().length, 3);
		assert_eq!(MatchExpr::signed_integer().match_text("19.0 xaba").unwrap().length, 2);
		assert_eq!(MatchExpr::signed_integer().match_text("9.0 xaba").unwrap().length, 1);
		assert_eq!(MatchExpr::signed_integer().match_text(".0 xaba"), None);
		assert_eq!(MatchExpr::signed_integer().match_text("0 xaba").unwrap().length, 1);
		assert_eq!(MatchExpr::signed_integer().match_text(" xaba"), None);
		assert_eq!(MatchExpr::signed_integer().match_text(""), None);
	}

	#[test]
	fn test_matcher_signed_float() {
		assert_eq!(MatchExpr::float().match_text("-19.0 xaba").unwrap().length, 5);
		assert_eq!(MatchExpr::float().match_text("19.0 xaba").unwrap().length, 4);
		assert_eq!(MatchExpr::float().match_text("9.0 xaba").unwrap().length, 3);
		assert_eq!(MatchExpr::float().match_text(".0 xaba"), None);
		assert_eq!(MatchExpr::float().match_text("0 xaba").unwrap().length, 1);
		assert_eq!(MatchExpr::float().match_text(" xaba"), None);
		assert_eq!(MatchExpr::float().match_text(""), None);
	}



	/* WORD-LIKE MATCH-EXPRESSION MATCHER TEST */
	
	#[test]
	fn test_matcher_alphabetic() {
		assert_eq!(MatchExpr::alphabetic().match_text(" \nzAbA"), None);
		assert_eq!(MatchExpr::alphabetic().match_text("\nzAbA"), None);
		assert_eq!(MatchExpr::alphabetic().match_text("zAbA").unwrap().length, 1);
		assert_eq!(MatchExpr::alphabetic().match_text("AbA").unwrap().length, 1);
		assert_eq!(MatchExpr::alphabetic().match_text(""), None);
	}
	
	#[test]
	fn test_matcher_lowercase_alphabetic() {
		assert_eq!(MatchExpr::lowercase_alphabetic().match_text(" \nzAbA"), None);
		assert_eq!(MatchExpr::lowercase_alphabetic().match_text("\nzAbA"), None);
		assert_eq!(MatchExpr::lowercase_alphabetic().match_text("zAbA").unwrap().length, 1);
		assert_eq!(MatchExpr::lowercase_alphabetic().match_text("AbA"), None);
		assert_eq!(MatchExpr::lowercase_alphabetic().match_text(""), None);
	}
	
	#[test]
	fn test_matcher_uppercase_alphabetic() {
		assert_eq!(MatchExpr::uppercase_alphabetic().match_text(" \nzAbA"), None);
		assert_eq!(MatchExpr::uppercase_alphabetic().match_text("\nzAbA"), None);
		assert_eq!(MatchExpr::uppercase_alphabetic().match_text("zAbA"), None);
		assert_eq!(MatchExpr::uppercase_alphabetic().match_text("AbA").unwrap().length, 1);
		assert_eq!(MatchExpr::uppercase_alphabetic().match_text(""), None);
	}
	
	#[test]
	fn test_matcher_word() {
		assert_eq!(MatchExpr::word().match_text(" \nzAbA"), None);
		assert_eq!(MatchExpr::word().match_text("\nzAbA"), None);
		assert_eq!(MatchExpr::word().match_text("zAbA").unwrap().length, 4);
		assert_eq!(MatchExpr::word().match_text("AbA").unwrap().length, 3);
		assert_eq!(MatchExpr::word().match_text(""), None);
	}
}