#[derive(Debug)]
pub struct Node	{
	pub name_id: u8,
    pub input_relay: u8, //Replace with enum data later
    pub output_relay: u8,
}
pub fn new_node(id: u8) -> Node	{
	Node {
		name_id: id,
        input_relay: 0,
        output_relay: 0,
	}
}
