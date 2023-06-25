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
impl Network    {
    pub fn create_node(&mut self)   {
        match input_parsing("Enter connection line public ID") {
            Some(con_id) =>    {
                match input_parsing("Enter node public ID") {
                    Some(node_id) =>    {
                        self.nodes.insert(node_id, new_node(node_id));
                        self.connections.insert(con_id, {
                            let cons = self.connections.entry(con_id);
                            *cons.push(node_id)
                    }),
                    None => (),
                },
            None => (),
        }
    }
    pub fn create_connection(&mut self) {
        match input_parsing("Enter connection line public ID") {
            Some(num) => self.connections.insert(num, new_connection(num)),
            None => (),
        }

    }   
    
    pub fn Display_Network(&self)   {
        for (line_id, con) in &self.connectins  {
            println!("{}, {}", line_id, con);
        }
    }

}
