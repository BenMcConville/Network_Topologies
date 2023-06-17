use std::io;
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
	
    let mut network: Vec<connections::Connection> = Vec::new();
	let mut run = true;
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    while run   {
        println!("Select an option: \n1- Add node to line\n2- Create Line\n3- View network");
        let mut userIn = String::new();
        io::stdin().read_line(&mut userIn).expect("NVI__");
        let userIn: u8 = match userIn.trim().parse() {
            Ok(num) => {
                run_op(&mut network, num);
                num
            },
            Err(_) => {
                run = false;
                continue
            },
        };
    }
    println!("Enter a valid input");
    /*
    let n = node::new_node(3);
	let mut m = connections::new_connection(3);
    let q = node::new_node(2);
    m.add_node(q);
    network.push(m);*/
	//println!("{:#?}", network);
    //Display_AdjM(&network);
}

fn run_op(mat: &mut Vec<connections::Connection>, n: u8)    {
    println!("You Picked {}", n);
    match n {
        0 => print!("{esc}[2J{esc}[1;1H", esc = 27 as char),
        1 => node_to_line(),
        2 => line_to_network(mat),
        3 => Display_AdjM(mat),
        _ => println!("Non Valid Response")
    }
}

fn node_to_line()   {
        
}

fn line_to_network(mat: &mut Vec<connections::Connection>)    {
    println!("Enter Id or nothing to auto assign Id"); 
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("NVI__");
    let id: u8 = match id.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => {
               255 
            },
    };
    mat.push(connections::new_connection(id));
    
    println!("{:?}", &mat);

}


fn Display_AdjM(mat: &Vec<connections::Connection>) {
    println!("{:?}", mat);
    for line in mat  {
        println!("{}: ", line.name_id);
        for elem in &line.node_connections {
            print!("{}", elem.name_id);
        }
        println!("");
    }
}
