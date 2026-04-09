trait BufWriter {
    fn push(&mut self, c: u8);
}

impl BufWriter for Vec<u8> {
    fn push(&mut self, c: u8) {
        std::vec::Vec::push(self, c);
    }
}

fn main() {
    let mut buf = Vec::new();
    |c: u8| BufWriter::push(&mut buf, c);
}