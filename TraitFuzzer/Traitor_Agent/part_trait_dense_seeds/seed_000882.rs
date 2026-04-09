trait AsBytes { fn as_bytes(&self) -> &[u8]; }
impl AsBytes for String { fn as_bytes(&self) -> &[u8] { self.as_bytes() } }

fn main() {
    let s: String = "hello".to_string();

    assert_eq!(s.as_bytes()[5], 0x0 as u8);
}