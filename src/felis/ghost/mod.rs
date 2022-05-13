pub trait Ghost {
    fn request(&self, event_id: &str, references: Vec<&str>) -> String;
}
