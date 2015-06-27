use std::env;

fn main() {
    // let mut argv = env::args();
    // let arg: String = argv.nth(1).unwrap();
    // let n: i32 = arg.parse().unwrap();
    // println!("{}", 2 * n);
    
    let mut word: &str = "HOLYMOLY";
    let a: char = 'L';
    
    match find(word, a) {
        None => println!("{} not found in word!", a),
        Some(i) => println!("Letter and more: {}", &word[i+1..]),
    }
}

fn find(haystack: &str, needle: char) -> Option<usize> {
    for (index, character) in haystack.char_indices() {
        if character == needle {
            return Some(index);
        }
    }
    None
}