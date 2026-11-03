use std::time::SystemTime; // use std::time::Duration
//use std::thread::sleep;

use crate::science::astrological::celestial::EARTH;
use crate::science::physical::constants::SPEED_OF_LIGHT_IN_VACUUM;

pub fn physical_time(_distance: f64) -> f64 {
    let c: f64 = SPEED_OF_LIGHT_IN_VACUUM.value;

    let distance: f64 = EARTH.velocity_m;

    let time: f64 = distance / c;

    time
}

pub fn system_time() -> SystemTime {
    let system_time_now = SystemTime::now();

    system_time_now
}

pub const SECONDS: u64 = 60;
pub const MINUTES: u64 = 60;
pub const HOURS: u64 = 24;

pub const SECONDS_IN_A_DAY: u64 = 86_400;

pub const DAYS_PER_WEEK: u64 = 7;
pub const DAYS_PER_YEAR: u64 = 365;

pub fn seconds_per_day(seconds: u64) -> u64 {
    let seconds_per_day = SECONDS * MINUTES * HOURS;
    
    let time = seconds / seconds_per_day;

    time
}

pub fn seconds_since_unix_epoc() {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            //println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs());

            let string = n.as_secs().to_string();

            let parse = string.parse::<u64>().expect("Not a valid number");

            let time = seconds_per_day(parse) / DAYS_PER_YEAR;

            

            println!("{}", time);
        }
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub const DAYS: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
pub const MONTHS: [&str; 12] = ["January", "Febuary", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
