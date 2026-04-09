struct S;

enum E {
    V,
}

type A = E;

trait Assign {
    fn assign(&mut self);
}

trait TupleAssign {
    fn tuple_assign(&mut self, other: ());
}

impl Assign for S {
    fn assign(&mut self) {
        *self = S;
    }
}

impl TupleAssign for (S, ()) {
    fn tuple_assign(&mut self, other: ()) {
        *self = (S, other);
    }
}

impl Assign for E {
    fn assign(&mut self) {
        *self = E::V;
    }
}

impl TupleAssign for (E, ()) {
    fn tuple_assign(&mut self, other: ()) {
        *self = (E::V, other);
    }
}

trait SelfAssign {
    fn self_assign();
}

impl SelfAssign for S {
    fn self_assign() {
        let mut s = S;
        s.assign();
    }
}

impl SelfAssign for E {
    fn self_assign() {
        let mut e = E::V;
        e.assign();
    }
}

fn main() {
    let mut s = S;
    s.assign();
    (s, ()).tuple_assign(());

    let mut e = E::V;
    e.assign();
    (e, ()).tuple_assign(());

    S::self_assign();
    E::self_assign();

    let mut a = A::V;
    (a, ()).tuple_assign(());
}