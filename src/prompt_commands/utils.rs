pub fn matches_to_string(list: Vec<&String>) -> String {
	list.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" ")
}