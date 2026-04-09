#![feature(contracts_internals, core_intrinsics)]

trait ContractChecker {
    fn check_requires(&self, condition: bool);
    fn check_ensures(&self, predicate: Option<fn(&u32) -> bool>, value: u32);
}

impl ContractChecker for () {
    fn check_requires(&self, condition: bool) {
        core::intrinsics::contract_check_requires(|| condition);
    }

    fn check_ensures(&self, predicate: Option<fn(&u32) -> bool>, value: u32) {
        core::intrinsics::contract_check_ensures(predicate, value);
    }
}

fn main() {
    let checker = ();

    checker.check_requires(true);

    #[cfg(chk_fail_requires)]
    checker.check_requires(false);

    fn doubles_to_two(ret: &u32) -> bool {
        let old = 2;
        ret + ret == old
    }

    checker.check_ensures(Some(doubles_to_two), 1);

    #[cfg(chk_fail_ensures)]
    checker.check_ensures(Some(doubles_to_two), 2);
}