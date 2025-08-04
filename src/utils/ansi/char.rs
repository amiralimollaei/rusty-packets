use std::ops::Add;

use super::string::AnsiString;
use super::{AnsiColor, AnsiGraphics, ColorGround, ColorMode};

#[derive(Clone)]
pub struct AnsiChar {
    pub char: char,
    pub back_color: Option<AnsiColor>,
    pub fore_color: Option<AnsiColor>,
    pub graphics: AnsiGraphics,
}

impl AnsiChar {
    #[inline]
    pub fn new(char: char, fore: Option<(u8, u8, u8)>, back: Option<(u8, u8, u8)>) -> AnsiChar {
        Self {
            char: char,
            back_color: match back {
                Some(back) => Some(AnsiColor(back.0, back.1, back.2)),
                None => None,
            },
            fore_color: match fore {
                Some(fore) => Some(AnsiColor(fore.0, fore.1, fore.2)),
                None => None,
            },
            graphics: AnsiGraphics { modes: Vec::new() },
        }
    }

    pub fn to_string(&self, mode: &ColorMode) -> String {
        let mut pre = String::new();
        pre.push_str(self.graphics.to_string(false).as_str());
        match &self.back_color {
            Some(back) => pre += back.to_string(mode, &ColorGround::Back).as_str(),
            None => {}
        };
        match &self.fore_color {
            Some(fore) => pre += fore.to_string(mode, &ColorGround::Fore).as_str(),
            None => {}
        };

        let mut post = String::new();
        post += self.graphics.to_string(false).as_str();

        format!("{}{}{}", pre, self.char, post)
    }

    pub fn setchar(&mut self, c: char) {
        self.char = c;
    }

    pub fn as_ansistring(&self) -> AnsiString {
        AnsiString {
            vec: [self.clone()].to_vec(),
        }
    }
}

impl Add for AnsiChar {
    type Output = AnsiString;

    fn add(self, other: Self) -> Self::Output {
        AnsiString {
            vec: [self, other].to_vec(),
        }
    }
}
