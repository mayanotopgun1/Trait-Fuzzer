trait LintChecker {
    fn expect_early_pass_lint(&self);
    fn check_specific_lint(&self);
    fn check_multiple_lints_with_lint_group(&self);
    fn check_expect_overrides_allow_lint_level(&self);
}

impl LintChecker for () {
    fn expect_early_pass_lint(&self) {
        #[expect(while_true)]
        while true {
            println!("I never stop");
        }
    }

    fn check_specific_lint(&self) {
        let x = 2;
    }

    fn check_multiple_lints_with_lint_group(&self) {
        let fox_name = "Sir Nibbles";
        let mut what_does_the_fox_say = "*ding* *deng* *dung*";

        println!("The fox says: {what_does_the_fox_say}");
    }

    fn check_expect_overrides_allow_lint_level(&self) {
        #[expect(unused_variables)]
        let this_should_fulfill_the_expectation = "The `#[allow]` has no power here";
    }
}

fn main() {
    let checker: &dyn LintChecker = &();
    checker.expect_early_pass_lint();
    checker.check_specific_lint();
    checker.check_multiple_lints_with_lint_group();
    checker.check_expect_overrides_allow_lint_level();
}