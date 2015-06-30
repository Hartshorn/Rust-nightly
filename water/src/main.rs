const WIDTH: i32  = 20;
const HEIGHT: i32 = 20;


struct Drop {
    x: i32,
    y: i32,
}

fn main() {
    let d: Drop = Drop{ x: WIDTH / 2, y: HEIGHT / 2 };

    draw(d);
}

fn draw(drop: Drop) {
    // print!("\x1B[5;0H");    // move cursor to 5,0
    print_line();
    for y in 0..(HEIGHT + 1) {
        print!("|");
        for x in 0..(WIDTH + 1) {
            if drop.x == x && drop.y == y {
                draw_shape_at(x,y,"star");
                // print!(" * ");
            } else {
                if y == (HEIGHT) {
                    print!("___");
                } else {
                    print!("   ");
                }
            }
        }
        println!("|");
    }
}

fn draw_shape_at(x: i32, y: i32, name: &str) {
    print!("\x1b[{};{}H", x, y);
    print!("*");
}

fn print_line() {
    print!(" ");
    for _ in 0..WIDTH + 1 {
        print!("\x1b[4m   \x1b[24m");
    }
    println!("");
}
