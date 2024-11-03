use chrono::Utc;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {

    pub fn print(&self) {
        println!("In print(), point is [{},{}]", self.x, self.y);
    }
    pub fn print_v1(&self) {
        println!("In print_v1(), point is [{},{}]", self.x, self.y);
    }

    pub fn print_v2(self: &Point) {
        println!("In print_v2(), point is [{},{}]", self.x, self.y);
    }

    pub fn print_v3(self: &Self) {
        println!("In print_v3(), point is [{},{}]", self.x, self.y);
    }

    pub fn to_string(&self) -> String {
        format!("[{},{}]", self.x, self.y)
    }

    pub fn reset(&mut self) {
        self.log("About to reset the point");
        self.x = 0;
        self.y = 0;
    }

    pub fn reset_v1(&mut self) {
        self.x = 0;
        self.y = 0;
    }

    pub fn reset_v2(self : &mut Point) {
        self.x = 0;
        self.y = 0;
    }

    pub fn reset_v3(self : &mut Self) {
        self.x = 0;
        self.y = 0;
    }

    pub fn move_by(&mut self, dx : i32, dy : i32) {
        self.log("About to move the point");
        self.x += dx;
        self.y += dy;
    }

    fn log(&self, msg : &str) {
        let ts = Utc::now().format("%T");
        println!("{} {} at {}", msg, self.to_string(), ts);
    }
}