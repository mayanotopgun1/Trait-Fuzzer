pub struct Lint {
    pub name: &'static str,
    pub desc: &'static str,
    pub report_in_external_macro: bool,
    pub is_externally_loaded: bool,
    pub crate_level_only: bool,
}

trait LintAccess {
    fn get_name(&self) -> &str;
    fn get_desc(&self) -> &str;
    fn reports_in_external_macro(&self) -> bool;
    fn is_externally_loaded(&self) -> bool;
    fn is_crate_level_only(&self) -> bool;
}

impl LintAccess for Lint {
    fn get_name(&self) -> &str { self.name }
    fn get_desc(&self) -> &str { self.desc }
    fn reports_in_external_macro(&self) -> bool { self.report_in_external_macro }
    fn is_externally_loaded(&self) -> bool { self.is_externally_loaded }
    fn is_crate_level_only(&self) -> bool { self.crate_level_only }
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