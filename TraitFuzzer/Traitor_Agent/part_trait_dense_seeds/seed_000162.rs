trait BufWriter {
    fn push(&mut self, c: u8);
}

impl BufWriter for Vec<u8> {
    fn push(&mut self, c: u8) {
        std::vec::Vec::push(self, c);
    }
}

fn write_to_buffer() -> impl BufWriter {
    Vec::new()
}

fn main() {
    let mut buf = write_to_buffer();
    |c: u8| BufWriter::push(&mut buf, c);
}