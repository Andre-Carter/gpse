use std::time::SystemTime;
//use std::time::Duration
//use std::time::{ SystemTime, Duration };
//use std::thread::sleep;

//use crate::science::astrological::celestial::EARTH;
//use crate::science::physical::constants::SPEED_OF_LIGHT_IN_VACUUM;

pub const SECONDS_PER_MINUTE: u64 = 60;
pub const MINUTES_PER_HOUR: u64 = 60;
pub const HOURS_PER_DAY: u64 = 24;

pub const SECONDS_PER_DAY: u64 = 86_400;

pub const DAYS_PER_WEEK: u64 = 7;
pub const DAYS_PER_YEAR: u64 = 365;

pub const DAYS: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];
pub const MONTHS: [&str; 12] = [
    "January",
    "Febuary",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

// do something with this

/*

pub fn physical_time(_distance: f64) -> f64 {
    let c: f64 = SPEED_OF_LIGHT_IN_VACUUM.value;

    let distance: f64 = EARTH.velocity_m;

    let time: f64 = distance / c;

    time
}

*/

// work in progress: time

pub fn system_time() -> SystemTime {
    SystemTime::now()
}

pub fn seconds_per_hour() -> u64 {
    SECONDS_PER_MINUTE * MINUTES_PER_HOUR
}

pub fn seconds_per_day() -> u64 {
    SECONDS_PER_MINUTE * MINUTES_PER_HOUR * HOURS_PER_DAY
}

pub fn seconds_per_year() -> u64 {
    SECONDS_PER_MINUTE * MINUTES_PER_HOUR * HOURS_PER_DAY * DAYS_PER_YEAR
}

pub fn minutes_since(seconds: u64) -> u64 {
    seconds / SECONDS_PER_MINUTE
}

pub fn hours_since(seconds: u64) -> u64 {
    let seconds_per_hour = seconds_per_hour();

    seconds / seconds_per_hour
}

pub fn days_since(seconds: u64) -> u64 {
    let seconds_per_day = seconds_per_day();

    seconds / seconds_per_day
}

pub fn years_since(seconds: u64) -> u64 {
    let seconds_per_year = seconds_per_year();

    seconds / seconds_per_year
}

pub fn seconds_since_unix_epoc() {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            //println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs());

            let string = n.as_secs().to_string();

            let parse = string.parse::<u64>().expect("Not a valid number");

            println!("Seconds since unix epoch: {}", parse);

            let minutes_since = minutes_since(parse);

            println!("{}", minutes_since);

            let hours_since = hours_since(parse);

            println!("{}", hours_since);

            let days_since = days_since(parse);

            println!("{}", days_since);

            let years_since = years_since(parse);

            println!("{}", years_since);
        }
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

/*
pub fn tick() {
    let sys_time = SystemTime::now();
    let new_sys_time = SystemTime::now();
    let difference = new_sys_time.duration_since(sys_time)
        .expect("Clock may have gone backwards");
    println!("{difference:?}");
}
*/
