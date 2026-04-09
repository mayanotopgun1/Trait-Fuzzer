trait Pat { const PAT: u8; }

impl Pat for () {
    const PAT: u8 = 1;
}

fn main() {
    match 0 {
        ..=<() as Pat>::PAT => {}
        _ => {}
    }
}