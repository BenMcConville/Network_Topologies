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

    pub fn cycle(&mut self)  {
        for elem in self.node_connections   {
            if elem.relay != 0  {
                self.emit_update(elem.relay, elem.name_id);
                elem.relay = 0;      
            }
        }
    }
    fn emit_update(&mut self, signal: u8, source_id: u8)    {
        for elem in self.node_connections   {
            if elem.name_id != source_id    {
                elem.relay = signal;
            }
        }
    }
}

