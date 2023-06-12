/*##########################
##   Network Topologies   ##
##########################*/


/*	Structs
##########################*/

pub mod node;
pub mod connections;
//use super::node;

fn main()	{
	
	let n = node::new_node(3);
	let m = connections::new_connection(3);
	println!("{:#?}", m);
}
