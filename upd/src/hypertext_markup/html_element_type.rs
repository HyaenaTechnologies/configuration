#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::primitive::u8;

// Hypertext Markup Language Element Type Definition
pub type HTMLElementType = u8;

// Hypertext Markup Language Element Type
pub const ESCAPABLE_RAW_TEXT_ELEMENT: HTMLElementType = 0;
pub const FOREIGN_ELEMENT: HTMLElementType = 1;
pub const NORMAL_ELEMENT: HTMLElementType = 2;
pub const RAW_TEXT_ELEMENT: HTMLElementType = 3;
pub const TEMPLATE_ELEMENT: HTMLElementType = 4;
pub const VOID_ELEMENT: HTMLElementType = 5;
