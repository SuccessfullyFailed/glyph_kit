#[cfg(test)]
mod tests {
	use crate::{ TextMatchResult, TextMatcherSet, TextMatcherSource };



	#[test]
	fn test_matcher_set_find_matcher() {
		let set:TextMatcherSet = TextMatcherSet::new().with_matchers(vec![("a", 'a'), ("b", 'b')]);
		assert_eq!(set["a"].0, "a");
		assert_eq!(set["a"].1.match_text_length("aba"), Some(1));
		assert_eq!(set["a"].1.match_text_length("bab"), None);
		
		assert_eq!(set["b"].0, "b");
		assert_eq!(set["b"].1.match_text_length("aba"), None);
		assert_eq!(set["b"].1.match_text_length("bab"), Some(1));
	}

	#[test]
	fn test_matcher_set_match_global() {
		let set:TextMatcherSet = TextMatcherSet::new().with_matchers(vec![("a", 'a'), ("b", 'b')]);

		assert_eq!(set.match_text("abax"), Some(TextMatchResult { match_type: "a".to_string(), match_length: 1, match_contents: "a".to_string() }));
		assert_eq!(set.match_text("bax"), Some(TextMatchResult { match_type: "b".to_string(), match_length: 1, match_contents: "b".to_string() }));
		assert_eq!(set.match_text("ax"), Some(TextMatchResult { match_type: "a".to_string(), match_length: 1, match_contents: "a".to_string() }));
		assert_eq!(set.match_text("x"), None);
	}

	#[test]
	fn test_matcher_set_multi_match_global() {
		let set:TextMatcherSet = TextMatcherSet::new().with_matchers(vec![("a", "a"), ("b", "b"), ("x", "xa")]);

		assert_eq!(
			set.multi_match_text("abaxa"),
			vec![
				TextMatchResult { match_type: "a".to_string(), match_length: 1, match_contents: "a".to_string() },
				TextMatchResult { match_type: "b".to_string(), match_length: 1, match_contents: "b".to_string() },
				TextMatchResult { match_type: "a".to_string(), match_length: 1, match_contents: "a".to_string() },
				TextMatchResult { match_type: "x".to_string(), match_length: 2, match_contents: "xa".to_string() }
			]
		);
	}
}