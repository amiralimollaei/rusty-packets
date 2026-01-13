pub mod ansi;
pub mod logging;

use std::io::{self, Read};

use ansi::{AnsiColor, AnsiGraphicMode, AnsiGraphics, string::AnsiString};
use serde_json::{Map, Value as JsonValue};

#[inline]
pub fn read_bytes<const N: usize>(stream: &mut impl std::io::Read) -> [u8; N] {
    let mut bytes: [u8; N] = [0; N];
    stream.read_exact(&mut bytes).expect("ReadError");
    bytes
}

pub fn read_n_bytes<S: Read, U: Into<usize>>(reader: &mut S, n: U) -> Result<Vec<u8>, io::Error> {
    // Create a buffer to store the data.
    let n = n.into();
    let mut buffer = Vec::with_capacity(n);

    // Attempt to read 'n' bytes into the buffer.
    match reader.take(n as u64).read_to_end(&mut buffer) {
        Ok(bytes_read) => {
            // If we read fewer than 'n' bytes, return an error.
            if bytes_read < n {
                Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Read fewer bytes than requested"))
            } else {
                Ok(buffer)  // Return the buffer containing 'n' bytes.
            }
        }
        Err(e) => Err(e), // Propagate any IO error.
    }
}

pub fn parse_color(value: &Map<String, JsonValue>) -> Option<(u8, u8, u8)> {
    let c = value.get("color");
    match c {
        None => None,
        Some(color) => match color.as_str() {
            None => None,
            Some(s) => match s {
                "black" => Some((0, 0, 0)),
                "dark_blue" => Some((0, 0, 170)),
                "dark_green" => Some((0, 170, 0)),
                "dark_aqua" => Some((0, 170, 170)),
                "dark_red" => Some((170, 0, 0)),
                "dark_purple" => Some((170, 0, 170)),
                "gold" => Some((255, 170, 0)),
                "gray" => Some((170, 170, 170)),
                "dark_gray" => Some((85, 85, 85)),
                "blue" => Some((85, 85, 255)),
                "green" => Some((85, 255, 85)),
                "aqua" => Some((85, 255, 255)),
                "red" => Some((255, 85, 85)),
                "light_purple" => Some((255, 85, 255)),
                "yellow" => Some((255, 255, 85)),
                "white" => Some((255, 255, 255)),

                _ => {
                    println!("WARNING: color {} is not implemented!", s);
                    None
                }
            },
        },
    }
}

pub fn parse_style(value: &Map<String, JsonValue>) -> (Vec<AnsiGraphicMode>, Vec<AnsiGraphicMode>) {
    fn apply(
        add_agraphics: &mut Vec<AnsiGraphicMode>,
        remove_agraphics: &mut Vec<AnsiGraphicMode>,
        value: &Map<String, JsonValue>,
        style: &str,
        agm: AnsiGraphicMode,
    ) {
        let c = value.get(style);
        match c {
            None => {}
            Some(v) => {
                if v.as_bool() == Some(true) {
                    add_agraphics.push(agm);
                } else {
                    remove_agraphics.push(agm);
                }
            }
        }
    }

    let mut add_agraphics: Vec<AnsiGraphicMode> = Vec::new();
    let mut remove_agraphics: Vec<AnsiGraphicMode> = Vec::new();
    apply(
        &mut add_agraphics,
        &mut remove_agraphics,
        value,
        "obfuscated",
        AnsiGraphicMode::Blinking,
    );
    apply(
        &mut add_agraphics,
        &mut remove_agraphics,
        value,
        "bold",
        AnsiGraphicMode::Bold,
    );
    apply(
        &mut add_agraphics,
        &mut remove_agraphics,
        value,
        "strikethrough",
        AnsiGraphicMode::Strike,
    );
    apply(
        &mut add_agraphics,
        &mut remove_agraphics,
        value,
        "underline",
        AnsiGraphicMode::Underline,
    );
    apply(
        &mut add_agraphics,
        &mut remove_agraphics,
        value,
        "italic",
        AnsiGraphicMode::Italic,
    );

    (add_agraphics, remove_agraphics)
}

pub fn parse_text(value: &Map<String, JsonValue>) -> Option<AnsiString> {
    let text = value.get("text");
    match text {
        None => None,
        Some(_text) => match _text.as_str() {
            None => None,
            Some(t) => Some(AnsiString::new_colorless(t)),
        },
    }
}

fn ansicolor(color: (u8, u8, u8)) -> AnsiColor {
    AnsiColor::new(color.0, color.1, color.2)
}

pub fn parce_text_component_json(
    value: &JsonValue,
    parentcolor: Option<(u8, u8, u8)>,
    parentstyle: Option<AnsiGraphics>,
) -> AnsiString {
    //println!("TextComponent: {}", value);
    if let Some(v) = value.as_object() {
        let mut result = AnsiString::empty();
        // get text as AnsiString if exists
        let text = parse_text(v);
        // new color
        let mut newcolor = None;
        match parentcolor {
            Some(pc) => {
                newcolor = Some(pc);
            }
            None => {}
        }
        // overwrite color if exists
        match parse_color(v) {
            Some(c) => {
                newcolor = Some(c);
            }
            None => {}
        }
        // get style overwrites
        let stylechanges = parse_style(v);
        // new style
        let mut newstyle = AnsiGraphics::new();
        match parentstyle {
            Some(ps) => {
                newstyle = ps.clone();
            }
            None => {}
        }
        // add new styles
        for agm in stylechanges.0 {
            newstyle.add(agm)
        }
        // remove parent styles
        for agm in stylechanges.1 {
            newstyle.remove(agm)
        }
        match text {
            Some(t) => {
                let mut astr = t.clone();
                astr.set_graphics(newstyle.clone());
                match newcolor {
                    Some(c) => {
                        astr.set_foreground(ansicolor(c));
                    }
                    None => {}
                }

                result = result + astr;
            }
            None => {}
        }

        let extra = v.get("extra");
        match extra {
            None => {}
            Some(array) => match array.as_array() {
                None => {}
                Some(val) => {
                    for obj in val {
                        let objstring = match obj {
                            JsonValue::String(s) => AnsiString::new_colorless(s),
                            _ => parce_text_component_json(obj, newcolor, Some(newstyle.clone())),
                        };
                        result = result + objstring;
                    }
                }
            },
        }

        result
    } else {
        AnsiString::empty()
    }
}
