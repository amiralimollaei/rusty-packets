use std::time::{SystemTime, UNIX_EPOCH};

use crate::utils::ansi::{string::AnsiString, AnsiColor, ColorMode};

pub static LOGGER: Logger<'static> = Logger {
    name: "main",
    level: 3,
};

pub fn get_logger() -> &'static Logger<'static> {
    &LOGGER
}

pub fn get_log_level() -> u8 {
    LOGGER.level
}

pub struct Logger<'a> {
    pub name: &'a str,
    pub level: u8,
}

impl<'a> Logger<'a> {
    #[inline]
    fn format_time() -> String {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        let hours = (duration.as_secs() / 3600) % 24;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    pub fn info<T: Into<AnsiString>>(&self, msg: T) {
        let message_astr = msg.into().with_default_foreground(AnsiColor(255, 255, 255));
        // TODO: handle the case when the message has more than one line
        let message_lines = message_astr.delimiter("\n");
        for line in message_lines {
            let astr = AnsiString::new_fore(
                format!("[{} - {}/INFO]: ", Self::format_time(), self.name).as_str(),
                Some((127, 127, 127)),
            ) + line;
            println!("{}", astr.to_string(&ColorMode::Limited));
        }
    }

    pub fn warn<T: Into<AnsiString>>(&self, msg: T) {
        if self.level < 1 {
            return;
        }
        let message_astr = msg.into().with_default_foreground(AnsiColor(255, 255, 0));
        let message_lines = message_astr.delimiter("\n");
        for line in message_lines {
            let astr = AnsiString::new_fore(
                format!("[{} - {}/WARN]: ", Self::format_time(), self.name).as_str(),
                Some((255, 255, 127)),
            ) + line;
            println!("{}", astr.to_string(&ColorMode::Limited));
        }
    }

    pub fn error<T: Into<AnsiString>>(&self, msg: T) {
        if self.level < 2 {
            return;
        }
        let message_astr = msg.into().with_default_foreground(AnsiColor(255, 0, 0));
        let message_lines = message_astr.delimiter("\n");
        for line in message_lines {
            let astr = AnsiString::new_fore(
                format!("[{} - {}/ERROR]: ", Self::format_time(), self.name).as_str(),
                Some((255, 127, 127)),
            ) + line;
            println!("{}", astr.to_string(&ColorMode::Limited));
        }
    }

    pub fn debug<T: Into<AnsiString>>(&self, msg: T) {
        if self.level < 3 {
            return;
        }
        let message_astr = msg.into().with_default_foreground(AnsiColor(0, 255, 255));
        let message_lines = message_astr.delimiter("\n");
        for line in message_lines {
            let astr = AnsiString::new_fore(
                format!("[{} - {}/DEBUG]: ", Self::format_time(), self.name).as_str(),
                Some((127, 255, 255)),
            ) + line;
            println!("{}", astr.to_string(&ColorMode::Limited));
        }
    }
}
