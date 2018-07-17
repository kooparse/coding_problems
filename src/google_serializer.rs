/// Problem asked by Google:
/// Given the root to a binary tree, implement serialize(root),
/// which serializes the tree into a string, and deserialize(s),
/// which deserializes the string back into the tree.

type ChildrenType = Option<Box<Node>>;

#[derive(Default, Debug)]
pub struct Node {
	value: String,
	pub left: ChildrenType,
	pub right: ChildrenType,
}

impl Node {
	pub fn new(value: &str, left: ChildrenType, right: ChildrenType) -> Self {
		Self {
			value: value.to_owned(),
			left,
			right,
		}
	}
}

// TODO: Could be more Rust idiomatic
pub fn serialize(node: Node) -> String {
	let value = node.value;

	let mut left = String::from("#");
	let mut right = left.clone();

	if node.left.is_some() {
		// Deref the boxing value to stringify it
		left = serialize(*node.left.unwrap());
	}

	if node.right.is_some() {
		right = serialize(*node.right.unwrap());
	}

	format!("{},{},{}", value, left, right)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn serializer() {
		let tree = Node::new(
			"root",
			Some(Box::new(Node::new(
				"left",
				Some(Box::new(Node::new("left.left", None, None))),
				None,
			))),
			Some(Box::new(Node::new("right", None, None))),
		);

		assert_eq!("root,left,left.left,#,#,#,right,#,#", serialize(tree));
	}

}
