use crate::shiori::{self, protocol::ShioriProtocol};
use shiori::parse_interface::ParseSource;

pub enum Protocol {
    Load { load_dir: String },
    Sync { sync_str: String },
    Request(shiori::protocol::RequestBody),
    Unload,
    Empty, /* Empty line */
    ParseError { reason: String },
}

pub fn parse(source: &dyn ParseSource) -> Protocol {
    let nl_string = source.next_line();
    let nl_header = &nl_string[0..3];

    let get_nl_body = || (&nl_string[(nl_header.len())..(nl_string.len())]);

    return match nl_header {
        "*L:" => Protocol::Load {
            load_dir: get_nl_body().to_string(),
        },
        "*S:" => Protocol::Sync {
            sync_str: get_nl_body().to_string(),
        },
        "*U:" => Protocol::Unload,
        &_ => shiori_request_parse(source),
    };
}

fn shiori_request_parse(source: &dyn ParseSource) -> Protocol {
    let result = shiori::protocol::parse(source);

    match result {
        ShioriProtocol::Request(body) => Protocol::Request(body),
        ShioriProtocol::Response(_) => Protocol::ParseError {
            reason: "invalid value: Response".to_string(),
        },
        ShioriProtocol::ParseError => Protocol::ParseError {
            reason: "parse error".to_string(),
        },
        ShioriProtocol::Empty => todo!(),
    }
}
