#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{primitive::str, vec::Vec};

use super::html_element::HypertextMarkupElement;

// Hypertext Markup Language Document Type Definition
pub type HTMLDocumentType = &'static str;

// Hypertext Markup Language Document Type
pub const HTML_DOCUMENT_TYPE: HTMLDocumentType = "<!DOCTYPE HTML>";

// Hypertext Markup Language Document Definition
pub struct HypertextMarkupDocument {
    pub elements: Vec<HypertextMarkupElement>,
    pub document_type: HTMLDocumentType,
}
