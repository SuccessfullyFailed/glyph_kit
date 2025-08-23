#[cfg(test)]
mod tests {
	use crate::{ TextMatchResult, TextMatcherSet, TextMatcherSource };



	#[test]
	fn test_matcher_set_find_matcher() {
		let set:TextMatcherSet = TextMatcherSet::new().with_matchers(vec![("a", 'a'), ("b", 'b')]);
		assert_eq!(set["a"].0, "a");
		assert_eq!(set["a"].1.match_text("aba").unwrap().length, 1);
		assert_eq!(set["a"].1.match_text("bab"), None);
		
		assert_eq!(set["b"].0, "b");
		assert_eq!(set["b"].1.match_text("aba"), None);
		assert_eq!(set["b"].1.match_text("bab").unwrap().length, 1);
	}

	#[test]
	fn test_matcher_set_match_global() {
		let set:TextMatcherSet = TextMatcherSet::new().with_matchers(vec![("a", 'a'), ("b", 'b')]);

		assert_eq!(set.match_text("abax").unwrap(), TextMatchResult { type_name: "a".to_string(), length: 1, contents: "a".to_string(), sub_matches: Vec::new() });
		assert_eq!(set.match_text("bax").unwrap(), TextMatchResult { type_name: "b".to_string(), length: 1, contents: "b".to_string(), sub_matches: Vec::new() });
		assert_eq!(set.match_text("ax").unwrap(), TextMatchResult { type_name: "a".to_string(), length: 1, contents: "a".to_string(), sub_matches: Vec::new() });
		assert_eq!(set.match_text("x"), None);
	}

	#[test]
	fn test_matcher_set_multi_match_global() {
		let set:TextMatcherSet = TextMatcherSet::new().with_matchers(vec![("a", "a"), ("b", "b"), ("x", "xa")]);

		assert_eq!(
			set.multi_match_text("abaxa").sub_matches,
			vec![
				TextMatchResult { type_name: "a".to_string(), length: 1, contents: "a".to_string(), sub_matches: Vec::new() },
				TextMatchResult { type_name: "b".to_string(), length: 1, contents: "b".to_string(), sub_matches: Vec::new() },
				TextMatchResult { type_name: "a".to_string(), length: 1, contents: "a".to_string(), sub_matches: Vec::new() },
				TextMatchResult { type_name: "x".to_string(), length: 2, contents: "xa".to_string(), sub_matches: Vec::new() }
			]
		);
	}
}