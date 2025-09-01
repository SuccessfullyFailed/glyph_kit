use std::fmt::{ Display, Formatter, Result };



#[derive(Clone, PartialEq, Debug)]
pub struct MatchHit {
	pub type_name:String,
	pub length:usize,
	pub contents:String,
	pub sub_matches:Vec<MatchHit>
}
impl MatchHit {

	/* CONSTRUCTOR METHODS */

	/// Create a new result.
	pub fn new(match_length:usize, source_text:&str) -> MatchHit {
		MatchHit {
			type_name: String::new(),
			length: match_length,
			contents: source_text[..match_length].to_string(),
			sub_matches: Vec::new()
		}
	}

	/// Create a new result.
	pub fn new_with_sub_matches(match_length:usize, source_text:&str, sub_matches:Vec<MatchHit>) -> MatchHit {
		let mut result:MatchHit = MatchHit::new(match_length, source_text);
		result.sub_matches = sub_matches;
		result.combine_sub_matches();
		if result.sub_matches.len() == 1 {
			result = result.sub_matches.remove(0);
		}
		result
	}

	/// Create a new result with a name.
	pub fn named(name:&str, match_length:usize, source_text:&str) -> MatchHit {
		let mut result:MatchHit = MatchHit::new(match_length, source_text);
		result.type_name = name.to_string();
		result
	}

	/// Create a new result.
	pub fn named_with_sub_matches(name:&str, match_length:usize, source_text:&str, sub_matches:Vec<MatchHit>) -> MatchHit {
		let mut result:MatchHit = MatchHit::named(name, match_length, source_text);
		result.sub_matches = sub_matches;
		result.combine_sub_matches();
		result
	}



	/* USAGE METHODS */

	/// Execute an action on this and all sub-results.
	pub fn execute_recursive<T:Fn(&MatchHit) + 'static>(&self, action:T) {
		self._execute_recursive(&action);
	}
	pub fn _execute_recursive(&self, action:&dyn Fn(&MatchHit)) {
		action(self);
		for sub_match in &self.sub_matches {
			sub_match._execute_recursive(action);
		}
	}

	/// Execute an action on this and all sub-results mutable.
	pub fn execute_recursive_mut<T:Fn(&mut MatchHit) + 'static>(&mut self, action:T) {
		self._execute_recursive_mut(&action);
	}
	pub fn _execute_recursive_mut(&mut self, action:&dyn Fn(&mut MatchHit)) {
		action(self);
		for sub_match in &mut self.sub_matches {
			sub_match._execute_recursive_mut(action);
		}
	}



	/* CHILD METHODS */

	/// Combine sub-matches.
	pub fn combine_sub_matches(&mut self) {
		let mut left_index:usize = 0;
		let mut right_index:usize = 1;
		while right_index < self.sub_matches.len() {
			if self.sub_matches[left_index].type_name.is_empty() && self.sub_matches[right_index].type_name.is_empty() {
				let right:MatchHit = self.sub_matches.remove(right_index);
				self.sub_matches[left_index].length += right.length;
				self.sub_matches[left_index].contents += &right.contents;
				self.sub_matches[left_index].sub_matches.extend(right.sub_matches);
			} else {
				left_index += 1;
				right_index += 1;
			}
		}
	}

	/// Create a string containing a tree of child type names.
	pub fn type_name_tree(&self) -> String {
		const PADDING:&str = "| ";
		self._type_name_tree(0).into_iter().map(|(depth, name)| PADDING.repeat(depth) + &name).collect::<Vec<String>>().join("\n")
	}
	/// Create a list of the depth and type name of each child.
	fn _type_name_tree(&self, current_depth:usize) -> Vec<(usize, String)> {
		let mut depth_list:Vec<(usize, String)> = vec![];
		let mut child_depth:usize = current_depth;
		if !self.type_name.is_empty() {
			depth_list.push((current_depth, self.type_name.clone()));
			child_depth += 1;
		}
		depth_list.extend(self.sub_matches.iter().map(|child| child._type_name_tree(child_depth)).flatten().collect::<Vec<(usize, String)>>());
		depth_list
	}

	/// Find a specific child by filter.
	pub fn find_child<T:Fn(&MatchHit) -> bool>(&self, filter:T) -> Option<&MatchHit> {
		self._find_child(&filter)
	}
	pub fn _find_child(&self, filter:&dyn Fn(&MatchHit) -> bool) -> Option<&MatchHit> {
		if filter(self) {
			return Some(self);
		}
		for sub_match in &self.sub_matches {
			if let Some(child) =  sub_match._find_child(filter) {
				return Some(child);
			}
		}
		None
	}

	/// Find a specific child by a path of type names.
	pub fn find_child_by_type_path(&self, type_path:&[&str]) -> Option<&MatchHit> {
		if type_path.is_empty() {
			return None;
		}
		if self.type_name == type_path[0] {
			if type_path.len() <= 1 {
				return Some(self);
			}
			for sub_match in &self.sub_matches {
				if let Some(found_child) = sub_match.find_child_by_type_path(&type_path[1..]) {
					return Some(found_child);
				}
			}
		}
		for sub_match in &self.sub_matches {
			if let Some(found_child) = sub_match.find_child_by_type_path(type_path) {
				return Some(found_child);
			}
		}
		None
	}
}
impl Display for MatchHit {
	fn fmt(&self, f:&mut Formatter<'_>) -> Result {
		write!(f, "{}:\n{}\n\n", self.type_name, self.contents.split('\n').map(|line| format!(">>\t{line}")).collect::<Vec<String>>().join("\n"))
	}
}