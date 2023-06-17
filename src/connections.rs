use super::node::Node;

#[derive(Debug)]
pub struct Connection	{
	pub name_id: u8, //possibly private in future
	pub node_connections: Vec<Node>, 
}

pub fn new_connection(id: u8) -> Connection	{
	Connection	{
		name_id: id,
		node_connections: Vec::new(),
	}
}
impl Connection {
    pub fn add_node(&mut self, node: Node)  {
        self.node_connections.push(node);
    }
}
