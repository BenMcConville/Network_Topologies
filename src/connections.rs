use super::node::Node;

#[derive(Debug)]
pub struct Connection	{
	name_id: u8,
	node_connections: Vec<Node>, 
}

pub fn new_connection(id: u8) -> Connection	{
	Connection	{
		name_id: id,
		node_connections: Vec::new(),
	}
}

pub fn add_node(node: Node)
