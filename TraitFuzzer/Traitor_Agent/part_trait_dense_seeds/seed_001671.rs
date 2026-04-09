pub trait MyTrait {
    fn dummy(&self) {}
}
impl MyTrait for isize {}


trait MyTraitExt: MyTrait {
    fn extended_dummy(&self) where Self: Sized { self.dummy() }
}
impl<T: MyTrait> MyTraitExt for T {}


pub struct EnumRefDynTrait<'a>(Enum<&'a (dyn MyTraitExt + 'a)>);
pub enum Enum<T> {
    Variant(T),
}

fn enum_dyn_trait() {
    let x: isize = 42;
    let y = EnumRefDynTrait(Enum::Variant(&x as &dyn MyTraitExt));
    let _ = y;
}


struct RefDynTraitNamed<'a> {
    x: Option<&'a (dyn MyTraitExt + 'a)>,
}

fn named_option_dyn_trait() {
    let x: isize = 42;
    let y = RefDynTraitNamed { x: Some(&x as &dyn MyTraitExt) };
    let _ = y;
}


pub struct RefDynTraitUnnamed<'a>(Option<&'a (dyn MyTraitExt + 'a)>);

fn unnamed_option_dyn_trait() {
    let x: isize = 42;
    let y = RefDynTraitUnnamed(Some(&x as &dyn MyTraitExt));
    let _ = y;
}

pub fn main() {
    enum_dyn_trait();
    named_option_dyn_trait();
    unnamed_option_dyn_trait();
}