struct It;

struct Data {
    items: Vec<It>,
}

trait DataTrait {
    fn content(&self) -> impl Iterator<Item = &It>;
}

impl DataTrait for Data {
    fn content(&self) -> impl Iterator<Item = &It> {
        self.items.iter()
    }
}

impl Data {
    fn new() -> Self {
        Self {
            items: vec![It, It],
        }
    }
}

struct Container<'a> {
    name: String,
    resolver: Box<dyn Resolver + 'a>,
}

impl<'a> Container<'a> {
    fn new<R: Resolver + 'a>(name: &str, resolver: R) -> Self {
        Self {
            name: name.to_owned(),
            resolver: Box::new(resolver),
        }
    }
}

trait Resolver {}

impl<R: Resolver> Resolver for &R {}

impl Resolver for It {}

trait GetResolver<'a> {
    fn get(self) -> impl Resolver + 'a;
}

impl<'a, I> GetResolver<'a> for I
where
    I: Iterator<Item = &'a It>,
{
    fn get(mut self) -> impl Resolver + 'a {
        self.next().unwrap()
    }
}

fn main() {
    let data = Data::new();
    let resolver = data.content().get();

    let _ = ["a", "b"]
        .iter()
        .map(|&n| Container::new(n, &resolver))
        .map(|c| c.name)
        .collect::<Vec<_>>();

    let resolver = data.content().get();

    let _ = ["a", "b"]
        .iter()
        .map(|&n| Container::new(n, &resolver))
        .map(|c| c.name)
        .collect::<Vec<_>>();
}