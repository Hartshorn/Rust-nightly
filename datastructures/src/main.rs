mod linkedlist;
mod binarytree;

fn main() {

    let mut n1 = linkedlist::Node::new();
    let mut n2 = binarytree::Node::new();
    
    n1.insert(1);
    n1.insert(2);
    n1.insert(3);
    n1.insert_after(123, 2);
    // n1.insert_after(666, 999); // panic!
    
    println!("List contains: {:#?}", n1);
    println!("List Length: {}", n1.length());
    
    
    n2.insert(10, "ten".to_string());
    n2.insert(1, "one".to_string());
    n2.insert(3, "three".to_string());
    n2.insert(14, "fourteen".to_string());
    
    println!("Tree contains: {:#?}", n2);
    println!("Tree Length: {}", n2.length());
}