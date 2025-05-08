#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{primitive::str, vec::Vec};

// Hypertext Markup Language Element Attribute Definition
pub type HTMLAttribute = &'static str;

// Hypertext Markup Language Element Attribute
pub const HTML_ACCEPT_ATTRIBUTE: HTMLAttribute = "accept";
pub const HTML_AUTO_COMPLETE_ATTRIBUTE: HTMLAttribute = "autocomplete";
pub const HTML_CAPTURE_ATTRIBUTE: HTMLAttribute = "capture";
pub const HTML_CROSS_ORIGIN_ATTRIBUTE: HTMLAttribute = "crossorigin";
pub const HTML_DIRECTORY_NAME_ATTRIBUTE: HTMLAttribute = "dirname";
pub const HTML_DISABLED_ATTRIBUTE: HTMLAttribute = "disabled";
pub const HTML_ELEMENT_TIMING_ATTRIBUTE: HTMLAttribute = "elementtiming";
pub const HTML_FOR_ATTRIBUTE: HTMLAttribute = "for";
pub const HTML_MAXIMUM_ATTRIBUTE: HTMLAttribute = "max";
pub const HTML_MAXIMUM_LENGTH_ATTRIBUTE: HTMLAttribute = "maxlength";
pub const HTML_MINIMUM_ATTRIBUTE: HTMLAttribute = "min";
pub const HTML_MINIMUM_LENGTH_ATTRIBUTE: HTMLAttribute = "minlength";
pub const HTML_MULTIPLE_ATTRIBUTE: HTMLAttribute = "multiple";
pub const HTML_PATTERN_ATTRIBUTE: HTMLAttribute = "pattern";
pub const HTML_PLACEHOLDER_ATTRIBUTE: HTMLAttribute = "placeholder";
pub const HTML_READ_ONLY_ATTRIBUTE: HTMLAttribute = "readonly";
pub const HTML_RELATION_ATTRIBUTE: HTMLAttribute = "rel";
pub const HTML_REQUIRED_ATTRIBUTE: HTMLAttribute = "required";
pub const HTML_SIZE_ATTRIBUTE: HTMLAttribute = "size";
pub const HTML_STEP_ATTRIBUTE: HTMLAttribute = "step";

// Hypertext Markup Language Attribute Vector
pub fn html_attributes() -> Vec<HTMLAttribute> {
    let hypertext_markup_attributes: Vec<HTMLAttribute> = Vec::from([
        HTML_ACCEPT_ATTRIBUTE,
        HTML_AUTO_COMPLETE_ATTRIBUTE,
        HTML_CAPTURE_ATTRIBUTE,
        HTML_CROSS_ORIGIN_ATTRIBUTE,
        HTML_DIRECTORY_NAME_ATTRIBUTE,
        HTML_DISABLED_ATTRIBUTE,
        HTML_ELEMENT_TIMING_ATTRIBUTE,
        HTML_FOR_ATTRIBUTE,
        HTML_MAXIMUM_ATTRIBUTE,
        HTML_MAXIMUM_LENGTH_ATTRIBUTE,
        HTML_MINIMUM_ATTRIBUTE,
        HTML_MINIMUM_LENGTH_ATTRIBUTE,
        HTML_MULTIPLE_ATTRIBUTE,
        HTML_PATTERN_ATTRIBUTE,
        HTML_PLACEHOLDER_ATTRIBUTE,
        HTML_READ_ONLY_ATTRIBUTE,
        HTML_RELATION_ATTRIBUTE,
        HTML_REQUIRED_ATTRIBUTE,
        HTML_SIZE_ATTRIBUTE,
        HTML_STEP_ATTRIBUTE,
    ]);

    return hypertext_markup_attributes;
}
