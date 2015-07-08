use std::mem;

/// The 'Node' type.
#[derive(Debug)]  // derives the trait for printing
pub enum Node {
    /// An entry in the list - contains data and next node
    Entry {
        data: i32,
        next: Box<Node>
    },
    /// A Nil value - used for new and end of list
    Nil
}

/// Implementation of Node type
impl Node {

    /// create a new Node - defaults to Nil
    pub fn new() -> Node { 
        Node::Nil 
    }
    
    // insert a Node (at the end of the list)
    pub fn insert(&mut self, data: i32) {
        match *self {
            Node::Entry { data: _, ref mut next } => next.insert(data),
            Node::Nil => *self = Node::Entry {  data: data, next: Box::new(Node::Nil) }
        }
    }
    
    /// get the length of the list of nodes
    pub fn length(&self) -> i32 {
        match *self {
            Node::Entry { data: _, ref next } => 1 + next.length(),
            Node::Nil => 0,
        }
    }
    
    /// insert a value at a specified location 
    pub fn insert_after(&mut self, new_data: i32, after: i32) {
        match *self {
            Node::Entry { data, ref mut next } => {
            
                if data == after {
                
                    let old = mem::replace(&mut **next, Node::Nil);
                    
                    **next = Node::Entry { data: new_data, next: Box::new(old) };
                    
                } else {
                
                    next.insert_after(new_data, after);
                }
            }
            Node::Nil => panic!("there is no element {}", after),
        }
    }
}