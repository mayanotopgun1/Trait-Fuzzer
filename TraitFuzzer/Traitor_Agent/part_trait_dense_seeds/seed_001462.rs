#![feature(impl_trait_in_assoc_type)]

trait Expectations {
    fn expect_early_pass_lint(terminate: bool) -> impl core::fmt::Debug;
    fn check_specific_lint() -> impl core::fmt::Debug;
    fn check_multiple_lints_with_lint_group() -> impl core::fmt::Debug;
    fn check_overridden_expectation_lint_level() -> impl core::fmt::Debug;
}

impl Expectations for () {
    #[expect(while_true)]
    fn expect_early_pass_lint(terminate: bool) -> impl core::fmt::Debug {
        while !terminate {
            println!("Do you know what a spin lock is?")
        }
        terminate
    }

    #[expect(unused_variables, reason="<this should fail and display this reason>")]
    fn check_specific_lint() -> impl core::fmt::Debug {
        let _x = 2;
        "unused variable"
    }

    #[expect(unused)]
    fn check_multiple_lints_with_lint_group() -> impl core::fmt::Debug {
        let fox_name = "Sir Nibbles";

        let what_does_the_fox_say = "*ding* *deng* *dung*";

        println!("The fox says: {what_does_the_fox_say}");
        println!("~ {fox_name}");
        (fox_name, what_does_the_fox_say)
    }

    #[expect(unused)]
    fn check_overridden_expectation_lint_level() -> impl core::fmt::Debug {
        #[allow(unused_variables)]
        let this_should_not_fulfill_the_expectation = "maybe";
        this_should_not_fulfill_the_expectation
    }
}

fn main() {
    <()>::check_multiple_lints_with_lint_group();
    <()>::check_overridden_expectation_lint_level();
}