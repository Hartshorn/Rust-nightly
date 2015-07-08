mod draw;
mod screenfunc;

use std::env;
use std::io::{self,BufRead};

fn main() {

    // cargo run -- $COLUMNS $LINES $(date +%S)

    let mut w: i32 = 0;
    let mut h: i32 = 0;
    let mut m: i32 = 0;
    let stdin = io::stdin();
    let mut line = String::new();


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
        } else if i == 3 {
            match a.parse::<i32>() {
                Ok(v)  => m = v,
                Err(e) => println!("Error: {}", e),
            }
        }
    }

    loop {
        stdin.lock().read_line(&mut line).unwrap();
        screenfunc::clearscreen();
        // draw::frame(w, h);
        //draw::moon(w, h, m);
        draw::stars(w, h);
        for _ in 1..6 {
            draw::mountain(w, h);
        }
        draw::groundfill(w, h);
        draw::move_to_bottom(h);
    }
}
