trait ArrayAccess {
    fn get(&self) -> [(String, String); 3];
}

impl ArrayAccess for () {
    fn get(&self) -> [(String, String); 3] {
        Default::default()
    }
}

trait MoveOutExt: ArrayAccess {
    fn move_out_from_begin_and_one_from_end(&self) {
        let a = self.get();
        let [_, _, _x] = a;
        let [.., ref _y, _] = a;
    }

    fn move_out_from_begin_field_and_end_field(&self) {
        let a = self.get();
        let [_, _, (_x, _)] = a;
        let [.., (_, ref _y)] = a;
    }

    fn move_out_by_const_index_and_subslice(&self) {
        let a = self.get();
        let [_x, _, _] = a;
        let [_, ref _y @ ..] = a;
    }

    fn move_out_by_const_index_end_and_subslice(&self) {
        let a = self.get();
        let [.., _x] = a;
        let [ref _y @ .., _] = a;
    }

    fn move_out_by_const_index_field_and_subslice(&self) {
        let a = self.get();
        let [(_x, _), _, _] = a;
        let [_, ref _y @ ..] = a;
    }

    fn move_out_by_const_index_end_field_and_subslice(&self) {
        let a = self.get();
        let [.., (_x, _)] = a;
        let [ref _y @ .., _] = a;
    }

    fn move_out_by_const_subslice_and_index_field(&self) {
        let a = self.get();
        let [_, _y @ ..] = a;
        let [(ref _x, _), _, _] = a;
    }

    fn move_out_by_const_subslice_and_end_index_field(&self) {
        let a = self.get();
        let [_y @ .., _] = a;
        let [.., (ref _x, _)] = a;
    }

    fn move_out_by_subslice_and_subslice(&self) {
        let a = self.get();
        let [x @ .., _, _] = a;
        let [_, ref _y @ ..] = a;
    }
}

impl MoveOutExt for () {}

fn main() {}