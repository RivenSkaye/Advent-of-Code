pub struct FlatGrid {
    inner: Vec<u8>,
    line_length: usize,
}

impl<T> From<&[u8]> for FlatGrid {
    fn from(value: &[u8]) -> Self {
        Self {
            line_length: value.iter().position(|c| b'\n'.eq(c)).unwrap(),
            inner: value.iter().filter(|chr| b'\n'.ne(chr)).collect(),
        }
    }
}
