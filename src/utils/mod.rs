pub mod terminal;
pub mod config;

use chrono::prelude::*;

// CODE

pub struct Utils {}
impl Utils {
    pub fn digit_format(number: u32) -> String {
        format!("{:02}", number)
    }

    pub fn get_local_time(is_seconds:bool) -> String {
        let _time = Local::now();

        let mut _time_str = String::new();
        _time_str.push_str(Utils::digit_format(_time.hour()).to_string().as_str());
        _time_str.push_str(":");
        _time_str.push_str(Utils::digit_format(_time.minute()).to_string().as_str());

        if is_seconds {
            _time_str.push_str(":");
            _time_str.push_str(Utils::digit_format(_time.second()).to_string().as_str());
        }

        _time_str
    }

    pub fn get_normalize_ip(ip:&str) -> String {
        let _local_result = ip.replace("::ffff:", "");
        _local_result
    }
}