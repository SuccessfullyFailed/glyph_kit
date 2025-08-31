#[cfg(test)]
mod tests {
	use crate::{ TextMatchResult, languages::IniParser };



	#[test]
	fn test_ini() {
		const INI_CODE:&str = "[]\nempty data=x\n\n[user]\nname=bob\nage=\t32\njob=soap tester\n\n[test results]\ntest1=full failure\ntest2=partial failure\ntest3=success";
		let parser:IniParser = IniParser::new().with_value_formatter(&|value| value.trim().to_string());
		let parse_result:TextMatchResult = parser.parse(INI_CODE);

		println!("{}", parse_result.type_name_tree());

		// Validate general value.
		const REMOVE_WHITESPACE_AND_GET_TYPE_AND_CONTENTS:fn(&[TextMatchResult]) -> Vec<(&str, &str)> = |matches_list| matches_list.iter().filter(|result| result.type_name != "whitespace").map(|result| (result.type_name.as_str(), result.contents.as_str())).collect::<Vec<(&str, &str)>>();
		assert_eq!(
			REMOVE_WHITESPACE_AND_GET_TYPE_AND_CONTENTS(&parse_result.sub_matches),
			vec![
				("group", "[]\nempty data=x"),
				("group", "[user]\nname=bob\nage=\t32\njob=soap tester"),
				("group", "[test results]\ntest1=full failure\ntest2=partial failure\ntest3=success")
			]
		);
		assert_eq!(
			REMOVE_WHITESPACE_AND_GET_TYPE_AND_CONTENTS(&parse_result.sub_matches[0].sub_matches),
			vec![
				("", "["),
				("category_name", ""),
				("variable_row", "empty data=x")
			]
		);
		assert_eq!(
			REMOVE_WHITESPACE_AND_GET_TYPE_AND_CONTENTS(&parse_result.sub_matches[2].sub_matches),
			vec![
				("", "["),
				("category_name", "user"),
				("variable_row", "name=bob"),
				("variable_row", "age=\t32"),
				("variable_row", "job=soap tester")
			]
		);

		// Get the age of the user.
		let user_obj:&TextMatchResult = parse_result.find_child(|child| child.sub_matches.iter().any(|sub_child| sub_child.type_name == "category_name" && sub_child.contents == "user")).unwrap();
		let age_obj:&TextMatchResult = user_obj.find_child(|child| child.type_name == "variable_row" && child.find_child(|sub_child| sub_child.contents == "age").is_some()).unwrap();
		let age:&str = &age_obj.find_child(|child| child.type_name == "value").unwrap().contents;
		assert_eq!(age, "32");
	}
}