#![allow(dead_code, unused_imports)]
#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
pub struct Currency{
    pub dollars: i32,
    pub cents: i32
}

impl Currency {
    pub fn new(dollars: i32, cents: i32) -> Currency {
        Currency{dollars, cents}
    }
}

/*impl Copy for Currency {

}

impl Clone for Currency {
    fn clone(&self) -> Currency {
        println!("Custom cloning of currency {}.{}", self.dollars, self.cents);
        *self
    }
}*/