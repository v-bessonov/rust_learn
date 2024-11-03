#![allow(dead_code, unused_imports)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Flight {
    origin: String,
    destination: String
}

impl Flight {
    pub fn new(origin: &str, destination: &str) -> Flight {
        Flight{origin: origin.to_string(), destination: destination.to_string()}
    }

    pub fn redirect_to(&mut self, destination: &str) {
        self.destination = destination.to_string()
    }
}

/*impl Clone for Flight {
    fn clone(&self) -> Flight {
        println!("Custom cloning of Flight from {} to {}", self.origin, self.destination);
        Flight{
            origin: self.origin.clone(),
            destination: self.destination.clone()
        }
    }
}*/