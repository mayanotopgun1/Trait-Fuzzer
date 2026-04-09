#[cfg(windows)]
trait DoWindowsStuff {
    fn do_windows_stuff(&self);
}

#[cfg(windows)]
impl DoWindowsStuff for () {
    fn do_windows_stuff(&self) {}
}

#[cfg_attr(uu, unix)]
trait DoTest {
    fn do_test(&self);
}

#[cfg_attr(uu, unix)]
impl DoTest for () {
    fn do_test(&self) {}
}

#[cfg(feature = "foo")]
trait UseFoo {
    fn use_foo(&self);
}

#[cfg(feature = "foo")]
impl UseFoo for () {
    fn use_foo(&self) {}
}

#[cfg(feature = "bar")]
trait UseBar {
    fn use_bar(&self);
}

#[cfg(feature = "bar")]
impl UseBar for () {
    fn use_bar(&self) {}
}

#[cfg(feature = "zebra")]
trait UseZebra {
    fn use_zebra(&self);
}

#[cfg(feature = "zebra")]
impl UseZebra for () {
    fn use_zebra(&self) {}
}

fn test_cfg_macro() {
    cfg!(windows);
    cfg!(widnows);

    cfg!(feature = "foo");
    cfg!(feature = "bar");

    cfg!(feature = "zebra");

    cfg!(xxx = "foo");

    cfg!(xxx);

    cfg!(any(xxx, windows));

    cfg!(any(feature = "bad", windows));

    cfg!(any(windows, xxx));

    cfg!(all(unix, xxx));

    cfg!(all(aa, bb));

    cfg!(any(aa, bb));

    cfg!(any(unix, feature = "zebra"));

    cfg!(any(xxx, feature = "zebra"));

    cfg!(any(xxx, unix, xxx));

    cfg!(all(feature = "zebra", feature = "zebra", feature = "zebra"));
}

fn main() {
    #[cfg(windows)]
    let _ = ().do_windows_stuff();

    #[cfg_attr(uu, unix)]
    let _ = ().do_test();

    #[cfg(feature = "foo")]
    let _ = ().use_foo();

    #[cfg(feature = "bar")]
    let _ = ().use_bar();

    #[cfg(feature = "zebra")]
    let _ = ().use_zebra();
}