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
impl Connection {
    pub fn node_waiting(&self) -> bool  {
        for node in &self.node_connections  {
            if node.output_relay != 0   {
                return true;
            }
        }
        return false;
    }
    pub fn cycle(&self) -> &Vec<u8>  {//Currently Half-Duplex
        &self.node_connections
    }   
}
