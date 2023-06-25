use super::node::Node;

#[derive(Debug)]
pub struct Connection	{
	pub name_id: u8, //possibly private in future
	pub node_connections: Vec<u8>, 
}

pub fn new_connection(id: u8) -> Connection	{
	Connection	{
		name_id: id,
		node_connections: Vec::new(),
	}
}
/*
impl Connection {
    pub fn add_node(&mut self, node: Node)  {
        self.node_connections.push(node);
    }
    pub fn cycle(&mut self)  {//Currently Half-Duplex
        let mut signal: u8 = 0; 
        let mut source_id: u8 = 0;
        for elem in &mut self.node_connections   {
            if elem.relay != 0  {
                signal = elem.relay;
                source_id = elem.name_id;
                elem.relay = 0;      
            }
        }
        for elem in &mut self.node_connections  {
            if elem.name_id != source_id    {
                elem.relay = signal;
            }
        }
    }
}
*/
