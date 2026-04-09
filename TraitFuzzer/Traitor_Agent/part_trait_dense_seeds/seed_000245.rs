trait Search {
    fn search(target: (bool, bool, bool)) -> u32;
}

impl Search for () {
    fn search(target: (bool, bool, bool)) -> u32 {
        let x = ((false, true), (false, true), (false, true));
        let mut guard_count = 0;
        match x {
            ((a, _) | (_, a), (b @ _, _) | (_, b @ _), (c @ false, _) | (_, c @ _))
                if {
                    guard_count += 1;
                    (a, b, c) == target
                } =>
            {
                guard_count
            }
            _ => unreachable!(),
        }
    }
}

trait SearchOldStyle {
    fn search_old_style(target: (bool, bool, bool)) -> u32;
}

impl SearchOldStyle for () {
    fn search_old_style(target: (bool, bool, bool)) -> u32 {
        let x = ((false, true), (false, true), (false, true));
        let mut guard_count = 0;
        match x {
            ((a, _), (b @ _, _), (c @ false, _))
            | ((a, _), (b @ _, _), (_, c @ true))
            | ((a, _), (_, b @ _), (c @ false, _))
            | ((a, _), (_, b @ _), (_, c @ true))
            | ((_, a), (b @ _, _), (c @ false, _))
            | ((_, a), (b @ _, _), (_, c @ true))
            | ((_, a), (_, b @ _), (c @ false, _))
            | ((_, a), (_, b @ _), (_, c @ true))
                if {
                    guard_count += 1;
                    (a, b, c) == target
                } =>
            {
                guard_count
            }
            _ => unreachable!(),
        }
    }
}

trait SearchWithDummy {
    fn search_with_dummy(target: (bool, bool)) -> u32;
}

impl SearchWithDummy for () {
    fn search_with_dummy(target: (bool, bool)) -> u32 {
        let x = ((false, true), (false, true), ());
        let mut guard_count = 0;
        match x {
            ((a, _) | (_, a), (b, _) | (_, b), _ | _)
                if {
                    guard_count += 1;
                    (a, b) == target
                } =>
            {
                guard_count
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    assert_eq!(<() as Search>::search((false, false, false)), 1);
    assert_eq!(<() as Search>::search((false, false, true)), 2);
    assert_eq!(<() as Search>::search((false, true, false)), 3);
    assert_eq!(<() as Search>::search((false, true, true)), 4);
    assert_eq!(<() as Search>::search((true, false, false)), 5);
    assert_eq!(<() as Search>::search((true, false, true)), 6);
    assert_eq!(<() as Search>::search((true, true, false)), 7);
    assert_eq!(<() as Search>::search((true, true, true)), 8);

    assert_eq!(<() as SearchOldStyle>::search_old_style((false, false, false)), 1);
    assert_eq!(<() as SearchOldStyle>::search_old_style((false, false, true)), 2);
    assert_eq!(<() as SearchOldStyle>::search_old_style((false, true, false)), 3);
    assert_eq!(<() as SearchOldStyle>::search_old_style((false, true, true)), 4);
    assert_eq!(<() as SearchOldStyle>::search_old_style((true, false, false)), 5);
    assert_eq!(<() as SearchOldStyle>::search_old_style((true, false, true)), 6);
    assert_eq!(<() as SearchOldStyle>::search_old_style((true, true, false)), 7);
    assert_eq!(<() as SearchOldStyle>::search_old_style((true, true, true)), 8);

    assert_eq!(<() as SearchWithDummy>::search_with_dummy((false, false)), 1);
    assert_eq!(<() as SearchWithDummy>::search_with_dummy((false, true)), 3);
    assert_eq!(<() as SearchWithDummy>::search_with_dummy((true, false)), 5);
    assert_eq!(<() as SearchWithDummy>::search_with_dummy((true, true)), 7);
}