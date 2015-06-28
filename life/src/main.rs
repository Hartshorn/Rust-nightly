use std::thread;

const WIDTH: i32  = 50;
const HEIGHT: i32 = 100;


fn main() {

    let c: i32 = WIDTH / 2_i32; // why is this backwards?
    let m: i32 = HEIGHT / 2_i32 + 20_i32;

    let glider: Vec<(i32, i32)> = vec![(14,12),
                                      (12,13),(14,13),
                                      (13,14),(14,14)];
    let blinker: Vec<(i32, i32)> = vec![(12,13),(13,13),(14,13)];
    // ********  *****   ***      ******* *****
    // lhs = (-20) ; rhs = (+20) ; mid = width / 2
    let flat: Vec<(i32, i32)> =
        vec![
            (c,m-18),(c,m-17),(c,m-16),(c,m-15),(c,m-14),(c,m-13),(c,m-12),(c,m-11),
            (c,m-9),(c,m-8),(c,m-7),(c,m-6),(c,m-5),
            (c,m-1),(c,m),(c,m+1),
            (c,m+8),(c,m+9),(c,m+10),(c,m+11),(c,m+12),(c,m+13),(c,m+14),
            (c,m+16),(c,m+17),(c,m+18),(c,m+19),(c,m+20),
        ];
    life(&flat);
}

fn life<'a >(board: &'a Vec<(i32, i32)>) {
    print!("{}[{}", '\x1b', "2J");
    showcells(&board);
    life(&nextgen(&board));
}

fn writeat<'a >(p: &'a (i32, i32), s: char) {
    let &(x, y) = p;

    print!("{}[{};{}H", '\x1b', x, y);
    print!("\x1b[38;5;9m{}\x1b[0m", s);
}


fn showcells<'a >(board: &'a Vec<(i32, i32)>) {
    for pos in board {
        writeat(pos, '*');
    }
}

fn wrap<'a >(pos: &'a (i32, i32)) -> (i32, i32) {
    let &(x,y) = pos;
    (((x - 1) % WIDTH) + 1, ((y - 1) % HEIGHT) + 1)
}

fn neighbors<'a >(pos: &'a (i32, i32)) -> Vec<(i32, i32)> {
    let &(x,y) = pos;
    let mut result: Vec<(i32, i32)> = vec!();
    let shift: Vec<(i32, i32)> = vec!((x-1, y-1), (x, y-1),
                                      (x+1, y-1), (x-1, y),
                                      (x+1, y)  , (x-1, y+1),
                                      (x, y+1 ) , (x+1, y+1));
    for p in shift {
        result.push(wrap(&p));
    }
    result
}

fn liveneighbors<'a >(board: &'a Vec<(i32, i32)>, pos: &'a (i32, i32)) -> usize {
    let mut alive: usize = 0;

    for n in neighbors(pos) {
        if board.contains(&n) {
            alive += 1_usize;
        }
    }
    alive
}

fn survivors<'a >(board: &'a Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut survivors: Vec<(i32, i32)> = vec!();
    for pos in board {
        if liveneighbors(&board, pos) == 2 || liveneighbors(&board, pos) == 3 {
            survivors.push(*pos);
        }
    }
    survivors
}

fn births<'a >(board: &'a Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut births: Vec<(i32, i32)> = vec!();

    for pos in board {
        for result in neighbors(pos) {
            if !board.contains(&result) && liveneighbors(board, &result) == 3 {
                births.push(result);
            }
        }
    }
    births.sort();
    births.dedup();

    births
}

fn nextgen<'a >(board: &'a Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut nextgen: Vec<(i32, i32)> = survivors(board);

    for b in births(&board) {
        nextgen.push(b);
    }
    nextgen
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap() {
        let pos1: (i32, i32) = (123, 345);
        let pos2: (i32, i32) = super::wrap(&pos1);
        let new_x: i32 = ((123 - 1) % super::WIDTH) + 1;
        let new_y: i32 = ((345 - 1) % super::HEIGHT) + 1;

        assert!(pos1 != pos2, "wrap: pos1 != pos2");
        assert!(pos2 == (new_x, new_y), "wrap: new_x, new_y");
    }

    #[test]
    fn test_dedup() {
        let mut rep: Vec<(i32, i32)> = vec![(1,2),(1,2),(1,2),(2,3)];

        assert!(rep == [(1,2),(1,2),(1,2),(2,3)], "pre dedup");
        rep.sort();
        rep.dedup();
        assert!(rep == [(1,2),(2,3)], "post dedup");
    }

    #[test]
    fn test_neighbors() {
        let pos: (i32, i32) = (2,3);
        assert!(super::neighbors(&pos).len() == 8, "neighbors");
    }

    #[test]
    fn test_liveneighbors() {
        // point
        let lhs: (i32, i32) = (2,3);
        let rhs: (i32, i32) = (4,3);
        let mid: (i32, i32) = (3,3);
        // row above it
        let tll: (i32, i32) = (1,2);
        let tpl: (i32, i32) = (2,2);
        let tpm: (i32, i32) = (3,2);
        let tpr: (i32, i32) = (4,2);
        let trr: (i32, i32) = (5,2);
        // row below it
        let bll: (i32, i32) = (1,4);
        let btl: (i32, i32) = (2,4);
        let btm: (i32, i32) = (3,4);
        let btr: (i32, i32) = (4,4);
        let brr: (i32, i32) = (5,4);
        // far left and right side
        let lls: (i32, i32) = (1,3);
        let rrs: (i32, i32) = (5,3);

        let blinker: Vec<(i32, i32)> = vec![lhs,mid,rhs];

        // check points
        assert!(super::liveneighbors(&blinker, &lhs) == 1, "lhs");
        assert!(super::liveneighbors(&blinker, &rhs) == 1, "rhs");
        assert!(super::liveneighbors(&blinker, &mid) == 2, "mid");
        // check left side
        assert!(super::liveneighbors(&blinker, &tll) == 1, "tll");
        assert!(super::liveneighbors(&blinker, &lls) == 1, "lls");
        assert!(super::liveneighbors(&blinker, &bll) == 1, "bll");
        // check right side
        assert!(super::liveneighbors(&blinker, &trr) == 1, "trr");
        assert!(super::liveneighbors(&blinker, &rrs) == 1, "rrs");
        assert!(super::liveneighbors(&blinker, &brr) == 1, "brr");
        // check top row
        assert!(super::liveneighbors(&blinker, &tpl) == 2, "tpl");
        assert!(super::liveneighbors(&blinker, &tpm) == 3, "tpm");
        assert!(super::liveneighbors(&blinker, &tpr) == 2, "tpr");
        // check bottom row
        assert!(super::liveneighbors(&blinker, &btl) == 2, "btl");
        assert!(super::liveneighbors(&blinker, &btm) == 3, "btm");
        assert!(super::liveneighbors(&blinker, &btr) == 2, "btr");
    }

    #[test]
    fn test_births() {
        let blinker: Vec<(i32, i32)> = vec![(2,3),(3,3),(4,3)];
        let mut liven_vec: Vec<(i32, i32)> = vec!();
        let mut onboard_vec: Vec<(i32, i32)> = vec!();
        let mut count_total: usize   = 0;
        let actual_length: usize = super::nextgen(&blinker).len();

        for pos in &blinker {
            for res in super::neighbors(pos) {
                count_total += 1;
                if blinker.contains(&res) {
                    onboard_vec.push(res);
                }
                if super::liveneighbors(&blinker, &res) == 3 {
                    liven_vec.push(res);
                }
            }
        }
        liven_vec.sort();
        onboard_vec.sort();

        liven_vec.dedup();
        onboard_vec.dedup();

        assert!(actual_length == 3, "Actual length: {}",
                                        actual_length);
        assert!(count_total == 24, "Total checked: {}",
                                        count_total);
        assert!(liven_vec.len() == 2, "Total with 3 live neighbors: {}",
                                            liven_vec.len());
        assert!(onboard_vec.len() == 3, "Total found on board: {}",
                                            onboard_vec.len());
    }

    #[test]
    fn test_survivors() {
        let blinker: Vec<(i32, i32)> = vec![(2,3),(3,3),(4,3)];
        let survivors: Vec<(i32, i32)> = super::survivors(&blinker);

        assert!(survivors.len() == 1, "Total survivors: {}", survivors.len());
    }

    #[test]
    fn test_nextgen() {
        let blinker: Vec<(i32, i32)> = vec![(2,3),(3,3),(4,3)];
        let nextgen: Vec<(i32, i32)> = super::nextgen(&blinker);

        assert!(nextgen.len() == 3, "Nextgen length: {}", nextgen.len());
    }
}
