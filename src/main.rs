/*##########################
##   Network Topologies   ##
##########################*/


/*	Structs
##########################*/

pub mod node;
pub mod connections;
//use super::node;

/* Ring 
 0 1 2 3 4 5
00 1 0 0 0 0
10 0 1 0 0 0
20 0 0 1 0 0
30 0 0 0 1 0
40 0 0 0 0 1
51 0 0 0 0 0   
*/

fn main()	{
	
	let n = node::new_node(3);
	let mut m = connections::new_connection(3);
    let q = node::new_node(2);
    m.add_node(q);
	println!("{:#?}", m);
}
