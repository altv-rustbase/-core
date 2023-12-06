use std::cell::RefCell;

use inline_colorization::*;
use crate::modules::utils::Utils;

// CODE

#[allow(non_upper_case_globals)]
pub(crate) const color_gray: &str = "\x1B[90m";

thread_local! {
    pub static IS_DEBUG:RefCell<bool> = RefCell::new(false);
    pub static IS_DEBUG_DETAILED:RefCell<bool> = RefCell::new(false);
}

pub struct Terminal {}
impl Terminal {
    pub fn set_debug_status(is_debug:bool, is_debug_detailed:bool) {
        IS_DEBUG.set(is_debug);
        IS_DEBUG_DETAILED.set(is_debug_detailed);

        Terminal::debug_detailed("[Terminal] set_debug_status();");
    }

    pub fn debug(text:&str) {
        if IS_DEBUG.take() {
            let _time = Utils::get_local_time(true);
            println!("{color_gray}[{_time}] {color_cyan}[DEBUG] {color_reset}{text}");
        }
    }

    pub fn debug_detailed(text:&str) {
        if IS_DEBUG_DETAILED.take() {
            let _time = Utils::get_local_time(true);
            println!("{color_gray}[{_time}] {color_gray}[DEBUG] {color_gray}{text}");
        }
    }

    pub fn done(text:&str) {
        let _time = Utils::get_local_time(true);
        println!("{color_gray}[{_time}] {color_green}[DONE] {color_reset}{text}");
    }

    pub fn log(text:&str) {
        let _time = Utils::get_local_time(true);
        println!("{color_gray}[{_time}] {color_reset}[LOG] {text}");
    }

    pub fn info(text:&str) {
        let _time = Utils::get_local_time(true);
        println!("{color_gray}[{_time}] {color_yellow}[INFO] {color_reset}{text}");
    }

    pub fn error(text:&str) {
        let _time = Utils::get_local_time(true);
        println!("{color_gray}[{_time}] {color_red}[ERROR] {color_reset}{text}");
    }
}