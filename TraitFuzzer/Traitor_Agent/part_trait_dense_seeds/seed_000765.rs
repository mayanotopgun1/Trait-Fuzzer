#![feature(associated_type_defaults)]

fn main() {}

trait FilterFoldExt<T, Acc>: FilterFold<T, Acc> {
    fn filter_fold_with_identity(&self, predicate: Self::Predicate, fold: Self::Fold) -> impl FnMut(Acc, T) -> Acc {
        Self::filter_fold(predicate, fold)
    }
}

impl<T, Acc, S> FilterFoldExt<T, Acc> for S where S: FilterFold<T, Acc> {}

trait FilterFold<T, Acc> {
    type Predicate = Box<dyn FnMut(&T) -> bool>;
    type Fold = Box<dyn FnMut(Acc, T) -> Acc>;

    fn filter_fold(predicate: Self::Predicate, fold: Self::Fold) -> impl FnMut(Acc, T) -> Acc;
}

impl<T, Acc> FilterFold<T, Acc> for () {
    type Predicate = Box<dyn FnMut(&T) -> bool>;
    type Fold = Box<dyn FnMut(Acc, T) -> Acc>;

    fn filter_fold(mut predicate: Self::Predicate, mut fold: Self::Fold) -> impl FnMut(Acc, T) -> Acc {
        move |acc, item| if predicate(&item) { fold(acc, item) } else { acc }
    }
}