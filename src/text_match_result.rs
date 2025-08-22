use std::fmt::{ Display, Formatter, Result };



#[derive(Clone, PartialEq, Debug)]
pub struct TextMatchResult {
	pub match_type:String,
	pub match_length:usize,
	pub match_contents:String
}
impl TextMatchResult {

	/// Create a new result.
	pub fn new(match_length:usize, source_text:&str) -> TextMatchResult {
		TextMatchResult {
			match_type: String::new(),
			match_length,
			match_contents: source_text[..match_length].to_string()
		}
	}

	/// Create a new result with a name.
	pub fn named(name:&str, match_length:usize, source_text:&str) -> TextMatchResult {
		let mut result:TextMatchResult = TextMatchResult::new(match_length, source_text);
		result.match_type = name.to_string();
		result
	}
}
impl Display for TextMatchResult {
	fn fmt(&self, f:&mut Formatter<'_>) -> Result {
		write!(f, "{}:\n{}\n\n", self.match_type, self.match_contents.split('\n').map(|line| format!(">>\t{line}")).collect::<Vec<String>>().join("\n"))
	}
}