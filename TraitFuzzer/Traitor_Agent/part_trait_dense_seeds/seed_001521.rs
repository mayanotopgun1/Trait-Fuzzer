#![allow(dropping_copy_types)]

#[derive(Copy, Clone)]
struct C;

fn mk_c() -> C { C }

#[derive(Copy, Clone)]
struct P<A, B>(A, B);

enum E<A, B> { L(A), R(B) }

trait CloneTuple {
    fn clone_tuple(&self) -> Self;
}

impl<T1: Clone, T2: Clone> CloneTuple for (T1, T2) {
    fn clone_tuple(&self) -> Self {
        (self.0.clone(), self.1.clone())
    }
}

impl<A: Clone, B: Clone> CloneTuple for P<A, B> {
    fn clone_tuple(&self) -> Self {
        P(self.0.clone(), self.1.clone())
    }
}

fn main() {
    let a @ b @ c @ d = C;
    let a @ (b, c) = (C, mk_c()).clone_tuple();
    let a @ P(b, P(c, d)) = P(mk_c(), P(C, C)).clone_tuple();
    let a @ [b, c] = [C, C];
    let a @ [b, .., c] = [C, mk_c(), C];
    let a @ [b, mid @ .., c] = [C, mk_c(), C];
    let a @ &(b, c) = &(C, C);
    let a @ &(b, &P(c, d)) = &(mk_c(), &P(C, C));

    fn foo(a @ [b, mid @ .., c]: [C; 3]) {}

    use self::E::*;
    match L(C) {
        L(a) | R(a) => {
            let a: C = a;
            drop(a);
            drop(a);
        }
    }
    match R(&L(&mk_c())) {
        L(L(&a)) | L(R(&a)) | R(L(&a)) | R(R(&a)) => {
            let a: C = a;
            drop(a);
            drop(a);
        }
    }

    match Ok(mk_c()) {
        Ok(ref a @ b) | Err(b @ ref a) => {
            let _: &C = a;
            let _: C = b;
        }
    }
}