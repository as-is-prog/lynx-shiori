pub trait ParseSource {
    /** get next line str.(not include cr/lf) */
    fn next_line(&self) -> String;
}
