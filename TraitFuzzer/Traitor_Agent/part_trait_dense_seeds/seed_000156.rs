#![feature(impl_trait_in_assoc_type)]

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

fn client<H>(handler: H) -> impl Connect
where H: Fn() + Copy
{
    let connector = Connector {
        handler,
    };
    Client::build(connector)
}

impl<H, T> Connect for Client<Connector<H>>
where
    T: 'static,
    H: Future<Item = T>
{
}

fn main() { }