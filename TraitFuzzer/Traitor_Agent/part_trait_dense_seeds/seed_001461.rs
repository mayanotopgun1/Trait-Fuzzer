trait Expectations {
    fn expect_early_pass_lint(terminate: bool);
    fn check_specific_lint();
    fn check_multiple_lints_with_lint_group();
    fn check_overridden_expectation_lint_level();
}

impl Expectations for () {
    #[expect(while_true)]
    fn expect_early_pass_lint(terminate: bool) {
        while !terminate {
            println!("Do you know what a spin lock is?")
        }
    }

    #[expect(unused_variables, reason="<this should fail and display this reason>")]
    fn check_specific_lint() {
        let _x = 2;
    }

    #[expect(unused)]
    fn check_multiple_lints_with_lint_group() {
        let fox_name = "Sir Nibbles";

        let what_does_the_fox_say = "*ding* *deng* *dung*";

        println!("The fox says: {what_does_the_fox_say}");
        println!("~ {fox_name}");
    }

    #[expect(unused)]
    fn check_overridden_expectation_lint_level() {
        #[allow(unused_variables)]
        let this_should_not_fulfill_the_expectation = "maybe";
    }
}

fn main() {
    <()>::check_multiple_lints_with_lint_group();
    <()>::check_overridden_expectation_lint_level();
}