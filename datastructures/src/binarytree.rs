#[derive(Debug)]
pub enum Node {
    Entry {
        key:   String,
        data:  i32,
        left:  Box<Node>,
        right: Box<Node>
    },
    Nil
}

impl Node {
    
    pub fn new() -> Node {
        Node::Nil
    }
    
    pub fn insert(&mut self, new_data: i32, new_key: String) {
        match *self {
            Node::Entry { ref key, data, ref mut left, ref mut right } => {
                    if new_data > data {
                        right.insert(new_data, new_key);
                    } else {
                        left.insert(new_data, new_key);
                    }
            }
            Node::Nil => *self = Node::Entry {
                    key:   new_key,
                    data:  new_data, 
                    left:  Box::new(Node::Nil), 
                    right: Box::new(Node::Nil) 
            }
        }
    }
    
    pub fn length(&self) -> i32 {
        match *self {
            Node::Entry { ref key, data: _, ref left, ref right } => {
                1 + left.length() + right.length()
            }
            Node::Nil => 0,
        }
    }
    
    // pub fn search(&self, skey: String) {
        
    //     match *self {
    //         Node::Entry { ref key, data, left, ref right } => {
    //             if *key == skey {
    //                 println!("for key {} found {}", skey, data);
    //             } else {
    //                 right.search(skey);
    //             }
    //         },
    //         Node::Entry {ref key, data, ref left, right: _ } => {
    //             if *key == skey {
    //                 println!("for key {} found {}", skey, data);
    //             } else {
    //                 left.search(skey);
    //             }
    //         },
    //         Node::Nil => println!("Value not found"),
    //     }
    // }
}