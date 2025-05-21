#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{collections::HashMap, primitive::str};

// Hypertext Transfer Protocol Status Code Definition
pub type HTTPStatusCode = &'static str;

// Hypertext Transfer Protocol Status Text Definition
pub type HTTPStatusText = &'static str;

// Hypertext Transfer Protocol Status Codes
pub const HTTP_ONE_HUNDRED: HTTPStatusCode = "100";
pub const HTTP_ONE_HUNDRED_ONE: HTTPStatusCode = "101";
pub const HTTP_ONE_HUNDRED_TWO: HTTPStatusCode = "102";
pub const HTTP_ONE_HUNDRED_THREE: HTTPStatusCode = "103";
pub const HTTP_TWO_HUNDRED: HTTPStatusCode = "200";
pub const HTTP_TWO_HUNDRED_ONE: HTTPStatusCode = "201";
pub const HTTP_TWO_HUNDRED_TWO: HTTPStatusCode = "202";
pub const HTTP_TWO_HUNDRED_THREE: HTTPStatusCode = "203";
pub const HTTP_TWO_HUNDRED_FOUR: HTTPStatusCode = "204";
pub const HTTP_TWO_HUNDRED_FIVE: HTTPStatusCode = "205";
pub const HTTP_TWO_HUNDRED_SIX: HTTPStatusCode = "206";
pub const HTTP_TWO_HUNDRED_SEVEN: HTTPStatusCode = "207";
pub const HTTP_TWO_HUNDRED_EIGHT: HTTPStatusCode = "208";
pub const HTTP_TWO_HUNDRED_TWENTY_SIX: HTTPStatusCode = "226";
pub const HTTP_THREE_HUNDRED: HTTPStatusCode = "300";
pub const HTTP_THREE_HUNDRED_ONE: HTTPStatusCode = "301";
pub const HTTP_THREE_HUNDRED_TWO: HTTPStatusCode = "302";
pub const HTTP_THREE_HUNDRED_THREE: HTTPStatusCode = "303";
pub const HTTP_THREE_HUNDRED_FOUR: HTTPStatusCode = "304";
pub const HTTP_THREE_HUNDRED_SEVEN: HTTPStatusCode = "307";
pub const HTTP_THREE_HUNDRED_EIGHT: HTTPStatusCode = "308";
pub const HTTP_FOUR_HUNDRED: HTTPStatusCode = "400";
pub const HTTP_FOUR_HUNDRED_ONE: HTTPStatusCode = "401";
pub const HTTP_FOUR_HUNDRED_TWO: HTTPStatusCode = "402";
pub const HTTP_FOUR_HUNDRED_THREE: HTTPStatusCode = "403";
pub const HTTP_FOUR_HUNDRED_FOUR: HTTPStatusCode = "404";
pub const HTTP_FOUR_HUNDRED_FIVE: HTTPStatusCode = "405";
pub const HTTP_FOUR_HUNDRED_SIX: HTTPStatusCode = "406";
pub const HTTP_FOUR_HUNDRED_SEVEN: HTTPStatusCode = "407";
pub const HTTP_FOUR_HUNDRED_EIGHT: HTTPStatusCode = "408";
pub const HTTP_FOUR_HUNDRED_NINE: HTTPStatusCode = "409";
pub const HTTP_FOUR_HUNDRED_TEN: HTTPStatusCode = "410";
pub const HTTP_FOUR_HUNDRED_ELEVEN: HTTPStatusCode = "411";
pub const HTTP_FOUR_HUNDRED_TWELVE: HTTPStatusCode = "412";
pub const HTTP_FOUR_HUNDRED_THIRTEEN: HTTPStatusCode = "413";
pub const HTTP_FOUR_HUNDRED_FOURTEEN: HTTPStatusCode = "414";
pub const HTTP_FOUR_HUNDRED_FIFTEEN: HTTPStatusCode = "415";
pub const HTTP_FOUR_HUNDRED_SIXTEEN: HTTPStatusCode = "416";
pub const HTTP_FOUR_HUNDRED_SEVENTEEN: HTTPStatusCode = "417";
pub const HTTP_FOUR_HUNDRED_EIGHTEEN: HTTPStatusCode = "418";
pub const HTTP_FOUR_HUNDRED_TWENTY_ONE: HTTPStatusCode = "421";
pub const HTTP_FOUR_HUNDRED_TWENTY_TWO: HTTPStatusCode = "422";
pub const HTTP_FOUR_HUNDRED_TWENTY_THREE: HTTPStatusCode = "423";
pub const HTTP_FOUR_HUNDRED_TWENTY_FOUR: HTTPStatusCode = "424";
pub const HTTP_FOUR_HUNDRED_TWENTY_FIVE: HTTPStatusCode = "425";
pub const HTTP_FOUR_HUNDRED_TWENTY_SIX: HTTPStatusCode = "426";
pub const HTTP_FOUR_HUNDRED_TWENTY_EIGHT: HTTPStatusCode = "428";
pub const HTTP_FOUR_HUNDRED_TWENTY_NINE: HTTPStatusCode = "429";
pub const HTTP_FOUR_HUNDRED_THIRTY_ONE: HTTPStatusCode = "431";
pub const HTTP_FOUR_HUNDRED_FIFTEY_ONE: HTTPStatusCode = "451";
pub const HTTP_FIVE_HUNDRED: HTTPStatusCode = "500";
pub const HTTP_FIVE_HUNDRED_ONE: HTTPStatusCode = "501";
pub const HTTP_FIVE_HUNDRED_TWO: HTTPStatusCode = "502";
pub const HTTP_FIVE_HUNDRED_THREE: HTTPStatusCode = "503";
pub const HTTP_FIVE_HUNDRED_FOUR: HTTPStatusCode = "504";
pub const HTTP_FIVE_HUNDRED_FIVE: HTTPStatusCode = "505";
pub const HTTP_FIVE_HUNDRED_SIX: HTTPStatusCode = "506";
pub const HTTP_FIVE_HUNDRED_SEVEN: HTTPStatusCode = "507";
pub const HTTP_FIVE_HUNDRED_EIGHT: HTTPStatusCode = "508";
pub const HTTP_FIVE_HUNDRED_TEN: HTTPStatusCode = "510";
pub const HTTP_FIVE_HUNDRED_ELEVEN: HTTPStatusCode = "511";

// Hypertext Transfer Protocol Status Texts
pub const HTTP_CONTINUE: HTTPStatusText = "Continue";
pub const HTTP_SWITCHING_PROTOCOLS: HTTPStatusText = "Switching Protocols";
pub const HTTP_PROCESSING: HTTPStatusText = "Processing";
pub const HTTP_EARLY_HINTS: HTTPStatusText = "Early Hints";
pub const HTTP_OK: HTTPStatusText = "OK";
pub const HTTP_CREATED: HTTPStatusText = "Created";
pub const HTTP_ACCEPTED: HTTPStatusText = "Accepted";
pub const HTTP_NON_AUTHORITATIVE_INFORMATION: HTTPStatusText = "Non-Authoritative Information";
pub const HTTP_NO_CONTENT: HTTPStatusText = "No Content";
pub const HTTP_RESET_CONTENT: HTTPStatusText = "Reset Content";
pub const HTTP_PARTIAL_CONTENT: HTTPStatusText = "Partial Content";
pub const HTTP_MULTI_STATUS: HTTPStatusText = "Multi-Status";
pub const HTTP_ALREADY_REPORTED: HTTPStatusText = "Already Reported";
pub const HTTP_IM_USED: HTTPStatusText = "IM Used";
pub const HTTP_MULTIPLE_CHOICES: HTTPStatusText = "Multiple Choices";
pub const HTTP_MOVED_PERMANENTLY: HTTPStatusText = "Moved Permanently";
pub const HTTP_FOUND: HTTPStatusText = "Found";
pub const HTTP_SEE_OTHER: HTTPStatusText = "See Other";
pub const HTTP_NOT_MODIFIED: HTTPStatusText = "Not Modified";
pub const HTTP_TEMPORARY_REDIRECT: HTTPStatusText = "Temporary Redirect";
pub const HTTP_PREMANENT_REDIRECT: HTTPStatusText = "Permanent Redirect";
pub const HTTP_BAD_REQUEST: HTTPStatusText = "Bad Request";
pub const HTTP_UNAUTHORIZED: HTTPStatusText = "Unauthorized";
pub const HTTP_PAYMENT_REQUIRED: HTTPStatusText = "Payment Required";
pub const HTTP_FORBIDDEN: HTTPStatusText = "Forbidden";
pub const HTTP_NOT_FOUND: HTTPStatusText = "Not Found";
pub const HTTP_METHOD_NOT_ALLOWED: HTTPStatusText = "Method Not Allowed";
pub const HTTP_NOT_ACCEPTABLE: HTTPStatusText = "Not Acceptable";
pub const HTTP_PROXY_AUTHENTICATION_REQUIRED: HTTPStatusText = "Proxy Authentication Required";
pub const HTTP_REQUEST_TIMEOUT: HTTPStatusText = "Request Timeout";
pub const HTTP_CONFLICT: HTTPStatusText = "Conflict";
pub const HTTP_GONE: HTTPStatusText = "Gone";
pub const HTTP_LENGTH_REQUIRED: HTTPStatusText = "Length Required";
pub const HTTP_PRECONDITION_FAILED: HTTPStatusText = "Precondition Failed";
pub const HTTP_CONTENT_TOO_LARGE: HTTPStatusText = "Content Too Large";
pub const HTTP_URI_TOO_LONG: HTTPStatusText = "URI Too Long";
pub const HTTP_UNSUPPORTED_MEDIA_TYPE: HTTPStatusText = "Unsupported Media Type";
pub const HTTP_RANGE_NOT_SATISFIABLE: HTTPStatusText = "Range Not Satisfiable";
pub const HTTP_EXPECTATION_FAILED: HTTPStatusText = "Expectation Failed";
pub const HTTP_TEAPOT: HTTPStatusText = "I'm a teapot";
pub const HTTP_MISDIRECTED_REQUEST: HTTPStatusText = "Misdirected Request";
pub const HTTP_UNPROCESSABLE_CONTENT: HTTPStatusText = "Unprocessable Content";
pub const HTTP_LOCKED: HTTPStatusText = "Locked";
pub const HTTP_FAILED_DEPENDENCY: HTTPStatusText = "Failed Dependency";
pub const HTTP_TOO_EARLY: HTTPStatusText = "Too Early";
pub const HTTP_UPGRADE_REQUIRED: HTTPStatusText = "Upgrade Required";
pub const HTTP_PRECONDITION_REQUIRED: HTTPStatusText = "Precondition Required";
pub const HTTP_TOO_MANY_REQUESTS: HTTPStatusText = "Too Many Requests";
pub const HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE: HTTPStatusText = "Request Header Fields Too Large";
pub const HTTP_UNAVAILABLE_FOR_LEGAL_REASONS: HTTPStatusText = "Unavailable For Legal Reasons";
pub const HTTP_INTERNAL_SERVER_ERROR: HTTPStatusText = "Internal Server Error";
pub const HTTP_NOT_IMPLEMENTED: HTTPStatusText = "Not Implemented";
pub const HTTP_BAD_GATEWAY: HTTPStatusText = "Bad Gateway";
pub const HTTP_SERVICE_UNAVAILABLE: HTTPStatusText = "Service Unavailable";
pub const HTTP_GATEWAY_TIMEOUT: HTTPStatusText = "Gateway Timeout";
pub const HTTP_VERSION_NOT_SUPPORTED: HTTPStatusText = "HTTP Version Not Supported";
pub const HTTP_VARIANT_ALSO_NEGOTIATES: HTTPStatusText = "Variant Also Negotiates";
pub const HTTP_INSUFFICENT_STORAGE: HTTPStatusText = "Insufficient Storage";
pub const HTTP_LOOP_DETECTED: HTTPStatusText = "Loop Detected";
pub const HTTP_NOT_EXTENDED: HTTPStatusText = "Not Extended";
pub const HTTP_NETWORK_AUTHENTICATION_REQUIRED: HTTPStatusText = "Network Authentication Required";

// Hypertext Transfer Protocol Status Code and Status Text Hash Map
pub fn http_status_codes() -> HashMap<HTTPStatusCode, HTTPStatusText> {
    let hypertext_transfer_status_codes: HashMap<HTTPStatusCode, HTTPStatusText> = HashMap::from([
        (HTTP_ONE_HUNDRED, HTTP_CONTINUE),
        (HTTP_ONE_HUNDRED_ONE, HTTP_SWITCHING_PROTOCOLS),
        (HTTP_ONE_HUNDRED_TWO, HTTP_PROCESSING),
        (HTTP_ONE_HUNDRED_THREE, HTTP_EARLY_HINTS),
        (HTTP_TWO_HUNDRED, HTTP_OK),
        (HTTP_TWO_HUNDRED_ONE, HTTP_CREATED),
        (HTTP_TWO_HUNDRED_TWO, HTTP_ACCEPTED),
        (HTTP_TWO_HUNDRED_THREE, HTTP_NON_AUTHORITATIVE_INFORMATION),
        (HTTP_TWO_HUNDRED_FOUR, HTTP_NO_CONTENT),
        (HTTP_TWO_HUNDRED_FIVE, HTTP_RESET_CONTENT),
        (HTTP_TWO_HUNDRED_SIX, HTTP_PARTIAL_CONTENT),
        (HTTP_TWO_HUNDRED_SEVEN, HTTP_MULTI_STATUS),
        (HTTP_TWO_HUNDRED_EIGHT, HTTP_ALREADY_REPORTED),
        (HTTP_TWO_HUNDRED_TWENTY_SIX, HTTP_IM_USED),
        (HTTP_THREE_HUNDRED, HTTP_MULTIPLE_CHOICES),
        (HTTP_THREE_HUNDRED_ONE, HTTP_MOVED_PERMANENTLY),
        (HTTP_THREE_HUNDRED_TWO, HTTP_FOUND),
        (HTTP_THREE_HUNDRED_THREE, HTTP_SEE_OTHER),
        (HTTP_THREE_HUNDRED_FOUR, HTTP_NOT_MODIFIED),
        (HTTP_THREE_HUNDRED_SEVEN, HTTP_TEMPORARY_REDIRECT),
        (HTTP_THREE_HUNDRED_EIGHT, HTTP_PREMANENT_REDIRECT),
        (HTTP_FOUR_HUNDRED, HTTP_BAD_REQUEST),
        (HTTP_FOUR_HUNDRED_ONE, HTTP_UNAUTHORIZED),
        (HTTP_FOUR_HUNDRED_TWO, HTTP_PAYMENT_REQUIRED),
        (HTTP_FOUR_HUNDRED_THREE, HTTP_FORBIDDEN),
        (HTTP_FOUR_HUNDRED_FOUR, HTTP_NOT_FOUND),
        (HTTP_FOUR_HUNDRED_FIVE, HTTP_METHOD_NOT_ALLOWED),
        (HTTP_FOUR_HUNDRED_SIX, HTTP_NOT_ACCEPTABLE),
        (HTTP_FOUR_HUNDRED_SEVEN, HTTP_PROXY_AUTHENTICATION_REQUIRED),
        (HTTP_FOUR_HUNDRED_EIGHT, HTTP_REQUEST_TIMEOUT),
        (HTTP_FOUR_HUNDRED_NINE, HTTP_CONFLICT),
        (HTTP_FOUR_HUNDRED_TEN, HTTP_GONE),
        (HTTP_FOUR_HUNDRED_ELEVEN, HTTP_LENGTH_REQUIRED),
        (HTTP_FOUR_HUNDRED_TWELVE, HTTP_PRECONDITION_FAILED),
        (HTTP_FOUR_HUNDRED_THIRTEEN, HTTP_CONTENT_TOO_LARGE),
        (HTTP_FOUR_HUNDRED_FOURTEEN, HTTP_URI_TOO_LONG),
        (HTTP_FOUR_HUNDRED_FIFTEEN, HTTP_UNSUPPORTED_MEDIA_TYPE),
        (HTTP_FOUR_HUNDRED_SIXTEEN, HTTP_RANGE_NOT_SATISFIABLE),
        (HTTP_FOUR_HUNDRED_SEVENTEEN, HTTP_EXPECTATION_FAILED),
        (HTTP_FOUR_HUNDRED_EIGHTEEN, HTTP_TEAPOT),
        (HTTP_FOUR_HUNDRED_TWENTY_ONE, HTTP_MISDIRECTED_REQUEST),
        (HTTP_FOUR_HUNDRED_TWENTY_TWO, HTTP_UNPROCESSABLE_CONTENT),
        (HTTP_FOUR_HUNDRED_TWENTY_THREE, HTTP_LOCKED),
        (HTTP_FOUR_HUNDRED_TWENTY_FOUR, HTTP_FAILED_DEPENDENCY),
        (HTTP_FOUR_HUNDRED_TWENTY_FIVE, HTTP_TOO_EARLY),
        (HTTP_FOUR_HUNDRED_TWENTY_SIX, HTTP_UPGRADE_REQUIRED),
        (HTTP_FOUR_HUNDRED_TWENTY_EIGHT, HTTP_PRECONDITION_REQUIRED),
        (HTTP_FOUR_HUNDRED_TWENTY_NINE, HTTP_TOO_MANY_REQUESTS),
        (
            HTTP_FOUR_HUNDRED_THIRTY_ONE,
            HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE,
        ),
        (
            HTTP_FOUR_HUNDRED_FIFTEY_ONE,
            HTTP_UNAVAILABLE_FOR_LEGAL_REASONS,
        ),
        (HTTP_FIVE_HUNDRED, HTTP_INTERNAL_SERVER_ERROR),
        (HTTP_FIVE_HUNDRED_ONE, HTTP_NOT_IMPLEMENTED),
        (HTTP_FIVE_HUNDRED_TWO, HTTP_BAD_GATEWAY),
        (HTTP_FIVE_HUNDRED_THREE, HTTP_SERVICE_UNAVAILABLE),
        (HTTP_FIVE_HUNDRED_FOUR, HTTP_GATEWAY_TIMEOUT),
        (HTTP_FIVE_HUNDRED_FIVE, HTTP_VERSION_NOT_SUPPORTED),
        (HTTP_FIVE_HUNDRED_SIX, HTTP_VARIANT_ALSO_NEGOTIATES),
        (HTTP_FIVE_HUNDRED_SEVEN, HTTP_INSUFFICENT_STORAGE),
        (HTTP_FIVE_HUNDRED_EIGHT, HTTP_LOOP_DETECTED),
        (HTTP_FIVE_HUNDRED_TEN, HTTP_NOT_EXTENDED),
        (
            HTTP_FIVE_HUNDRED_ELEVEN,
            HTTP_NETWORK_AUTHENTICATION_REQUIRED,
        ),
    ]);

    return hypertext_transfer_status_codes;
}
