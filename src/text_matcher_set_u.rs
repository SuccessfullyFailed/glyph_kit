#[cfg(test)]
mod tests {
	use crate::{ TextMatcherSet, TextMatcherSource };



	#[test]
	fn test_matcher_set_find_matcher() {
		let set:TextMatcherSet = TextMatcherSet::new().with_matchers(vec![("a", 'a'), ("b", 'b')]);
		assert_eq!(set["a"].0, "a");
		assert_eq!(set["a"].1.match_text("aba"), Some(1));
		assert_eq!(set["a"].1.match_text("bab"), None);
		
		assert_eq!(set["b"].0, "b");
		assert_eq!(set["b"].1.match_text("aba"), None);
		assert_eq!(set["b"].1.match_text("bab"), Some(1));
	}

	#[test]
	fn test_matcher_set_match_global() {
		let set:TextMatcherSet = TextMatcherSet::new().with_matchers(vec![("a", 'a'), ("b", 'b')]);

		assert_eq!(set.match_text("abax"), Some(("a", 1)));
		assert_eq!(set.match_text("bax"), Some(("b", 1)));
		assert_eq!(set.match_text("ax"), Some(("a", 1)));
		assert_eq!(set.match_text("x"), None);
	}
}