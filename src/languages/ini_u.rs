#[cfg(test)]
mod tests {
	use crate::{ TextMatchResult, languages::IniParser };



	#[test]
	fn test_ini() {
		const INI_CODE:&str = "[]\nempty data=x\n\n[user]\nname=bob\nage=32\njob=soap tester\n\n[test results]\ntest1=full failure\ntest2=partial failure\ntest3=success";
		let parser:IniParser = IniParser::new();
		let parse_result:Vec<TextMatchResult> = parser.parse(INI_CODE);

		assert_eq!(
			parse_result.iter().filter(|result| result.match_type != "whitespace").map(|result| (result.match_type.as_str(), result.match_contents.as_str())).collect::<Vec<(&str, &str)>>(),
			vec![
				("category", "[]"),
				("value", "empty data=x"),

				("category", "[user]"),
				("value", "name=bob"),
				("value", "age=32"),
				("value", "job=soap tester"),

				("category", "[test results]"),
				("value", "test1=full failure"),
				("value", "test2=partial failure"),
				("value", "test3=success"),
			]
		);
	}
}