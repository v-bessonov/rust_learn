#![allow(dead_code, unused_imports)]

use std::fmt::{write, Debug, Formatter, Result};

#[derive(Debug)]
pub struct Coord {
    long: f32,
    lat: f32
}

impl Coord {
    pub fn new(long: f32, lat: f32) -> Coord {
        Coord {long, lat}
    }
}

/*impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Coord with custom debug format: [{}, {}]", self.lat, self.long)
    }
}*/

/*impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("Coordinate structure ")
            .field("Longitude", &self.long)
            .field("Latitude", &self.lat)
            .finish()
    }
}*/