mod b {
    pub mod http {
        pub trait HttpMap {}
        pub struct HeaderMap;
        impl HttpMap for HeaderMap {}
    }

    pub use self::http::*;
    #[derive(Debug)]
    pub struct HeaderMap;
    impl HttpMap for HeaderMap {}
}

use crate::b::*;

fn main() {
    let h: crate::b::HeaderMap = HeaderMap;
}