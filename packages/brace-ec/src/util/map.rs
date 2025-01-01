pub trait TryMap {
    type Item;

    fn try_map<F, E>(self, f: F) -> Result<Self, E>
    where
        F: FnMut(Self::Item) -> Result<Self::Item, E>,
        Self: Sized;
}

impl<const N: usize, T> TryMap for [T; N] {
    type Item = T;

    fn try_map<F, E>(self, f: F) -> Result<Self, E>
    where
        F: FnMut(Self::Item) -> Result<Self::Item, E>,
    {
        array_util::try_map(self, f)
    }
}

impl<T> TryMap for Vec<T> {
    type Item = T;

    fn try_map<F, E>(self, f: F) -> Result<Self, E>
    where
        F: FnMut(Self::Item) -> Result<Self::Item, E>,
    {
        self.into_iter().map(f).collect()
    }
}

impl<T> TryMap for Option<T> {
    type Item = T;

    fn try_map<F, E>(self, mut f: F) -> Result<Self, E>
    where
        F: FnMut(Self::Item) -> Result<Self::Item, E>,
    {
        match self {
            Some(item) => f(item).map(Some),
            None => Ok(None),
        }
    }
}
