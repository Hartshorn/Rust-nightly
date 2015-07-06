
pub fn clearscreen() {
    print!("{}[{}", '\x1b', "2J");
}

pub fn writeat<'a >(position: &'a (i32, i32), string: &str) {
    let &(x, y) = position;

    print!("{}[{};{}H", '\x1b', x, y);
    print!("{}", string);
}

pub fn writeat_n<'a >(position: &'a (i32, i32), n: i32) {
    let &(x, y) = position;

    print!("{}[{};{}H", '\x1b', x, y);
    print!("{}", n);
}

pub fn writestrings(list: Vec<((i32, i32), &str)>) {
    for item in list {
        let (pos, string) = item;
        writeat(&pos, string);
    }
}

pub fn move_to_bottom(h: i32) {
    writeat(&(h + 5, 0), "\x1b");
}

pub fn y_axis(x: i32, y: i32) {
    for n in 0..10 {
        writeat(&(y + n, x), "|");
    }
}

pub fn x_axis(x: i32, y: i32) {
    for n in 0..30 {
        writeat(&(y, x + n), "-");
    }
}

pub fn draw_axis_at(x: i32, y: i32) {
    x_axis(x, y + 10);
    y_axis(x, y);
}

pub fn y_axis_nums(x: i32, y: i32) {
    for n in 0..11 {
        writeat_n(&(y + n, x - 2), 10 - n);
    }
}

pub fn x_axis_nums(x: i32, y: i32) {
    for n in 0..31 {
        writeat_n(&(y + 1, x + n), 30 - n);
    }
}

pub fn draw_axes_with_nums(x: i32, y: i32) {
    draw_axis_at(x, y);
    // x_axis_nums(x, y + 10); // these are bunched...
    y_axis_nums(x, y);
}

pub fn make_graph(oset: (i32, i32)) {
    let (x,y) = oset;
    draw_axes_with_nums(x, y);
}

pub fn graph(f: fn(i32) -> i32, oset: (i32, i32), n: i32) {
    let (o_x, o_y) = oset;

    for i in 0..n {
        writeat(&(i, f(i)), "*");
    }
}
