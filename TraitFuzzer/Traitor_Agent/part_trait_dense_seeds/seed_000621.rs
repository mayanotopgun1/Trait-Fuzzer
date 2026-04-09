#![feature(contracts_internals, core_intrinsics)]

trait ContractChecker {
    fn check_requires(&self, condition: bool) -> impl Fn();
    fn check_ensures(&self, predicate: Option<fn(&u32) -> bool>, value: u32) -> impl Fn();
}

impl ContractChecker for () {
    fn check_requires(&self, condition: bool) -> impl Fn() {
        move || core::intrinsics::contract_check_requires(|| condition)
    }

    fn check_ensures(&self, predicate: Option<fn(&u32) -> bool>, value: u32) -> impl Fn() {
        move || { 
            let _ = core::intrinsics::contract_check_ensures(predicate, value); 
        }
    }
}

fn main() {
    let checker = ();

    let require_check = checker.check_requires(true);
    require_check();

    #[cfg(chk_fail_requires)]
    {
        let fail_require_check = checker.check_requires(false);
        fail_require_check();
    }

    fn doubles_to_two(ret: &u32) -> bool {
        let old = 2;
        ret + ret == old
    }

    let ensure_check = checker.check_ensures(Some(doubles_to_two), 1);
    ensure_check();

    #[cfg(chk_fail_ensures)]
    {
        let fail_ensure_check = checker.check_ensures(Some(doubles_to_two), 2);
        fail_ensure_check();
    }
}