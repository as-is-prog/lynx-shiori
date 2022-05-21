use crate::shiori;

use self::ghost::Ghost;

pub mod ghost;

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

        self.ghost.request("aaaa", references)
    }

    pub fn unload(&self) {}
}
