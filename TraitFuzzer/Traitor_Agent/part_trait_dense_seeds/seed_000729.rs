#![feature(return_position_impl_trait_in_trait)]

trait MyTrait { }

impl MyTrait for &i32 {
}

trait MyTraitExt: MyTrait { }
impl<S> MyTraitExt for S where S: MyTrait {}

fn impls_my_trait<T: MyTrait>() -> impl std::fmt::Debug {
    println!("Type implements MyTrait");
}

fn impls_my_trait_val<T: MyTrait>(_: T) -> impl std::fmt::Debug {
    impls_my_trait::<T>()
}

fn random_where_clause()
where for<'a> &'a i32: MyTrait
{
    println!("Random where clause satisfied");
}

fn main() {
    let x = 22;
    let f = &x;

    let result1 = impls_my_trait_val(f);
    println!("{:?}", result1);

    let result2 = impls_my_trait::<&'static i32>();
    println!("{:?}", result2);

    random_where_clause();
}