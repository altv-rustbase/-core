pub mod terminal;
pub mod config;

use std::str::FromStr;

use chrono::prelude::*;

// CODE

pub struct Utils {}
impl Utils {
    pub fn digit_format(number: u32) -> String {
        format!("{:02}", number)
    }

    pub fn get_local_time(is_seconds:bool) -> String {
        let _time = Local::now();

        let _local_hour = Utils::digit_format(_time.hour());
        let _local_minute = Utils::digit_format(_time.minute());
        let mut _time_str = format!("{_local_hour}:{_local_minute}");

        if is_seconds {
            let _local_sec = Utils::digit_format(_time.second());
            _time_str.push_str(format!(":{_local_sec}").as_str());
        }

        _time_str
    }

    pub fn get_normalize_ip(ip:&str) -> String {
        let _local_result = ip.replace("::ffff:", "");
        _local_result
    }


    pub fn convert_to<T>(value:String) -> Result<T, <T as FromStr>::Err> where T:FromStr {
        value.parse::<T>()
    }

}