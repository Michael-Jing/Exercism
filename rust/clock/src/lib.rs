use std::cmp::Eq;
use std::fmt;
#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let m = (hours * 60 + minutes).rem_euclid(24 * 60);
        Clock { minutes: m }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let m = (self.minutes + minutes).rem_euclid(24 * 60);
        Clock { minutes: m }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (hours, minutes) = (self.minutes.div_euclid(60), self.minutes.rem_euclid(60));
        write!(f, "{:0>2}:{:0>2}", hours, minutes)
    }
}
