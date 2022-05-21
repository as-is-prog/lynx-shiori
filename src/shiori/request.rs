use super::{consts::RequestMethod, parse_interface::ParseSource, protocol::RequestBody};

pub(crate) fn parse(source: &dyn ParseSource, method: RequestMethod) -> RequestBody {
    let nl_string = source.next_line();
    let nl = nl_string.as_str();

    RequestBody {
        method,
        charset: todo!(),
        sender: todo!(),
        sender_type: todo!(),
        security_level: todo!(),
        status: todo!(),
    }
}
