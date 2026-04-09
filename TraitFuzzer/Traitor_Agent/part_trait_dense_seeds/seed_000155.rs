trait Future {
    type Item;
}

impl<F, T> Future for F
where F: Fn() -> T
{
    type Item = T;
}

trait Connect {}

struct Connector<H> {
    handler: H,
}

impl<H, T> Connect for Connector<H>
where
    T: 'static,
    H: Future<Item = T>
{
}

trait ClientBuilder<C> {
    fn build(connector: C) -> Self;
}

impl<C> ClientBuilder<C> for Client<C> {
    fn build(connector: C) -> Self {
        unimplemented!()
    }
}

struct Client<C> {
    connector: C,
}

fn client<H>(handler: H) -> Client<impl Connect>
where H: Fn() + Copy
{
    let connector = Connector {
        handler,
    };
    let client = Client::<_>::build(connector);
    client
}

fn main() { }