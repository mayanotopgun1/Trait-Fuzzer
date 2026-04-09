#![feature(generic_associated_types)]

#[cfg_attr(feature = "has_foo", derive)]
trait FooTrait {
    type Assoc<'a> where Self: 'a;
    fn foo(&self) -> Option<Self::Assoc<'static>>;
}

#[cfg(feature = "has_foo")] impl FooTrait for () {
    type Assoc<'a> = &'a u8;
    fn foo(&self) -> Option<Self::Assoc<'static>> { None }
}

#[cfg_attr(feature = "has_foo_yes", derive)]
trait FooTraitYes {
    type Assoc<'a> where Self: 'a;
    fn foo_yes(&self) -> Option<Self::Assoc<'static>>;
}

#[cfg(feature = "has_foo_yes")] impl FooTraitYes for () {
    type Assoc<'a> = &'a u8;
    fn foo_yes(&self) -> Option<Self::Assoc<'static>> { None }
}

#[cfg_attr(feature = "has_bar_yes", derive)]
trait BarTrait {
    type Assoc<'a> where Self: 'a;
    fn has_bar(&self) -> Option<Self::Assoc<'static>>;
}

#[cfg(feature = "has_bar_yes")] impl BarTrait for () {
    type Assoc<'a> = &'a u8;
    fn has_bar(&self) -> Option<Self::Assoc<'static>> { None }
}

// New trait to increase participation
trait ExtendedFooTrait: FooTrait {
    fn foo_or_none(&self) -> Option<Self::Assoc<'static>> {
        self.foo()
    }
}

#[cfg(feature = "has_foo")] impl<T: FooTrait> ExtendedFooTrait for T {}

// New trait to increase participation
trait ExtendedBarTrait: BarTrait {
    fn bar_or_none(&self) -> Option<Self::Assoc<'static>> {
        self.has_bar()
    }
}

#[cfg(feature = "has_bar_yes")] impl<T: BarTrait> ExtendedBarTrait for T {}

fn main() {}