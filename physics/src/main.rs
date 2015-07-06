
use std::env;

mod screen;

fn main() {

    // cargo run -- $COLUMNS $LINES

    let (mut w, mut h): (i32, i32) = (0, 0);
    let oset: (i32, i32) = (5,5);

    for (i, a) in env::args().enumerate() {
        if i == 1 {
            match a.parse::<i32>() {
                Ok(v)  => w = v,
                Err(e) => println!("Error: {}", e),
            }
        } else if i == 2 {
            match a.parse::<i32>() {
                Ok(v)  => h = v,
                Err(e) => println!("Error: {}", e),
            }
        }
    }

    screen::clearscreen();
    // screen::make_graph(oset);
    screen::graph(f, oset, 10);

    screen::move_to_bottom(h);
}


fn f(x: i32) -> i32 {
    x*x
}
