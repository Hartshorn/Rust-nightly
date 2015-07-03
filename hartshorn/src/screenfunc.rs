pub fn clearscreen() {
    print!("{}[{}", '\x1b', "2J");
}

pub fn writeat<'a >(position: &'a (i32, i32), string: &str) {
    let &(x, y) = position;

    print!("{}[{};{}H", '\x1b', x, y);
    print!("{}", string);
}

pub fn writestrings(list: Vec<((i32, i32), &str)>) {
    for item in list {
        let (pos, string) = item;
        writeat(&pos, string);
    }
}
    
    