use crate::shiori::{self, consts, protocol::ResponseBody};

use self::ghost::Ghost;

pub mod ghost;

const FELIS_CHARSET_DEFAULT: &str = "UTF-8";
const FELIS_SENDER_DEFAULT: &str = "Felis";

pub struct Felis<'a> {
    ghost: &'a dyn Ghost,
}

impl Felis<'_> {
    pub fn new(ghost: &dyn Ghost) -> Felis {
        Felis { ghost }
    }

    pub fn load(&self, _load_dir: &str) {}

    pub fn request(&self, request: shiori::protocol::RequestBody) -> String {
        let references = vec!["a", "b", "c"];

        let value = self.ghost.request(&request.id, references);

        let response = ResponseBody {
            charset: FELIS_CHARSET_DEFAULT.to_string(),
            sender: FELIS_SENDER_DEFAULT.to_string(),
            value: Option::Some(value),
            value_notify: None,
            security_level: consts::SecurityLevel::Local,
            marker: None,
            error_level: None,
            error_description: None,
            other_header: None,
        };

        return response.to_string();
    }

    pub fn unload(&self) {}
}
