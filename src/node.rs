#[derive(Debug)]
pub struct Node	{
	name_id: u8,
}
pub fn new_node(id: u8) -> Node	{
	Node {
		name_id: id,
	}
}
