pub struct GstRc {
    _obj: *const (),
    _borrowed: bool,
}

trait GstRcTrait {
    fn get_obj(&self) -> *const ();
    fn is_borrowed(&self) -> bool;
}

impl GstRcTrait for GstRc {
    fn get_obj(&self) -> *const () {
        self._obj
    }

    fn is_borrowed(&self) -> bool {
        self._borrowed
    }
}

const FOO: Option<GstRc> = None;

fn main() {
    let _meh = FOO;
}