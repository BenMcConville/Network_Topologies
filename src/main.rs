use std::io;
pub use std::collections::HashMap;
/*##########################
##   Network Topologies   ##
##########################*/

/*---Updates
- Add Hashmap for node/connection/network storage
    - Connot access same node from different lines
- Add single id parsing function (Network Module)
- Add Network module
*/


/*	Structs
##########################*/
pub mod node;
pub mod connections;
pub mod network;
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

pub fn input_parsing(prompt: &str) -> Option<u8>   {
    println!("{} ", prompt);
    let mut user_in = String::new();
    io::stdin().read_line(&mut user_in).expect("NVI__");
    match user_in.trim().parse()    {
        Ok(num) => {
            Some(num)
        }
        Err(_) =>   {
            None
        }
    }
}


fn main()	{
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let mut net = network::new_network(0);
    let mut run = true;
    //net.create_connection();
    //net.create_node();
    //net.Display_Network();
    while run   {
        match input_parsing("Select an option: \n1- Add node to line\n2- Create Line\n3- View network\n4- Send Signal\n")    {
            Some(0) => print!("{esc}[2J{esc}[1;1H", esc = 27 as char),
            Some(1) => net.create_node(),  
            Some(2) => net.create_connection(),
            Some(3) => net.Display_Network(),
            //4 => Send_Signal(mat),
            _ => println!("Non Valid Response")
            
        } 
    }
}
/*
fn run_op(mat: &mut Vec<connections::Connection>, n: u8)    {
    match n {
        0 => print!("{esc}[2J{esc}[1;1H", esc = 27 as char),
        1 => node_to_line(mat),    //Updated
        2 => line_to_network(mat), //Updated
        3 => Display_AdjM(mat),
        4 => Send_Signal(mat),
        _ => println!("Non Valid Response")
    }
    for line in mat {
        line.cycle();
    }
}
fn Send_Signal(mat : &mut Vec<connections::Connection>) {
    let mut line_id = String::new();
    println!("Enter Signal Line Source");
    io::stdin().read_line(&mut line_id).expect("NVI__");
    let line_id: u8 = match line_id.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => {
               255 
            },
    };
    let mut node_id = String::new();
    println!("Enter Node Source");
    io::stdin().read_line(&mut node_id).expect("NVI__");
    let node_id: u8 = match node_id.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => {
               255 
            },
    };
    for line in mat {
        if line.name_id == line_id  {
            for node in &mut line.node_connections   {
                if node.name_id == node_id  {
                    node.relay = 1;
                }
            }
        }
    }
}


fn check_for_existing_connection(vec: &Vec<connections::Connection>, node: u8) -> bool {//Use Generic
    for elem in vec  {
        if elem.name_id == node {
            return false;
        }
    }
    return true;
}
*/


