use super::parse_interface::ParseSource;

pub struct RequestBody {}

pub struct ResponseBody {}

pub enum ShioriProtocol {
    Request(RequestBody),
    Response(ResponseBody),
    ParseError,
}

pub fn parse(source: &dyn ParseSource) -> ShioriProtocol {
    let nl_string = source.next_line();
    let nl = nl_string.as_str();

    return match nl {
        "req" => ShioriProtocol::Request(RequestBody {}),
        "res" => ShioriProtocol::Response(ResponseBody {}),
        _ => ShioriProtocol::ParseError,
    };
}
