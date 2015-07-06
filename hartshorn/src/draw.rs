extern crate rand;

use self::rand::distributions::{IndependentSample, Range};
use screenfunc;

pub fn move_to_bottom(h: i32) {
    screenfunc::writeat(&(h + 5, 0), "\x1b");
}

fn get_random(axis: &str, w: i32, h: i32) -> i32 {
    match axis {
        "y" => {
                    let between  = Range::new(0_i32, h / 2);
                    let mut rng = rand::thread_rng();

                    between.ind_sample(&mut rng) + 2
                },
        "x" => {
                    let between  = Range::new(0_i32, w - 1);
                    let mut rng = rand::thread_rng();

                    between.ind_sample(&mut rng)
                },
        "p_y" => {
                    let between  = Range::new(h / 3, 2 * h / 3);
                    let mut rng = rand::thread_rng();

                    between.ind_sample(&mut rng) + 2
                },
        "m_y" => {
                    let between  = Range::new(0, h / 3);
                    let mut rng = rand::thread_rng();

                    between.ind_sample(&mut rng) + 2
                },
        _ => unreachable!("Invalid value passed to get_random()!"),
    }
}

pub fn frame(w: i32, h: i32) {

    let top: &str       = "\x1b[34m \x1b[0m";
    let bottom: &str    = "\x1b[34m \x1b[0m";
    let leftside: &str  = "\x1b[34m \x1b[0m";
    let rightside: &str = "\x1b[34m \x1b[0m";

    for y in 1..h + 1 {
        screenfunc::writeat(&(y, 0), leftside);
        if y - 1 == 0 {
            for x in 2..w - 1 {
                screenfunc::writeat(&(y, x), top);
            }
        } else if y == h {
            for x in 2..w - 1 {
                screenfunc::writeat(&(y, x), bottom);
            }
        } else {
            screenfunc::writeat(&(y, w), rightside);
        }
    }
    println!("");
}

pub fn stars(w: i32, h: i32) {

    let mut stars: Vec<((i32, i32), &str)> = vec!();
    let whitestar: &str = "\x1b[0m*\x1b[0m";

    for _ in 1..50 {
        stars.push(((get_random("y", w, h), get_random("x", w, h)), whitestar));
    }
    screenfunc::writestrings(stars);
}

pub fn mountain(w: i32, h: i32) {

    let mut mountain: Vec<((i32, i32), &str)> = vec!();
    let (y, x) = (get_random("p_y", w, h), get_random("x", w, h));
    let range = 0 .. h - y - 5;

    let mountainleft: &str      = "\x1b[34m \x1b[0m";
    let mountainright: &str     = "\x1b[34m \x1b[0m";
    let mountainfillgrey: &str  = "\x1b[30;1mM\x1b[0m";
    let mountainfillwhite: &str = "\x1b[37;1mM\x1b[0m";

    for n in range {
        let xplusn:  i32 = x + n;
        let xminusn: i32 = x - n;
        let yplusn:  i32 = y + n;

        if xplusn >= w - 1 {
            mountain.push(((yplusn, xminusn), mountainleft));

            for i in xminusn + 1 .. w - 1 {
                if yplusn > y + 10 {
                    mountain.push(((yplusn, i), mountainfillgrey));
                } else {
                    mountain.push(((yplusn, i), mountainfillwhite));
                }
            }
        } else if xminusn < 2 {
            mountain.push(((yplusn, xplusn), mountainright));

            for i in 2 .. xplusn - 1 {
                if yplusn > y + 10 {
                    mountain.push(((yplusn, i), mountainfillgrey));
                } else {
                    mountain.push(((yplusn, i), mountainfillwhite));
                }
            }
        } else {
            mountain.push(((yplusn, xplusn), mountainright));
            mountain.push(((yplusn, xminusn), mountainleft));

            for i in xminusn + 1.. xplusn - 1 {
                if yplusn > y + 10 {
                    mountain.push(((yplusn, i), mountainfillgrey));
                } else {
                    mountain.push(((yplusn, i), mountainfillwhite));
                }
            }
        }
    }
    screenfunc::writestrings(mountain);
}

pub fn groundfill(w: i32, h: i32) {

    let grasscolor: &str = "\x1b[32;1mi\x1b[0m";
    let mut grass: Vec<((i32, i32), &str)> = vec!();

    for y in h - 5 .. h {
        for x in 1 .. w - 1 {
            grass.push(((y,x), grasscolor));
        }
    }
    screenfunc::writestrings(grass);
}

pub fn moon(w: i32, h: i32, m: i32) {

    let mooncolor: &str = "\x1b[37;1mo\x1b[0m";
    let mut moon: Vec<((i32, i32), &str)> = vec!();

    let (x, y) = (get_random("m_y", w, h), get_random("x", w, h));

    for n in y - 1 .. y + 2 {
        moon.push(((n, x), mooncolor));
    }
    for n in y .. y + 1 {
        moon.push(((n, x - 1), mooncolor));
    }
    for n in y .. y + 1 {
        moon.push(((n, x + 1), mooncolor));
    }
    moon.push(((y, x - 2), mooncolor));
    moon.push(((y, x + 2), mooncolor));

    screenfunc::writestrings(moon);
}
