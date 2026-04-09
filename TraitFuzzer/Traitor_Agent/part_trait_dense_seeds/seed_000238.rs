#![allow(dead_code)]

enum TyS {
    Nil,
}

struct RawT {
    struct_: TyS,
    cname: Option<String>,
    hash: usize,
}

trait RawTyMaker {
    fn mk_raw_ty(st: TyS, cname: Option<String>) -> Self;
}

impl RawTyMaker for RawT {
    fn mk_raw_ty(st: TyS, cname: Option<String>) -> Self {
        RawT { struct_: st, cname: cname, hash: 0 }
    }
}

pub fn main() {
    let _ = RawT::mk_raw_ty(TyS::Nil, None::<String>);
}