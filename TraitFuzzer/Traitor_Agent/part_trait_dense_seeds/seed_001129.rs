#[derive(Clone)]
struct MyVec<T>(Vec<T>);

trait ExtractInner {
    type Inner;
    fn extract_inner(self) -> Self::Inner;
}

impl<T: Clone> ExtractInner for MyVec<T> {
    type Inner = Vec<T>;
    fn extract_inner(self) -> Self::Inner {
        let MyVec(inner_vec) = self;
        inner_vec.clone()
    }
}

trait GetFirstElement {
    type Element;
    fn get_first_element(self) -> Self::Element;
}

impl<T> GetFirstElement for MyVec<T> {
    type Element = T;
    fn get_first_element(self) -> Self::Element {
        let MyVec(inner_vec) = self;
        inner_vec.into_iter().next().unwrap()
    }
}

pub fn main() {
    let my_vec = MyVec(vec![1, 2, 3]);
    let cloned_vec = my_vec.clone();

    let extracted = ExtractInner::extract_inner(cloned_vec);
    assert_eq!(extracted[1], 2);

    assert_eq!(GetFirstElement::get_first_element(my_vec.clone()), 1);

    let MyVec(inner) = my_vec;
    assert_eq!(inner[2], 3);
}