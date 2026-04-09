#![crate_type = "lib"]
#![feature(impl_trait_in_assoc_type)]

pub trait Archive {
    type Archived;
    type Resolver;

    fn resolve(resolver: Self::Resolver, out: *mut Self::Archived);
}

pub type Archived<T> = <T as Archive>::Archived;
pub type Resolver<T> = <T as Archive>::Resolver;

pub struct Record<'a> {
    _payload: &'a [u8],
}

pub struct ArchivedRecord<'a>
where
    &'a [u8]: Archive,
{
    _payload: Archived<&'a [u8]>,
}

pub struct RecordResolver<'a>
where
    &'a [u8]: Archive,
{
    _payload: Resolver<&'a [u8]>,
}

impl<'a> Archive for Record<'a>
where
    &'a [u8]: Archive,
{
    type Archived = ArchivedRecord<'a>;
    type Resolver = RecordResolver<'a>;

    fn resolve(_resolver: Self::Resolver, _out: *mut Self::Archived) {}
}

trait ExtendedArchive {
    fn extended_resolve(resolver: Resolver<Self>, out: *mut Archived<Self>) where Self: Archive;
}

impl<'a> ExtendedArchive for Record<'a>
where
    &'a [u8]: Archive,
{
    fn extended_resolve(resolver: Resolver<Self>, out: *mut Archived<Self>) {
        Self::resolve(resolver, out);
    }
}