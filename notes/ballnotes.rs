trait Move {
    fn mv<'a >(&'a mut self);
}

struct Ball{
    x: i32,
    y: i32,
    d: i32,
}

impl Move for Ball {
    fn mv<'a >(&'a mut self) {
        
        if collision(self.x, self.y, self.d) {
            if is_corner(self.x, self.y) {
                self.d = flip(get_corner(self.x, self.y), 
                              true, 
                              direct(self.d, 
                                     get_corner(self.x, self.y), 
                                     true),
                              0);
            } else {
                self.d = flip(get_wall(self.x, self.y), 
                              false, 
                              direct(self.d, 
                                     get_wall(self.x, self.y), 
                                     false), 
                              self.d);
            }
        }
        self.x += d_fac(self.d, true);
        self.y += d_fac(self.d, false);
    }
}