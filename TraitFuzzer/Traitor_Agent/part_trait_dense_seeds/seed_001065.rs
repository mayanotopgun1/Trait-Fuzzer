use std::marker::PhantomData;

trait Encoder {
    type Error;
}

trait Encodable<S: Encoder> {
    fn encode(&self, s: &mut S) -> Result<(), S::Error>;
}

impl<S: Encoder> Encodable<S> for i32 {
    fn encode(&self, _s: &mut S) -> Result<(), S::Error> {
        Ok(())
    }
}

struct JsonEncoder<'a>(PhantomData<&'a mut ()>);

impl Encoder for JsonEncoder<'_> {
    type Error = ();
}

trait EncodableExt<S: Encoder>: Encodable<S> {
    fn encode_with_context(&self, s: &mut S, _context: &str) -> Result<(), S::Error>
    where
        Self: Sized,
    {
        self.encode(s)
    }
}

impl<T, S> EncodableExt<S> for T
where
    T: Encodable<S>,
    S: Encoder,
{
}

fn encode_json<'r, T>(object: &T) -> Result<String, ()>
where
    T: for<'a> Encodable<JsonEncoder<'a>>,
{
    let s = String::new();
    {
        let mut encoder = JsonEncoder(PhantomData);
        object.encode(&mut encoder)?;
    }
    Ok(s)
}

struct Foo<T: for<'a> Encodable<JsonEncoder<'a>>>(T);

impl<T: for<'a> Encodable<JsonEncoder<'a>>> Drop for Foo<T> {
    fn drop(&mut self) {
        let _ = encode_json(&self.0);
    }
}

fn main() {
    let _ = Foo(10);
}