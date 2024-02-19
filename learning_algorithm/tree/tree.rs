#[derive(Clone)]
struct TreeNode {
	value: usize,
	children: Vec<TreeNode>
}

impl TreeNode {
	fn new(value: usize) -> TreeNode {
		TreeNode {
			value: value,
			children: Vec::new()
		}
	}

	fn add_child(&mut self, child: TreeNode) {
		self.children.push(child);
	}

	fn print_tree(&self) {
		println!("{}", self.value);
		for child in &self.children {
			child.print_tree();
		}
	}

	fn pre_order_traversal(&self, res: &mut Vec<usize>) {
		res.push(self.value);
		for child in &self.children {
			child.pre_order_traversal(res);
		}
	}
}

fn main() {
	let mut root = TreeNode::new(1);
	let child1 = TreeNode::new(2);
	let mut child2 = TreeNode::new(3);
	let mut child3 = TreeNode::new(4);
	let mut child4 = TreeNode::new(5);
	let mut child5 = TreeNode::new(6);
	let child6 = TreeNode::new(7);
	let child7 = TreeNode::new(8);
	let child8 = TreeNode::new(9);
	let child9 = TreeNode::new(10);

	root.add_child(child1.clone());
	root.add_child(child2.clone());
	root.add_child(child3.clone());
	child2.add_child(child4.clone());
	child2.add_child(child5.clone());
	child3.add_child(child6.clone());
	child4.add_child(child7.clone());
	child4.add_child(child8.clone());
	child5.add_child(child9.clone());

	let mut pre_order_list = Vec::new();
	root.pre_order_traversal(&mut pre_order_list);
	println!("Pre-order traversal: {:?}", pre_order_list);
}

