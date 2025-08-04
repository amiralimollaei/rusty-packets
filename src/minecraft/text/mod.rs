mod contents;

use contents::{
    KeybindContent, NBTContent, ScoreContent, SelectorContent, TextContent, TranslatableContent,
};
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Component {
    text: Option<TextContent>,
    translate: Option<TranslatableContent>,
    score: Option<ScoreContent>,
    selector: Option<SelectorContent>,
    keybind: Option<KeybindContent>,
    nbt: Option<NBTContent>,
    extra: Option<Box<Component>>,
}
