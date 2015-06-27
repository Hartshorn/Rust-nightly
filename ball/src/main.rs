// lets bounce a ball!
use std::thread;

const WIDTH:  i32 = 10;
const HEIGHT: i32 = 8;

enum Wall { North, South, East, West, TLeft, TRight, BLeft, BRight }

struct Ball{
    x: i32,
    y: i32,
    d: i32,
}

fn main() {
    let mut ball = Ball { x: WIDTH / 2, y: HEIGHT / 2, d: 7};
    
    loop {
        tick(&mut ball);
        thread::sleep_ms(80);
    }
}

fn tick<'a > (ball: &'a mut Ball) {
    print!("\x1B[2J"); //clear the screen
    mv(ball);
    show_world(ball);
}

fn mv<'a >(ball: &'a mut Ball) {
    if collision(ball.x, ball.y, ball.d) {
        if is_corner(ball.x, ball.y) {
            ball.d = flip(get_corner(ball.x, ball.y), 
                          true, 
                          direct(ball.d, 
                                 get_corner(ball.x, ball.y), 
                                 true),
                          0);
        } else {
            ball.d = flip(get_wall(ball.x, ball.y), 
                          false, 
                          direct(ball.d, 
                                 get_wall(ball.x, ball.y), 
                                 false), 
                          ball.d);
        }
    }
    ball.x += d_fac(ball.d, true);
    ball.y += d_fac(ball.d, false);
}

fn is_corner(x: i32, y: i32) -> bool {
    match (x,y) {
        (0,0)           => true,
        (WIDTH,0)       => true,
        (0,HEIGHT)      => true,
        (WIDTH,HEIGHT)  => true,
        (_,_)           => false,
    }
}

fn get_corner(x: i32, y: i32) -> Wall {
    match (x, y) {
        (0,0) => Wall::TLeft,
        (_,0) => Wall::TRight,
        (0,_) => Wall::BLeft,
        (_,_) => Wall::BRight,
    }
}

fn d_fac(d: i32, is_x: bool) -> i32 {
    
    if is_x {
        match d {
            5...7 => -1,
            1...3 =>  1,
            _ => 0,
        }
    } else {
        match d {
            0 | 1 | 7 => -1,
            2 | 6     =>  0,
            _ => 1,
        }
    }
}

fn collision(x: i32, y: i32, d: i32) -> bool {
    
    let in_list =
        |n: i32, (x, y, z): (i32, i32, i32)| -> bool 
            { n == x || n == y || n == z };
    
    if x == 0 && in_list(d, (5,6,7)) {
        true
    } else if x == WIDTH && in_list(d, (1,2,3)) {
        true
    } else if y == 0 && in_list(d, (7,0,1)) {
        true
    } else if y == HEIGHT && in_list(d, (3,4,5)) {
        true
    } else {
        false
    }
}

fn get_wall(x: i32, y: i32) -> Wall {

    match (x,y) {
        (0,_)       => Wall::West,
        (WIDTH,_)   => Wall::East,
        (_,0)       => Wall::North,
        (_,HEIGHT)  => Wall::South,
        (_,_) => unreachable!("get_wall NOT FOUND!"),
    }
}

fn direct(d: i32, wall: Wall, corner: bool) -> bool {
    if corner {
        match (d, wall) {
            (7, Wall::TLeft)  => true,
            (1, Wall::TRight) => true,
            (5, Wall::BLeft)  => true,
            (3, Wall::BRight) => true,
            (_,_) => false,
        }
    } else {
        match (d, wall) {
            (0, Wall::North) => true,
            (6, Wall::West)  => true,
            (4, Wall::South) => true,
            (2, Wall::East)  => true,
            (_,_) => false,
        }
    }
}

fn angle(d: i32 , wall: Wall) -> i32 {
    match (d, wall) {
        (3, Wall::East)  => 5,
        (1, Wall::East)  => 7,
        (7, Wall::West)  => 1,
        (5, Wall::West)  => 3,
        (7, Wall::North) => 5,
        (1, Wall::North) => 3,
        (5, Wall::South) => 7,
        (3, Wall::South) => 1,
        (_,_) => unreachable!("angle, wall / d: no match!"),
    }
}

fn flip(wall: Wall, corner: bool, direct: bool, d: i32) -> i32 {
    
    if corner {
        if direct {
            match wall {
                Wall::TRight => 5,
                Wall::TLeft  => 3,
                Wall::BRight => 7,
                Wall::BLeft  => 1,
                _ => unreachable!("set_opp_d, corner, direct"),
            }
         }
        else {
            // simple case for hitting corner while running along a wall
            // only happens out of starting postion e.g. x:0, y:0, d:0
            match (wall, d) {
                (Wall::TRight,0) => 4,
                (Wall::TLeft, 0) => 4,
                (Wall::BRight,4) => 0,
                (Wall::BLeft, 4) => 0,
                (Wall::TRight,2) => 6,
                (Wall::BRight,2) => 6,
                (Wall::TLeft, 6) => 2,
                (Wall::BLeft, 6) => 2,
                (_,_) => unreachable!("set_opp_d, corner, not direct"),
            }
        }
    } else {
        if direct {
            match wall {
                Wall::North => 4,
                Wall::East  => 6,
                Wall::West  => 2,
                Wall::South => 0,
                _ => unreachable!("set_opp_d, wall, direct"),
            }
        } else {
            match wall {
                Wall::North => angle(d, Wall::North),
                Wall::East  => angle(d, Wall::East),
                Wall::South => angle(d, Wall::South),
                Wall::West  => angle(d, Wall::West),
                _ => unreachable!("set_opp_d, wall, not direct"),
            }
        }
    }
}

fn print_line() {
    print!(" ");
    for _ in 0..WIDTH + 1 {
        print!("\x1b[4m   \x1b[24m");
    }
    println!("");
}

fn mark(result: i32) {
    match result {
        0 => print!("\x1b[38;5;46m O \x1b[0m"),
        1 => print!("\x1b[38;5;9m * \x1b[0m"),
        _ => unreachable!("mark, Bad result code!"),
    }
}

fn show_world(b: &mut Ball) {
    print!("\x1B[5;0H");    // move cursor to 5,0
    print_line();
    for y in 0..(HEIGHT + 1) {
        print!("|");
        for x in 0..(WIDTH + 1) {
            if b.x == x && b.y == y {
                mark(0);
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