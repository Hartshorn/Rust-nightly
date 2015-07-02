extern crate rand;
use rand::random;

fn main() {

    let mut nums: Vec<i32> = vec!();

    for _ in 1..10 {
        nums.push(get_random().abs());
    }

    for next in nums.iter() {
        println!("{}", next);
    }
}

fn get_random() -> i32 {
    random::<i32>() % 10
}
