use inline_colorization::*;
use crate::modules::utils::Utils;

// CODE

#[allow(non_upper_case_globals)]
pub(crate) const color_gray: &str = "\x1B[90m";

static mut IS_DEBUG:Option<bool> = Some(false);
static mut IS_DEBUG_DETAILED:Option<bool> = Some(false);

pub struct Terminal {}
impl Terminal {
    pub fn set_debug_status(is_debug:bool, is_debug_detailed:bool) {
        unsafe {
            IS_DEBUG = Some(is_debug);
            IS_DEBUG_DETAILED = Some(is_debug_detailed);
        }

        Terminal::debug_detailed("[Terminal] set_debug_status();");
    }

    pub fn debug(text:&str) {
        if unsafe { IS_DEBUG.unwrap() } {
            let _time = Utils::get_local_time(true);
            println!("{color_gray}[{_time}] {color_cyan}[DEBUG] {color_reset}{text}");
        }
    }

    pub fn debug_detailed(text:&str) {
        if unsafe { IS_DEBUG_DETAILED.unwrap() } {
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
        println!("{color_gray}[{_time}] {color_yellow}[LOG] {color_reset}{text}");
    }
}