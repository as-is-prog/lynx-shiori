use super::{consts, parse_interface::ParseSource, request, response};

pub struct RawShioriHeader {
    pub header: String,
    pub value: String,
}

pub struct ResponseBody {
    pub charset: String,
    pub sender: String,
    pub value: Option<String>,
    pub value_notify: Option<String>,
    pub security_level: consts::SecurityLevel,
    pub marker: Option<String>,
    pub error_level: Option<Vec<consts::ErrorLevel>>,
    pub error_description: Option<Vec<String>>,
    pub other_header: Option<Vec<RawShioriHeader>>,
}
impl ResponseBody {
    pub fn get_raw_headers(&self) -> Vec<RawShioriHeader> {
        let mut vec: Vec<RawShioriHeader> = vec![];

        // TODO: other以外も入れる
        if let Some(other_vec) = &self.other_header {
            for h in other_vec {
                vec.push(RawShioriHeader {
                    header: h.header.clone(),
                    value: h.value.clone(),
                });
            }
        }

        return vec;
    }
}

pub struct RequestBody {
    pub method: consts::RequestMethod,
    pub charset: String,
    pub sender: String,
    pub sender_type: consts::SenderType,
    pub security_level: consts::SecurityLevel,
    pub status: Vec<consts::GhostExecuteStatus>,
    pub id: String,
    pub base_id: Option<String>,
    pub references: Vec<String>,
    pub other_header: Vec<RawShioriHeader>,
}

pub enum ShioriProtocol {
    Request(RequestBody),   /* Request */
    Response(ResponseBody), /* Reponse */
    Empty,                  /* (original) empty line */
    ParseError,             /* (original) parse error */
}

pub fn parse(first_nl: String, source: &dyn ParseSource) -> ShioriProtocol {
    let nl = first_nl.as_str();

    if nl.starts_with(&consts::RequestMethod::Get.to_string()) {
        ShioriProtocol::Request(request::parse(source, consts::RequestMethod::Get))
    } else if nl.starts_with(&consts::RequestMethod::Notify.to_string()) {
        ShioriProtocol::Request(request::parse(source, consts::RequestMethod::Notify))
    } else if nl.starts_with(consts::SHIORI_PROTOCOL_VERSION_3_0) {
        ShioriProtocol::Response(response::parse(source))
    } else if nl.is_empty() {
        ShioriProtocol::Empty
    } else {
        ShioriProtocol::ParseError
    }
}
