#[cfg(test)]
mod tests {
	use crate::{ TextMatchResult, languages::IniParser };



	#[test]
	fn test_ini() {
		const INI_CODE:&str = "[]\nempty data=x\n\n[user]\nname=bob\nage=32\njob=soap tester\n\n[test results]\ntest1=full failure\ntest2=partial failure\ntest3=success";
		let parser:IniParser = IniParser::new();
		let parse_result:TextMatchResult = parser.parse(INI_CODE);

		assert_eq!(
			parse_result.sub_matches.iter().filter(|result| result.type_name != "whitespace").map(|result| (result.type_name.as_str(), result.contents.as_str())).collect::<Vec<(&str, &str)>>(),
			vec![
				("category", "[]"),
				("variable", "empty data=x"),

				("category", "[user]"),
				("variable", "name=bob"),
				("variable", "age=32"),
				("variable", "job=soap tester"),

				("category", "[test results]"),
				("variable", "test1=full failure"),
				("variable", "test2=partial failure"),
				("variable", "test3=success"),
			]
		);
	}
}