#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{primitive::str, vec::Vec};

// Hypertext Markup Language Element Global Attribute Definition
pub type HTMLGlobalAttribute = &'static str;

// Hypertext Markup Language Element Global Attribute
pub const HTML_GLOBAL_ACCESS_KEY_ATTRIBUTE: HTMLGlobalAttribute = "accesskey";
pub const HTML_GLOBAL_AUTO_CAPITALIZE_ATTRIBUTE: HTMLGlobalAttribute = "autocapitalize";
pub const HTML_GLOBAL_AUTO_CORRECT_ATTRIBUTE: HTMLGlobalAttribute = "autocorrect";
pub const HTML_GLOBAL_AUTO_FOCUS_ATTRIBUTE: HTMLGlobalAttribute = "autofocus";
pub const HTML_GLOBAL_CLASS_ATTRIBUTE: HTMLGlobalAttribute = "class";
pub const HTML_GLOBAL_CONTENT_EDITABLE_ATTRIBUTE: HTMLGlobalAttribute = "contenteditable";
pub const HTML_GLOBAL_DATA_ATTRIBUTE: HTMLGlobalAttribute = "data-";
pub const HTML_GLOBAL_DIRECTORY_ATTRIBUTE: HTMLGlobalAttribute = "dir";
pub const HTML_GLOBAL_DRAGGABLE_ATTRIBUTE: HTMLGlobalAttribute = "draggable";
pub const HTML_GLOBAL_ENTER_KEY_HINT_ATTRIBUTE: HTMLGlobalAttribute = "enterkeyhint";
pub const HTML_GLOBAL_EXPORT_PARTS_ATTRIBUTE: HTMLGlobalAttribute = "exportparts";
pub const HTML_GLOBAL_HIDDEN_ATTRIBUTE: HTMLGlobalAttribute = "hidden";
pub const HTML_GLOBAL_IDENTIFICATION_ATTRIBUTE: HTMLGlobalAttribute = "id";
pub const HTML_GLOBAL_INERT_ATTRIBUTE: HTMLGlobalAttribute = "inert";
pub const HTML_GLOBAL_INPUT_MODE_ATTRIBUTE: HTMLGlobalAttribute = "inputmode";
pub const HTML_GLOBAL_IS_ATTRIBUTE: HTMLGlobalAttribute = "is";
pub const HTML_GLOBAL_ITEM_IDENTIFICATION_ATTRIBUTE: HTMLGlobalAttribute = "itemid";
pub const HTML_GLOBAL_ITEM_PROPERTY_ATTRIBUTE: HTMLGlobalAttribute = "itemprop";
pub const HTML_GLOBAL_ITEM_REFERENCE_ATTRIBUTE: HTMLGlobalAttribute = "itemref";
pub const HTML_GLOBAL_ITEM_SCOPE_ATTRIBUTE: HTMLGlobalAttribute = "itemscope";
pub const HTML_GLOBAL_ITEM_TYPE_ATTRIBUTE: HTMLGlobalAttribute = "itemtype";
pub const HTML_GLOBAL_LANGUAGE_ATTRIBUTE: HTMLGlobalAttribute = "lang";
pub const HTML_GLOBAL_NUMBER_USED_ONCE_ATTRIBUTE: HTMLGlobalAttribute = "nonce";
pub const HTML_GLOBAL_PART_ATTRIBUTE: HTMLGlobalAttribute = "part";
pub const HTML_GLOBAL_POPOVER_ATTRIBUTE: HTMLGlobalAttribute = "popover";
pub const HTML_GLOBAL_SLOT_ATTRIBUTE: HTMLGlobalAttribute = "slot";
pub const HTML_GLOBAL_SPELL_CHECK_ATTRIBUTE: HTMLGlobalAttribute = "spellcheck";
pub const HTML_GLOBAL_STYLE_ATTRIBUTE: HTMLGlobalAttribute = "style";
pub const HTML_GLOBAL_TAB_INDEX_ATTRIBUTE: HTMLGlobalAttribute = "tabindex";
pub const HTML_GLOBAL_TITLE_ATTRIBUTE: HTMLGlobalAttribute = "title";
pub const HTML_GLOBAL_TRANSLATE_ATTRIBUTE: HTMLGlobalAttribute = "translate";
pub const HTML_GLOBAL_WRITING_SUGGESTIONS_ATTRIBUTE: HTMLGlobalAttribute = "writingsuggestions";

// Hypertext Markup Language Global Attribute Vector
pub fn html_global_attributes() -> Vec<HTMLGlobalAttribute> {
    let hypertext_markup_global_attributes: Vec<HTMLGlobalAttribute> = Vec::from([
        HTML_GLOBAL_ACCESS_KEY_ATTRIBUTE,
        HTML_GLOBAL_AUTO_CAPITALIZE_ATTRIBUTE,
        HTML_GLOBAL_AUTO_CORRECT_ATTRIBUTE,
        HTML_GLOBAL_AUTO_FOCUS_ATTRIBUTE,
        HTML_GLOBAL_CLASS_ATTRIBUTE,
        HTML_GLOBAL_CONTENT_EDITABLE_ATTRIBUTE,
        HTML_GLOBAL_DATA_ATTRIBUTE,
        HTML_GLOBAL_DIRECTORY_ATTRIBUTE,
        HTML_GLOBAL_DRAGGABLE_ATTRIBUTE,
        HTML_GLOBAL_ENTER_KEY_HINT_ATTRIBUTE,
        HTML_GLOBAL_EXPORT_PARTS_ATTRIBUTE,
        HTML_GLOBAL_HIDDEN_ATTRIBUTE,
        HTML_GLOBAL_IDENTIFICATION_ATTRIBUTE,
        HTML_GLOBAL_INERT_ATTRIBUTE,
        HTML_GLOBAL_INPUT_MODE_ATTRIBUTE,
        HTML_GLOBAL_IS_ATTRIBUTE,
        HTML_GLOBAL_ITEM_IDENTIFICATION_ATTRIBUTE,
        HTML_GLOBAL_ITEM_PROPERTY_ATTRIBUTE,
        HTML_GLOBAL_ITEM_REFERENCE_ATTRIBUTE,
        HTML_GLOBAL_ITEM_SCOPE_ATTRIBUTE,
        HTML_GLOBAL_ITEM_TYPE_ATTRIBUTE,
        HTML_GLOBAL_LANGUAGE_ATTRIBUTE,
        HTML_GLOBAL_NUMBER_USED_ONCE_ATTRIBUTE,
        HTML_GLOBAL_PART_ATTRIBUTE,
        HTML_GLOBAL_POPOVER_ATTRIBUTE,
        HTML_GLOBAL_SLOT_ATTRIBUTE,
        HTML_GLOBAL_SPELL_CHECK_ATTRIBUTE,
        HTML_GLOBAL_STYLE_ATTRIBUTE,
        HTML_GLOBAL_TAB_INDEX_ATTRIBUTE,
        HTML_GLOBAL_TITLE_ATTRIBUTE,
        HTML_GLOBAL_TRANSLATE_ATTRIBUTE,
        HTML_GLOBAL_WRITING_SUGGESTIONS_ATTRIBUTE,
    ]);

    return hypertext_markup_global_attributes;
}
