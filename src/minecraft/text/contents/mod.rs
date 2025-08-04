use super::Component;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct TextContent {
    // Set as the content directly, with no additional processing. (Formatting codes still apply.)
    text: String,
}

// The translation text may contain any number of the placeholder tokens %s, %n$s (where n is a one-based index) and %%.
// Each %s is replaced by the element of with corresponding to the placeholder's index among all %s's (excluding %n$s) in the string.
// Each %n$s is replaced by the nth element of with.
// Each %% is replaced by a literal % symbol.
// Any out-of-bounds or otherwise invalid placeholders are simply left intact in the output.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct TranslatableContent {
    // A translation key, looked up in the current language file to obtain a translation text,
    // becomes the component's content after processing.
    // If the translation key is unrecognized, it itself becomes the translation text,
    // which then has placeholder replacement performed on it as usual.
    translate: String,
    // Array of Text Component. Replacements for placeholders in the translation text.
    with: Option<Vec<Component>>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct KeybindContent {
    // The name of a keybinding. The client's current setting for the specified keybinding becomes the component's content.
    keybind: String,
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct ScoreboardData {
    // A player username, player or entity UUID, entity selector (that selects one entity),
    // or * to match the sending player.
    name: String,
    // The name of the objective.
    objective: String,
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct ScoreContent {
    // Object. Contains ScoreboardData
    score: ScoreboardData,
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct SelectorContent {
    //  An entity selector.
    selector: String,
    // Separator to place between results. If omitted, defaults to {"color":"gray","text":", "}.
    separator: Option<Box<Component>>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct NBTContent {
    // NBT path to be queried.
    nbt: String,
    // If true, the server will attempt to parse and resolve each result as (an NBT string containing)
    // a text component for display. If omitted, defaults to false.
    interpret: Option<bool>,
    // Separator to place between results. If omitted, defaults to {"text":", "}.
    separator: Option<Box<Component>>,
    // Location of a block entity to be queried, in the usual space-separated coordinate syntax
    // with support for ~ and ^.
    block: String,
    // Selector specifying the set of entities to be queried.
    entity: String,
    // Identifier specifying the storage to be queried.
    storage: String,
}
