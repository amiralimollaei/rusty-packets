static mut LOGLEVEL: u8 = 0;

pub fn get_log_level() -> u8 {
    unsafe {LOGLEVEL}
}

pub fn set_log_level(lvl: u8) {
    unsafe {LOGLEVEL = lvl}
}