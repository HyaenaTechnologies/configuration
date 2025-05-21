#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{primitive::str, vec::Vec};

// Hypertext Transfer Protocol Content Security Policy Directive Definition
pub type HTTPSecurityDirective = &'static str;

// Hypertext Transfer Protocol Content Security Policy Directives
pub const HTTP_BASE_URI: HTTPSecurityDirective = "base-uri";
pub const HTTP_CHILD_SOURCE: HTTPSecurityDirective = "child-src";
pub const HTTP_CONNECT_SOURCE: HTTPSecurityDirective = "connect-src";
pub const HTTP_DEFAULT_SOURCE: HTTPSecurityDirective = "default-src";
pub const HTTP_FONT_SOURCE: HTTPSecurityDirective = "font-src";
pub const HTTP_FORM_ACTION: HTTPSecurityDirective = "form-action";
pub const HTTP_FRAME_ANCESTORS: HTTPSecurityDirective = "frame-ancestors";
pub const HTTP_FRAME_SOURCE: HTTPSecurityDirective = "frame-src";
pub const HTTP_IMAGE_SOURCE: HTTPSecurityDirective = "img-src";
pub const HTTP_MANIFEST_SOURCE: HTTPSecurityDirective = "manifest-src";
pub const HTTP_MEDIA_SOURCE: HTTPSecurityDirective = "media-src";
pub const HTTP_OBJECT_SOURCE: HTTPSecurityDirective = "object-src";
pub const HTTP_REPORT_TO: HTTPSecurityDirective = "report-to";
pub const HTTP_REQUIRE_TRUSTED_TYPES_FOR: HTTPSecurityDirective = "require-trusted-types-for";
pub const HTTP_SANDBOX: HTTPSecurityDirective = "sandbox";
pub const HTTP_SCRIPT_SOURCE: HTTPSecurityDirective = "script-src";
pub const HTTP_SCRIPT_SOURCE_ATTRIBUTE: HTTPSecurityDirective = "script-src-attr";
pub const HTTP_SCRIPT_SOURCE_ELEMENT: HTTPSecurityDirective = "script-src-elem";
pub const HTTP_STYLE_SOURCE: HTTPSecurityDirective = "style-src";
pub const HTTP_STYLE_SOURCE_ATTRIBUTE: HTTPSecurityDirective = "style-src-attr";
pub const HTTP_STYLE_SOURCE_ELEMENT: HTTPSecurityDirective = "style-src-elem";
pub const HTTP_TRUSTED_TYPES: HTTPSecurityDirective = "trusted-types";
pub const HTTP_UPGRADE_INSECURE_REQUESTS: HTTPSecurityDirective = "upgrade-insecure-requests";
pub const HTTP_WORKER_SOURCE: HTTPSecurityDirective = "worker-src";

// Hypertext Transfer Protocol Content Security Policy Directive Vector
pub fn http_security_directives() -> Vec<HTTPSecurityDirective> {
    let hypertext_transfer_security_directives: Vec<HTTPSecurityDirective> = Vec::from([
        HTTP_BASE_URI,
        HTTP_CHILD_SOURCE,
        HTTP_CONNECT_SOURCE,
        HTTP_DEFAULT_SOURCE,
        HTTP_FONT_SOURCE,
        HTTP_FORM_ACTION,
        HTTP_FRAME_ANCESTORS,
        HTTP_FRAME_SOURCE,
        HTTP_IMAGE_SOURCE,
        HTTP_MANIFEST_SOURCE,
        HTTP_MEDIA_SOURCE,
        HTTP_OBJECT_SOURCE,
        HTTP_REPORT_TO,
        HTTP_REQUIRE_TRUSTED_TYPES_FOR,
        HTTP_SANDBOX,
        HTTP_SCRIPT_SOURCE,
        HTTP_SCRIPT_SOURCE_ATTRIBUTE,
        HTTP_SCRIPT_SOURCE_ELEMENT,
        HTTP_STYLE_SOURCE,
        HTTP_STYLE_SOURCE_ATTRIBUTE,
        HTTP_STYLE_SOURCE_ELEMENT,
        HTTP_TRUSTED_TYPES,
        HTTP_UPGRADE_INSECURE_REQUESTS,
        HTTP_WORKER_SOURCE,
    ]);

    return hypertext_transfer_security_directives;
}
