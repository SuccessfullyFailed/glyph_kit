#[cfg(test)]
mod tests {
	use crate::{ TextMatcher, TextMatcherSource };


	#[test]
	fn test_matcher_creation() {

		// Matchers should be able to be created from any source and tested without panicking.
		assert_eq!(TextMatcher::new('x').match_text("xaba"), Some(1));
		assert_eq!(TextMatcher::new("xaba").match_text("xaba"), Some(4));
		assert_eq!(TextMatcher::new("xaba".to_string()).match_text("xaba"), Some(4));
		assert_eq!(TextMatcher::new(|text:&str| if text == "xaba" { Some(3) } else { None }).match_text("xaba"), Some(3));
		assert_eq!(TextMatcher::new(vec!["x", "aba", ""]).match_text("xaba"), Some(4));
		assert_eq!(TextMatcher::new(('x', "aba", "")).match_text("xaba"), Some(4));
		assert_eq!(TextMatcher::new('0').match_text("xaba"), None);
	}

	#[test]
	fn test_matcher_add() {
		let matcher:TextMatcher = TextMatcher::new("xa") + 'b';
		assert_eq!(matcher.match_text("xaba"), Some(3));

		let matcher:TextMatcher = TextMatcher::new("xa") + 'b' + 'a';
		assert_eq!(matcher.match_text("xaba"), Some(4));

		let matcher:TextMatcher = TextMatcher::new("xa") + 'b' + 'a' + 's';
		assert_eq!(matcher.match_text("xaba"), None);
	}

	#[test]
	fn test_matcher_mul() {
		let matcher:TextMatcher = TextMatcher::new("xa");
		assert_eq!(matcher.match_text("xaba"), Some(2));

		let matcher:TextMatcher = TextMatcher::new("xa") * 2;
		assert_eq!(matcher.match_text("xaxaba"), Some(4));

		let matcher:TextMatcher = TextMatcher::new("xa") * 3;
		assert_eq!(matcher.match_text("xaxaba"), None);
	}

	#[test]
	fn test_matcher_and() {
		let matcher:TextMatcher = TextMatcher::new("xa") & 'b';
		assert_eq!(matcher.match_text("xaba"), Some(3));
		let matcher:TextMatcher = matcher & 'a';
		assert_eq!(matcher.match_text("xaba"), Some(4));
		let matcher:TextMatcher = matcher & 'a';
		assert_eq!(matcher.match_text("xaba"), None);
	}

	#[test]
	fn test_matcher_or() {
		let matcher:TextMatcher = TextMatcher::new("xa") | 'b';
		assert_eq!(matcher.match_text("xaba"), Some(2));
		assert_eq!(matcher.match_text("baba"), Some(1));

		let matcher:TextMatcher = TextMatcher::new("xa") | "xaba";
		assert_eq!(matcher.match_text("xaba"), Some(2));

		let matcher:TextMatcher = TextMatcher::new("xa") | "daba";
		assert_eq!(matcher.match_text("daba"), Some(4));

		let matcher:TextMatcher = TextMatcher::new("xa") | "ba";
		assert_eq!(matcher.match_text("haba"), None);
	}

	#[test]
	fn test_matcher_not() {
		let matcher:TextMatcher = !TextMatcher::new("xa") + 'b';
		assert_eq!(matcher.match_text("xaba"), None);
		assert_eq!(matcher.match_text("baba"), Some(1));

		let matcher:TextMatcher = !TextMatcher::new("xa") | "xaba";
		assert_eq!(matcher.match_text("xaba"), Some(4));
	}
}