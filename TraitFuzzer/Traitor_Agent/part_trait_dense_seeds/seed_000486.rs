trait Hello<'b, F>
where
    for<'a> F: Iterator<Item: 'a> + 'b,
{
    fn hello();
}

impl<'b, F> Hello<'b, F> for ()
where
    for<'a> F: Iterator<Item: 'a> + 'b,
{
    fn hello() {}
}

fn main() {
    let _: () = <() as Hello<'_, std::iter::Once<()>>>::hello();
}