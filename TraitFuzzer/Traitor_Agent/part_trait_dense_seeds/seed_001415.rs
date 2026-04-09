#![allow(unused_variables)]
#![allow(non_snake_case)]
#![feature(specialization)]

fn main() {
    nom_sql::selection(b"x ");
}

pub enum Err<P> {
    Position(P),
    NodePosition(u32),
}

pub enum IResult<I, O> {
    Done(I, O),
    Error(Err<I>),
    Incomplete(u32, u64),
}

trait Multispace {
    fn multispace(self) -> crate::IResult<i8, i8>;
}

default impl<T: Copy> Multispace for T {
    default fn multispace(self) -> crate::IResult<i8, i8> {
        crate::IResult::Done(0, 0)
    }
}

impl Multispace for u8 {
    fn multispace(self) -> crate::IResult<i8, i8> {
        crate::IResult::Done(1, 1)
    }
}

impl<'a> Multispace for &'a [u8] {
    default fn multispace(self) -> crate::IResult<i8, i8> {
        self.get(0).copied().unwrap_or(0).multispace()
    }
}

mod nom_sql {
    use super::Multispace;

    fn where_clause(i: &[u8]) -> crate::IResult<&[u8], Option<String>> {
        let X = match i.multispace() {
            crate::IResult::Done(..) => crate::IResult::Done(i, None::<String>),
            _ => crate::IResult::Error(crate::Err::NodePosition(0)),
        };
        match X {
            crate::IResult::Done(_, _) => crate::IResult::Done(i, None),
            _ => X,
        }
    }

    pub fn selection(i: &[u8]) {
        let Y = match {
            match {
                where_clause(i)
            } {
                crate::IResult::Done(_, o) => crate::IResult::Done(i, Some(o)),
                crate::IResult::Error(_) => crate::IResult::Done(i, None),
                _ => crate::IResult::Incomplete(0, 0),
            }
        } {
            crate::IResult::Done(z, _) => crate::IResult::Done(z, None::<String>),
            _ => return (),
        };
        match Y {
            crate::IResult::Done(x, _) => {
                let bytes = b";   ";
                let len = x.len();
                if len < bytes.len() {
                    println!("{}", bytes[len]);
                }
            }
            _ => (),
        }
    }
}