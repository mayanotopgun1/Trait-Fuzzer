struct StringBuffer {
    s: String,
}

trait Append {
    fn append(&mut self, v: &str);
}

impl Append for StringBuffer {
    fn append(&mut self, v: &str) {
        self.s.push_str(v);
    }
}

fn to_string(sb: StringBuffer) -> String {
    sb.s
}

pub fn main() {
    let mut sb = StringBuffer {
        s: String::new(),
    };
    sb.append("Hello, ");
    sb.append("World!");
    let str = to_string(sb);
    assert_eq!(str, "Hello, World!");
}