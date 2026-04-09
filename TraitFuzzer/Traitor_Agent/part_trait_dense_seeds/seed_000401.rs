mod b {
    pub mod http {
        pub trait HttpMap {
            fn as_map(&self) -> impl std::fmt::Debug;
        }
        pub struct HeaderMap;
        impl HttpMap for HeaderMap {
            fn as_map(&self) -> impl std::fmt::Debug {
                vec![("Content-Type", "application/json")]
            }
        }
    }

    pub use self::http::*;
    #[derive(Debug)]
    pub struct HeaderMap;
    impl HttpMap for HeaderMap {
        fn as_map(&self) -> impl std::fmt::Debug {
            vec![("User-Agent", "Rust-Traitor")]
        }
    }
}

use crate::b::*;

fn main() {
    let h: crate::b::HeaderMap = HeaderMap;
    println!("{:?}", h.as_map());
}