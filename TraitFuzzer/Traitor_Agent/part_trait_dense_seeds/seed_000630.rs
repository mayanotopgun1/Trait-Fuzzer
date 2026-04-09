trait Proj<'a> {
    type Assoc;
}

impl<'a, 'b, F: FnOnce() -> &'b ()> Proj<'a> for F {
    type Assoc = ();
}

trait UseTrait {
    fn use_proj<F: for<'a> Proj<'a>>(f: F);
}

impl<T> UseTrait for T {
    fn use_proj<F: for<'a> Proj<'a>>(_f: F) {}
}

fn is_proj<F: for<'a> Proj<'a>>(f: F) {}

fn define() -> impl Sized + UseTrait {








    is_proj(define);
    &()
}

fn main() {}