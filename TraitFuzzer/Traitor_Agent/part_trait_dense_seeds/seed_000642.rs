#![allow(non_camel_case_types)]
#![feature(rustc_attrs)]

enum crew_of_enterprise_d {

    #[rustc_dummy]
    jean_luc_picard,

    #[rustc_dummy]
    william_t_riker,

    #[rustc_dummy]
    beverly_crusher,

    #[rustc_dummy]
    deanna_troi,

    #[rustc_dummy]
    data,

    #[rustc_dummy]
    worf,

    #[rustc_dummy]
    geordi_la_forge,
}

trait CrewMission {
    fn boldly_go(&self, where_: String);
}

impl CrewMission for crew_of_enterprise_d {
    fn boldly_go(&self, where_: String) { }
}

fn main() {
    let crew_member = crew_of_enterprise_d::worf;
    crew_member.boldly_go("where no one has gone before".to_string());
}