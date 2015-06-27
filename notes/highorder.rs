#![feature(core)]

fn main() {
    let upper = 1000;
    
    let is_odd = |n: u32| -> bool { n % 2 == 1 };
    
    let print_n = |n: string
    
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)
             .take_while(|&n| n < upper)
             .filter(|n| is_odd(*n))
             .sum();
    println!("{}", sum_of_squared_odd_numbers);
}