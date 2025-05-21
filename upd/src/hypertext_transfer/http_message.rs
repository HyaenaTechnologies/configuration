use std::{
    io::{BufRead, BufReader, Error, StdoutLock, Write, stdout},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    option::Option,
    primitive::str,
    result::{
        Result,
        Result::{Err, Ok},
    },
    string::String,
    vec::Vec,
};

use super::{
    http_headers::HTTPHeader,
    http_methods::HTTPMethod,
    http_security_directives::HTTPSecurityDirective,
    http_status_codes::{HTTPStatusCode, HTTPStatusText},
};

// Hypertext Transfer Protocol Body Definition
pub type HTTPBody<T> = T;

// Hypertext Transfer Protocol Version Definition
pub type HTTPVersion = &'static str;

// Hypertext Transfer Protocol Message Definition
pub struct HypertextTransferMessage<T> {
    pub body: Option<HTTPBody<T>>,
    pub headers: Vec<HTTPHeader>,
    pub method: HTTPMethod,
    pub security_directives: Option<Vec<HTTPSecurityDirective>>,
    pub status_code: HTTPStatusCode,
    pub status_text: HTTPStatusText,
    pub version: HTTPVersion,
}

// Hypertext Transfer Protocol Connection Management
pub fn http_connection_management(mut transmission_stream: TcpStream) -> () {
    let mut standard_output: StdoutLock = stdout().lock();
    let buffered_reader: BufReader<&TcpStream> = BufReader::new(&transmission_stream);
    let hypertext_transfer_request: Vec<String> = buffered_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    writeln!(
        standard_output,
        "Hypertext Tranfer Protocol Request: {:#?}",
        hypertext_transfer_request
    )
    .unwrap();
    writeln!(transmission_stream, "HTTP/1.1 200 OK").unwrap();

    return ();
}

// Hypertext Transer Protocol Client
pub fn http_client() -> () {
    return ();
}

// Hypertext Transfer Protocol Server
pub fn http_server(transmission_listener: TcpListener) -> () {
    transmission_listener.set_ttl(100).unwrap();

    for transmission_stream in transmission_listener.incoming() {
        let stream: TcpStream = transmission_stream.unwrap();

        stream.set_ttl(100).unwrap();
    }

    return ();
}
