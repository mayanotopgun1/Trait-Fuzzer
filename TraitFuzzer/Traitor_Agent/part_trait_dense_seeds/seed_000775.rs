#![feature(type_alias_impl_trait)]

pub struct Lint {
    pub name: &'static str,
    pub desc: &'static str,
    pub report_in_external_macro: bool,
    pub is_externally_loaded: bool,
    pub crate_level_only: bool,
}

trait LintAccess {
    type Accessor;
    fn get_name(&self) -> Self::Accessor;
    fn get_desc(&self) -> Self::Accessor;
    fn reports_in_external_macro(&self) -> Self::Accessor;
    fn is_externally_loaded(&self) -> Self::Accessor;
    fn is_crate_level_only(&self) -> Self::Accessor;
}

impl LintAccess for Lint {
    type Accessor = &'static str;

    fn get_name(&self) -> Self::Accessor { self.name }
    fn get_desc(&self) -> Self::Accessor { self.desc }
    fn reports_in_external_macro(&self) -> Self::Accessor { if self.report_in_external_macro { "true" } else { "false" } }
    fn is_externally_loaded(&self) -> Self::Accessor { if self.is_externally_loaded { "true" } else { "false" } }
    fn is_crate_level_only(&self) -> Self::Accessor { if self.crate_level_only { "true" } else { "false" } }
}

static FOO: &Lint = &Lint {
    name: &"foo",
    desc: "desc",
    report_in_external_macro: false,
    is_externally_loaded: true,
    crate_level_only: false,
};

fn main() {
    println!("Name: {}", FOO.get_name());
    println!("Description: {}", FOO.get_desc());
    println!("Reports in external macro: {}", FOO.reports_in_external_macro());
    println!("Is externally loaded: {}", FOO.is_externally_loaded());
    println!("Is crate level only: {}", FOO.is_crate_level_only());
}