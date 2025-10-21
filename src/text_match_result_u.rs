#[cfg(test)]
mod tests {
	use crate::MatchHit;



	/* CONSTRUCTOR TESTS */

	#[test]
	fn test_new() {
		assert_eq!(
			MatchHit::new(3, "hello"),
			MatchHit {
				type_name: String::new(),
				length: 3,
				contents: "hel".to_string(),
				sub_matches: Vec::new()
			}
		);
	}

	#[test]
	fn test_new_with_sub_matches_merging() {
		let combined:MatchHit = MatchHit::new_with_sub_matches(5, "abcdef", vec![
			MatchHit::new(2, "abcdef"),
			MatchHit::new(3, "abcdef")
		]);
		assert_eq!(combined.length, 5);
		assert_eq!(combined.contents, "ababc");
		assert_eq!(combined.sub_matches.len(), 0); // Wrapper should be removed.
	}

	#[test]
	fn test_new_with_sub_matches_single_unwrapped() {
		let sub:MatchHit = MatchHit::named("only", 3, "abcdef");
		let combined:MatchHit = MatchHit::new_with_sub_matches(3, "abcdef", vec![sub.clone()]);
		assert_eq!(combined, sub); // Wrapper should be removed.
	}

	#[test]
	fn test_named() {
		assert_eq!(
			MatchHit::named("word", 4, "wordplay"),
			MatchHit {
				type_name: "word".to_string(),
				length: 4,
				contents: "word".to_string(),
				sub_matches: Vec::new()
			}
		);
	}

	#[test]
	fn test_named_with_sub_matches() {
		let sub:MatchHit = MatchHit::named("sub", 2, "abcdef");
		let result:MatchHit = MatchHit::named_with_sub_matches("main", 3, "abcdef", vec![sub.clone()]);
		assert_eq!(result.type_name, "main");
		assert_eq!(result.contents, "abc");
		assert_eq!(result.sub_matches, vec![sub]);
	}



	/* CHILD SEARCH TESTS */

	#[test]
	fn test_type_name_tree_simple() {
		let root:MatchHit = MatchHit::named("root", 3, "abc");
		let tree:String = root.type_name_tree();
		assert_eq!(tree, "root");
	}

	#[test]
	fn test_type_name_tree_nested() {
		let root:MatchHit = MatchHit::named_with_sub_matches("root", 2, "xy", vec![
			MatchHit::named("child", 2, "xy")
		]);
		let tree:String = root.type_name_tree();
		assert_eq!(tree, "root\n| child");
	}

	#[test]
	fn test_find_child_matches_self() {
		assert!(MatchHit::named("self", 2, "ab").find_child(|c| c.type_name == "self").is_some());
	}

	#[test]
	fn test_find_child_nested() {
		let child:MatchHit = MatchHit::named("target", 2, "zz");
		let root:MatchHit = MatchHit::named_with_sub_matches("root", 2, "zz", vec![child.clone()]);
		let found:&MatchHit = root.find_child(|c| c.type_name == "target").unwrap();
		assert_eq!(found, &child);
	}

	#[test]
	fn test_find_children_nested() {
		let child_a:MatchHit = MatchHit::named("target", 2, "zz");
		let child_b:MatchHit = MatchHit::named("target", 2, "xx");
		let child_b_parent:MatchHit = MatchHit::named_with_sub_matches("target_parent", 2, "zz", vec![child_b.clone()]);
		let root:MatchHit = MatchHit::named_with_sub_matches("root", 2, "zz", vec![child_a.clone(), child_b_parent.clone()]);
		let found:Vec<&MatchHit> = root.find_children(|c| c.type_name == "target");
		assert_eq!(found, vec![&child_a, &child_b]);
	}

	#[test]
	fn test_find_child_none() {
		let root:MatchHit = MatchHit::named("root", 2, "ab");
		assert!(root.find_child(|c| c.type_name == "missing").is_none());
	}

	#[test]
	fn test_find_child_by_type_path_direct() {
		let match_result:MatchHit = MatchHit::named("direct", 2, "xy");
		assert!(match_result.find_child_by_type_path(&["direct"]).is_some());
	}

	#[test]
	fn test_find_child_by_type_path_nested() {
		let grandchild:MatchHit = MatchHit::named("gc", 1, "z");
		let child:MatchHit = MatchHit::named_with_sub_matches("c", 1, "z", vec![grandchild.clone()]);
		let root:MatchHit = MatchHit::named_with_sub_matches("r", 1, "z", vec![child]);
		let found:&MatchHit = root.find_child_by_type_path(&["r", "c", "gc"]).unwrap();
		assert_eq!(found, &grandchild);
	}

	#[test]
	fn test_find_child_by_type_path_none() {
		let root:MatchHit = MatchHit::named("root", 2, "ab");
		assert!(root.find_child_by_type_path(&["missing"]).is_none());
	}
}
