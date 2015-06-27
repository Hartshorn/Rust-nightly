
struct Log {
    ln: i32,
}

impl Drop for Log {
    fn drop(&mut self) {
        match self.ln {
            100 => println!("Logging in MAIN"),
            200 => println!("Logging in FUNCTION"),
            _   => println!("Logging in UNKNOWN"),
        }
    }
}

fn main() {
    
    Log { ln: 100 };
    
    println!("addem(3, 5) = {}", (|n: i32, m: i32| -> i32 { n + m })(3, 5));
    draw_map();
    
    Log { ln: 123 };
}

fn draw_map() {

    Log { ln: 200 };
    
    for _ in (0..10) {
        for _ in (0..10) {
            print!("{}  ", "*");
        }
        println!("");
    }
}