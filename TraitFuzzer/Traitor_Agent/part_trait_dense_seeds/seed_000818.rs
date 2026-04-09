trait DerefLike { fn deref(&self) -> &str; }
impl DerefLike for String { fn deref(&self) -> &str { self.as_str() } }
fn main() {
  let s = String::from("string");
  let _ = s.deref();
}