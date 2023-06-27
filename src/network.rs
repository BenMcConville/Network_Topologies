use super::node::{Node, new_node};//can call super or crate
use super::connections::{Connection, new_connection};
use super::input_parsing;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Network  {
    pub network_id: u8,
    pub nodes: HashMap<u8, Node>,
    pub connections : HashMap<u8, Connection>,
}
pub fn new_network(id: u8) -> Network   {
    Network {
        network_id: id,
        nodes: HashMap::new(),
        connections: HashMap::new(),
    }
}
impl Network    {

    pub fn create_node(&mut self)   { //Fix this later too many match statements looks ugly
        match input_parsing("Enter connection line public ID") {
            Some(con_id) => {
                match input_parsing("Enter node public ID") {
                    Some(node_id) =>    {
                        self.nodes.insert(node_id, new_node(node_id)); // This adds node to general node heap
                        match self.connections.get_mut(&con_id) {
                            Some(connection_line) => {
                                connection_line.node_connections.push(node_id);
                            },
                            None => println!("Node or Connection Line does not exist"),
                        }
                        println!("{:?}", self.connections.get_mut(&con_id));//.node_connections.push(node_id);   // This updates specific connection line with its vector nodes
                        //self.connections.insert(con_id, self.connections[&con_id]);     // This adds updated connection line to hashmap
                    },
                    None => (),
                };
            },
            None => (),
        }
    }
    
    pub fn create_connection(&mut self) {
        match input_parsing("Enter connection line public ID") {
            Some(num) =>    {
                self.connections.insert(num, new_connection(num));
            },
            None => (),
        }

    }   
    
    pub fn Display_Network(&self)   {
        for (line_id, con) in &self.connections  {
            println!("{}, {:?}", line_id, con);
        }
    }
    pub fn node_waiting(&mut self)  {

    }
    pub fn cycle(&mut self) {
        for (line_id, con) in &mut self.connections {
            for node_ids in &mut 
        } 
    }
}
