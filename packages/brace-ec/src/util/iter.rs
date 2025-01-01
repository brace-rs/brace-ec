pub trait Iterable {
    type Item;

    type Iter<'a>: Iterator<Item = &'a Self::Item>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}

impl<const N: usize, T> Iterable for [T; N] {
    type Item = T;

    type Iter<'a>
        = std::slice::Iter<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.as_slice().iter()
    }
}

impl<T> Iterable for Vec<T> {
    type Item = T;

    type Iter<'a>
        = std::slice::Iter<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        (**self).iter()
    }
}

impl<T> Iterable for Option<T> {
    type Item = T;

    type Iter<'a>
        = std::option::Iter<'a, T>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}
