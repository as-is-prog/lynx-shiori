use super::{parse_interface::ParseSource, protocol::ResponseBody};

pub(crate) fn parse(source: &dyn ParseSource) -> ResponseBody {
    let nl_string = source.next_line();
    let nl = nl_string.as_str();

    return ResponseBody {
        charset: todo!(),
        sender: todo!(),
        value: todo!(),
        value_notify: todo!(),
        security_level: todo!(),
        marker: todo!(),
        error_level: todo!(),
        error_description: todo!(),
        other_header: todo!(),
    };
}
