use super::{consts, parse_interface::ParseSource, request};

pub struct ResponseBody {}
pub struct RequestBody {
    pub method: consts::RequestMethod,
    pub charset: String,
    pub sender: String,
    pub sender_type: consts::SenderType,
    pub security_level: consts::SecurityLevel,
    pub status: Vec<consts::GhostExecuteStatus>,
}

pub enum ShioriProtocol {
    Request(RequestBody),   /* Request */
    Response(ResponseBody), /* Reponse */
    Empty,                  /* (original) empty line */
    ParseError,             /* (original) parse error */
}

pub fn parse(source: &dyn ParseSource) -> ShioriProtocol {
    let nl_string = source.next_line();
    let nl = nl_string.as_str();

    if nl.starts_with(consts::RequestMethod::GET.as_str()) {
        ShioriProtocol::Request(request::parse(source, consts::RequestMethod::GET))
    } else if nl.starts_with(consts::RequestMethod::GET.as_str()) {
        ShioriProtocol::Request(request::parse(source, consts::RequestMethod::GET))
    } else if nl.starts_with(consts::SHIORI_REQUEST_HEADER_STARTS) {
        ShioriProtocol::Response(ResponseBody {})
    } else if nl.is_empty() {
        ShioriProtocol::Empty
    } else {
        ShioriProtocol::ParseError
    }
}
