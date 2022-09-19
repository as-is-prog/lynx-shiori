use super::{
    consts::{self, GhostExecuteStatus, RequestMethod, SecurityLevel, SenderType},
    parse_interface::ParseSource,
    protocol::{RawShioriHeader, RequestBody},
};

fn str_to_header(str: &str) -> RawShioriHeader {
    let mut spl = str.split(":");
    let header = spl.next().unwrap_or("").trim().to_string();
    let value = spl.next().unwrap_or("").trim_start().to_string();

    RawShioriHeader { header, value }
}

pub(crate) fn parse(source: &dyn ParseSource, method: RequestMethod) -> RequestBody {
    let mut charset = String::new();
    let mut sender = String::new();
    let mut sender_type = SenderType::None;
    let mut security_level = SecurityLevel::None;
    let mut status: Vec<GhostExecuteStatus> = vec![];
    let mut id = String::new();
    let mut base_id: Option<String> = None;
    let mut other_header: Vec<RawShioriHeader> = vec![];
    let mut references: Vec<String> = vec![];

    loop {
        let nl_string = source.next_line();
        let nl = nl_string.as_str();

        if nl.starts_with(consts::SHIORI_PROTOCOL_HEADER_CHARSET_STARTS) {
            charset = str_to_header(nl).value;
        } else if nl.starts_with(consts::SHIORI_PROTOCOL_HEADER_SENDER_STARTS) {
            sender = str_to_header(nl).value;
        } else if nl.starts_with(consts::SHIORI_PROTOCOL_HEADER_SENDER_TYPE_STARTS) {
            // sender_type = str_to_header(nl).value;
        } else if nl.starts_with(consts::SHIORI_PROTOCOL_HEADER_SECULITY_LEVEL_STARTS) {
            // security_level = str_to_header(nl).value;
        } else if nl.starts_with(consts::SHIORI_PROTOCOL_HEADER_STATUS_STARTS) {
            // status = str_to_header(nl).value;
        } else if nl.starts_with(consts::SHIORI_PROTOCOL_HEADER_ID_STARTS) {
            id = str_to_header(nl).value;
        } else if nl.starts_with(consts::SHIORI_PROTOCOL_HEADER_BASE_ID_STARTS) {
            base_id = Some(str_to_header(nl).value);
        } else if nl.starts_with(consts::SHIORI_PROTOCOL_HEADER_REFERENCE_STARTS) {
            let str = str_to_header(nl).value;
            references.push(str);
        } else if nl.is_empty() {
            break;
        } else {
            other_header.push(str_to_header(nl));
        }
    }

    return RequestBody {
        method,
        charset,
        sender,
        sender_type,
        security_level,
        status,
        id,
        base_id,
        other_header,
        references,
    };
}
