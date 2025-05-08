#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{primitive::str, vec::Vec};

// Hypertext Transfer Protocol Header Definition
pub type HTTPHeader = &'static str;

// Hypertext Transfer Protocol Headers
pub const HTTP_ACCEPT: HTTPHeader = "Accept";
pub const HTTP_ACCEPT_CLIENT_HINT: HTTPHeader = "Accept-CH";
pub const HTTP_ACCEPT_ENCODING: HTTPHeader = "Accept-Encoding";
pub const HTTP_ACCEPT_LANGUAGE: HTTPHeader = "Accept-Language";
pub const HTTP_ACCEPT_PATCH: HTTPHeader = "Accept-Patch";
pub const HTTP_ACCEPT_POST: HTTPHeader = "Accept-Post";
pub const HTTP_ACCEPT_RANGES: HTTPHeader = "Accept-Ranges";
pub const HTTP_ACCESS_CONTROL_ALLOW_CREDENTIALS: HTTPHeader = "Access-Control-Allow-Credentials";
pub const HTTP_ACCESS_CONTROL_ALLOW_HEADERS: HTTPHeader = "Access-Control-Allow-Headers";
pub const HTTP_ACCESS_CONTROL_ALLOW_METHODS: HTTPHeader = "Access-Control-Allow-Methods";
pub const HTTP_ACCESS_CONTROL_ALLOW_ORIGIN: HTTPHeader = "Access-Control-Allow-Origin";
pub const HTTP_ACCESS_CONTROL_EXPOSE_HEADERS: HTTPHeader = "Access-Control-Expose-Headers";
pub const HTTP_ACCESS_CONTROL_MAX_AGE: HTTPHeader = "Access-Control-Max-Age";
pub const HTTP_ACCESS_CONTROL_REQUEST_HEADERS: HTTPHeader = "Access-Control-Request-Headers";
pub const HTTP_ACCESS_CONTROL_REQUEST_METHOD: HTTPHeader = "Access-Control-Request-Method";
pub const HTTP_AGE: HTTPHeader = "Age";
pub const HTTP_ALLOW: HTTPHeader = "Allow";
pub const HTTP_ALTERNATIVE_SERVICE: HTTPHeader = "Alt-Svc";
pub const HTTP_ALTERNATIVE_USED: HTTPHeader = "Alt-Used";
pub const HTTP_AUTHORIZATION: HTTPHeader = "Authorization";
pub const HTTP_CACHE_CONTROL: HTTPHeader = "Cache-Control";
pub const HTTP_CLEAR_SITE_DATA: HTTPHeader = "Clear-Site-Data";
pub const HTTP_CONNECTION: HTTPHeader = "Connection";
pub const HTTP_CONTENT_DIGEST: HTTPHeader = "Content-Digest";
pub const HTTP_CONTENT_DISPOSITION: HTTPHeader = "Content-Disposition";
pub const HTTP_CONTENT_ENCODING: HTTPHeader = "Content-Encoding";
pub const HTTP_CONTENT_LANGUAGE: HTTPHeader = "Content-Language";
pub const HTTP_CONTENT_LENGTH: HTTPHeader = "Content-Length";
pub const HTTP_CONTENT_LOCATION: HTTPHeader = "Content-Location";
pub const HTTP_CONTENT_RANGE: HTTPHeader = "Content-Range";
pub const HTTP_CONTENT_SECURITY_POLICY: HTTPHeader = "Content-Security-Policy";
pub const HTTP_CONTENT_SECURITY_POLICY_REPORT_ONLY: HTTPHeader =
    "Content-Security-Policy-Report-Only";
pub const HTTP_CONTENT_TYPE: HTTPHeader = "Content-Type";
pub const HTTP_COOKIE: HTTPHeader = "Cookie";
pub const HTTP_CROSS_ORIGIN_EMBEDDER_POLICY: HTTPHeader = "Cross-Origin-Embedder-Policy";
pub const HTTP_CROSS_ORIGIN_OPENER_POLICY: HTTPHeader = "Cross-Origin-Opener-Policy";
pub const HTTP_CROSS_ORIGIN_RESOURCE_POLICY: HTTPHeader = "Cross-Origin-Resource-Policy";
pub const HTTP_DATE: HTTPHeader = "Date";
pub const HTTP_DEVICE_MEMORY: HTTPHeader = "Device-Memory";
pub const HTTP_ENTITY_TAG: HTTPHeader = "ETag";
pub const HTTP_EXPECT: HTTPHeader = "Expect";
pub const HTTP_EXPIRES: HTTPHeader = "Expires";
pub const HTTP_FORWARDED: HTTPHeader = "Forwarded";
pub const HTTP_FROM: HTTPHeader = "From";
pub const HTTP_HOST: HTTPHeader = "Host";
pub const HTTP_IF_MATCH: HTTPHeader = "If-Match";
pub const HTTP_IF_MODIFIED_SINCE: HTTPHeader = "If-Modified-Since";
pub const HTTP_IF_NONE_MATCH: HTTPHeader = "If-None-Match";
pub const HTTP_IF_RANGE: HTTPHeader = "If-Range";
pub const HTTP_IF_UNMODIFIED_SINCE: HTTPHeader = "If-Unmodified-Since";
pub const HTTP_KEEP_ALIVE: HTTPHeader = "Keep-Alive";
pub const HTTP_LAST_MODIFIED: HTTPHeader = "Last-Modified";
pub const HTTP_LINK: HTTPHeader = "Link";
pub const HTTP_LOCATION: HTTPHeader = "Location";
pub const HTTP_MAXIMUM_FORWARDS: HTTPHeader = "Max-Forwards";
pub const HTTP_ORIGIN: HTTPHeader = "Origin";
pub const HTTP_PRIORITY: HTTPHeader = "Priority";
pub const HTTP_PROXY_AUTHENTICATION: HTTPHeader = "Proxy-Authenticate";
pub const HTTP_PROXY_AUTHORIZATION: HTTPHeader = "Proxy-Authorization";
pub const HTTP_RANGE: HTTPHeader = "Range";
pub const HTTP_REFERER: HTTPHeader = "Referer";
pub const HTTP_REFERRER_POLICY: HTTPHeader = "Referrer-Policy";
pub const HTTP_REFRESH: HTTPHeader = "Refresh";
pub const HTTP_REPR_DIGEST: HTTPHeader = "Repr-Digest";
pub const HTTP_RETRY_AFTER: HTTPHeader = "Retry-After";
pub const HTTP_SECURE_FETCH_DESTINATION: HTTPHeader = "Sec-Fetch-Dest";
pub const HTTP_SECURE_FETCH_MODE: HTTPHeader = "Sec-Fetch-Mode";
pub const HTTP_SECURE_FETCH_SITE: HTTPHeader = "Sec-Fetch-Site";
pub const HTTP_SECURE_FETCH_USER: HTTPHeader = "Sec-Fetch-User";
pub const HTTP_SECURE_PURPOSE: HTTPHeader = "Sec-Purpose";
pub const HTTP_SECURE_WEBSOCKET_ACCEPT: HTTPHeader = "Sec-WebSocket-Accept";
pub const HTTP_SECURE_WEBSOCKET_EXTENTIONS: HTTPHeader = "Sec-WebSocket-Extensions";
pub const HTTP_SECURE_WEBSOCKET_KEY: HTTPHeader = "Sec-WebSocket-Key";
pub const HTTP_SECURE_WEBSOCKET_PROTOCOL: HTTPHeader = "Sec-WebSocket-Protocol";
pub const HTTP_SECURE_WEBSOCKET_VERSION: HTTPHeader = "Sec-WebSocket-Version";
pub const HTTP_SERVER: HTTPHeader = "Server";
pub const HTTP_SERVER_TIMING: HTTPHeader = "Server-Timing";
pub const HTTP_SERVICE_WORKER: HTTPHeader = "Service-Worker";
pub const HTTP_SERVICE_WORKER_ALLOWED: HTTPHeader = "Service-Worker-Allowed";
pub const HTTP_SERVICE_WORKER_NAVIGATION_PRELOAD: HTTPHeader = "Service-Worker-Navigation-Preload";
pub const HTTP_SET_COOKIE: HTTPHeader = "Set-Cookie";
pub const HTTP_SOURCEMAP: HTTPHeader = "SourceMap";
pub const HTTP_STRICT_TRANSPORT_SECURITY: HTTPHeader = "Strict-Transport-Security";
pub const HTTP_TRANSFER_ENCODING_TYPE: HTTPHeader = "TE";
pub const HTTP_TIMING_ALLOW_ORIGIN: HTTPHeader = "Timing-Allow-Origin";
pub const HTTP_TRAILER: HTTPHeader = "Trailer";
pub const HTTP_TRANSFER_ENCODING: HTTPHeader = "Transfer-Encoding";
pub const HTTP_UPGRADE: HTTPHeader = "Upgrade";
pub const HTTP_UPGRADE_INSECURE_REQUESTS: HTTPHeader = "Upgrade-Insecure-Requests";
pub const HTTP_USER_AGENT: HTTPHeader = "User-Agent";
pub const HTTP_VARY: HTTPHeader = "Vary";
pub const HTTP_VIA_PROXY: HTTPHeader = "Via";
pub const HTTP_WANT_CONTENT_DIGEST: HTTPHeader = "Want-Content-Digest";
pub const HTTP_WANT_REPRESENTATION_DIGEST: HTTPHeader = "Want-Repr-Digest";
pub const HTTP_WWW_AUTHENTICATE: HTTPHeader = "WWW-Authenticate";
pub const HTTP_X_CONTENT_TYPE_OPTIONS: HTTPHeader = "X-Content-Type-Options";
pub const HTTP_X_FRAME_OPTIONS: HTTPHeader = "X-Frame-Options";

// Hypertext Transfer Protocol Header Vector
pub fn http_headers() -> Vec<HTTPHeader> {
    let hypertext_transfer_headers: Vec<HTTPHeader> = Vec::from([
        HTTP_ACCEPT,
        HTTP_ACCEPT_CLIENT_HINT,
        HTTP_ACCEPT_ENCODING,
        HTTP_ACCEPT_LANGUAGE,
        HTTP_ACCEPT_PATCH,
        HTTP_ACCEPT_POST,
        HTTP_ACCEPT_RANGES,
        HTTP_ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HTTP_ACCESS_CONTROL_ALLOW_HEADERS,
        HTTP_ACCESS_CONTROL_ALLOW_METHODS,
        HTTP_ACCESS_CONTROL_ALLOW_ORIGIN,
        HTTP_ACCESS_CONTROL_EXPOSE_HEADERS,
        HTTP_ACCESS_CONTROL_MAX_AGE,
        HTTP_ACCESS_CONTROL_REQUEST_HEADERS,
        HTTP_ACCESS_CONTROL_REQUEST_METHOD,
        HTTP_AGE,
        HTTP_ALLOW,
        HTTP_ALTERNATIVE_SERVICE,
        HTTP_ALTERNATIVE_USED,
        HTTP_AUTHORIZATION,
        HTTP_CACHE_CONTROL,
        HTTP_CLEAR_SITE_DATA,
        HTTP_CONNECTION,
        HTTP_CONTENT_DIGEST,
        HTTP_CONTENT_DISPOSITION,
        HTTP_CONTENT_ENCODING,
        HTTP_CONTENT_LANGUAGE,
        HTTP_CONTENT_LENGTH,
        HTTP_CONTENT_LOCATION,
        HTTP_CONTENT_RANGE,
        HTTP_CONTENT_SECURITY_POLICY,
        HTTP_CONTENT_SECURITY_POLICY_REPORT_ONLY,
        HTTP_CONTENT_TYPE,
        HTTP_COOKIE,
        HTTP_CROSS_ORIGIN_EMBEDDER_POLICY,
        HTTP_CROSS_ORIGIN_OPENER_POLICY,
        HTTP_CROSS_ORIGIN_RESOURCE_POLICY,
        HTTP_DATE,
        HTTP_DEVICE_MEMORY,
        HTTP_ENTITY_TAG,
        HTTP_EXPECT,
        HTTP_EXPIRES,
        HTTP_FORWARDED,
        HTTP_FROM,
        HTTP_HOST,
        HTTP_IF_MATCH,
        HTTP_IF_MODIFIED_SINCE,
        HTTP_IF_NONE_MATCH,
        HTTP_IF_RANGE,
        HTTP_IF_UNMODIFIED_SINCE,
        HTTP_KEEP_ALIVE,
        HTTP_LAST_MODIFIED,
        HTTP_LINK,
        HTTP_LOCATION,
        HTTP_MAXIMUM_FORWARDS,
        HTTP_ORIGIN,
        HTTP_PRIORITY,
        HTTP_PROXY_AUTHENTICATION,
        HTTP_PROXY_AUTHORIZATION,
        HTTP_RANGE,
        HTTP_REFERER,
        HTTP_REFERRER_POLICY,
        HTTP_REFRESH,
        HTTP_REPR_DIGEST,
        HTTP_RETRY_AFTER,
        HTTP_SECURE_FETCH_DESTINATION,
        HTTP_SECURE_FETCH_MODE,
        HTTP_SECURE_FETCH_SITE,
        HTTP_SECURE_FETCH_USER,
        HTTP_SECURE_PURPOSE,
        HTTP_SECURE_WEBSOCKET_ACCEPT,
        HTTP_SECURE_WEBSOCKET_EXTENTIONS,
        HTTP_SECURE_WEBSOCKET_KEY,
        HTTP_SECURE_WEBSOCKET_PROTOCOL,
        HTTP_SECURE_WEBSOCKET_VERSION,
        HTTP_SERVER,
        HTTP_SERVER_TIMING,
        HTTP_SERVICE_WORKER,
        HTTP_SERVICE_WORKER_ALLOWED,
        HTTP_SERVICE_WORKER_NAVIGATION_PRELOAD,
        HTTP_SET_COOKIE,
        HTTP_SOURCEMAP,
        HTTP_STRICT_TRANSPORT_SECURITY,
        HTTP_TRANSFER_ENCODING_TYPE,
        HTTP_TIMING_ALLOW_ORIGIN,
        HTTP_TRAILER,
        HTTP_TRANSFER_ENCODING,
        HTTP_UPGRADE,
        HTTP_UPGRADE_INSECURE_REQUESTS,
        HTTP_USER_AGENT,
        HTTP_VARY,
        HTTP_VIA_PROXY,
        HTTP_WANT_CONTENT_DIGEST,
        HTTP_WANT_REPRESENTATION_DIGEST,
        HTTP_WWW_AUTHENTICATE,
        HTTP_X_CONTENT_TYPE_OPTIONS,
        HTTP_X_FRAME_OPTIONS,
    ]);

    return hypertext_transfer_headers;
}
