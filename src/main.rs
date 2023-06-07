/*##########################
##   Network Topologies   ##
##########################*/


/*	Structs
##########################*/


//----- Nodes -----
#[derive(Debug, Clone)]
struct NetworkNode	{
	rank: u8,
	name: String,
	status: bool,
	transmitting: bool,
	connections: TransmissionLine,
	input:	bool,
}
//----- Node Connection Lines -----
#[derive(Debug, Clone)]
struct TransmissionLine	{
	attached_nodes: Vec<NetworkNode>,
	status: bool,
	data: bool,
}
/*    Structs Methods
##########################*/
impl NetworkNode	{
	fn transmit_signal(mut self, bit: bool)	{
		self.connections = TransmissionLine	{
			status: true,
			data: bit,
			attached_nodes: self.connections.attached_nodes,
		};
	}
}

impl TransmissionLine	{
	fn add_node(mut self, nnode: NetworkNode)	{
		self = TransmissionLine	{
			attached_nodes: self.attached_nodes.push(nnode),
			..self
		};	
	}
}


fn main() {
	let con01 = TransmissionLine	{
		attached_nodes: Vec::new(),
		status: false,
		data: false,
	};
	let node01 = NetworkNode	{
		rank: 0,
		name: String::from("Node_01"),
		status: true,
		transmitting: false,
		connections:	con01,
		input: false,
	};
	
}
