use std::marker::PhantomData;

use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;
use thiserror::Error;

use crate::core::individual::Individual;
use crate::core::population::Population;

use super::Selector;

pub struct Windows<S, P>
where
    P: ?Sized,
{
    selector: S,
    size: usize,
    marker: PhantomData<fn() -> P>,
}

impl<S, P> Windows<S, P>
where
    P: ?Sized,
{
    pub fn new(selector: S, size: usize) -> Self {
        Self {
            selector,
            size,
            marker: PhantomData,
        }
    }
}

impl<P, S, T> Selector<P> for Windows<S, P>
where
    P: Population<Individual = T> + AsRef<[T]> + ?Sized,
    S: Selector<[T], Output: IntoIterator<Item = T>>,
    T: Individual,
{
    type Output = Vec<T>;
    type Error = WindowsError<S::Error>;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        if self.size == 0 {
            return Err(WindowsError::Empty);
        }

        if population.len() < self.size {
            return Err(WindowsError::TooLarge);
        }

        population
            .as_ref()
            .windows(self.size)
            .map(|window| self.selector.select(window, rng))
            .flatten_ok()
            .collect::<Result<Vec<_>, _>>()
            .map_err(WindowsError::Select)
    }
}

pub struct ParWindows<S, P>
where
    P: ?Sized,
{
    selector: S,
    size: usize,
    marker: PhantomData<fn() -> P>,
}

impl<S, P> ParWindows<S, P>
where
    P: ?Sized,
{
    pub fn new(selector: S, size: usize) -> Self {
        Self {
            selector,
            size,
            marker: PhantomData,
        }
    }
}

impl<P, S, T> Selector<P> for ParWindows<S, P>
where
    P: Population<Individual = T> + AsRef<[T]> + ?Sized,
    S: Selector<[T], Output: IntoIterator<Item = T> + Send, Error: Send> + Sync,
    T: Individual + Send + Sync,
{
    type Output = Vec<T>;
    type Error = WindowsError<S::Error>;

    fn select<Rng>(&self, population: &P, _: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        if self.size == 0 {
            return Err(WindowsError::Empty);
        }

        if population.len() < self.size {
            return Err(WindowsError::TooLarge);
        }

        population
            .as_ref()
            .par_windows(self.size)
            .map_init(rand::thread_rng, |rng, window| {
                self.selector.select(window, rng)
            })
            .flat_map_iter(|result| std::iter::once(result).flatten_ok())
            .collect::<Result<Vec<_>, _>>()
            .map_err(WindowsError::Select)
    }
}

pub struct ArrayWindows<const N: usize, S, P>
where
    P: ?Sized,
{
    selector: S,
    marker: PhantomData<fn() -> P>,
}

impl<const N: usize, S, P> ArrayWindows<N, S, P>
where
    P: ?Sized,
{
    pub fn new(selector: S) -> Self {
        Self {
            selector,
            marker: PhantomData,
        }
    }
}

impl<const N: usize, P, S, T> Selector<P> for ArrayWindows<N, S, P>
where
    P: Population<Individual = T> + AsRef<[T]> + ?Sized,
    S: Selector<[T; N], Output: IntoIterator<Item = T>>,
    T: Individual,
{
    type Output = Vec<T>;
    type Error = WindowsError<S::Error>;

    fn select<Rng>(&self, population: &P, rng: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        if N == 0 {
            return Err(WindowsError::Empty);
        }

        if population.len() < N {
            return Err(WindowsError::TooLarge);
        }

        population
            .as_ref()
            .windows(N)
            .map(|window| {
                self.selector
                    .select(window.try_into().expect("window"), rng)
            })
            .flatten_ok()
            .collect::<Result<Vec<_>, _>>()
            .map_err(WindowsError::Select)
    }
}

pub struct ParArrayWindows<const N: usize, S, P>
where
    P: ?Sized,
{
    selector: S,
    marker: PhantomData<fn() -> P>,
}

impl<const N: usize, S, P> ParArrayWindows<N, S, P>
where
    P: ?Sized,
{
    pub fn new(selector: S) -> Self {
        Self {
            selector,
            marker: PhantomData,
        }
    }
}

impl<const N: usize, P, S, T> Selector<P> for ParArrayWindows<N, S, P>
where
    P: Population<Individual = T> + AsRef<[T]> + ?Sized,
    S: Selector<[T; N], Output: IntoIterator<Item = T> + Send, Error: Send> + Sync,
    T: Individual + Send + Sync,
{
    type Output = Vec<T>;
    type Error = WindowsError<S::Error>;

    fn select<Rng>(&self, population: &P, _: &mut Rng) -> Result<Self::Output, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        if N == 0 {
            return Err(WindowsError::Empty);
        }

        if population.len() < N {
            return Err(WindowsError::TooLarge);
        }

        population
            .as_ref()
            .par_windows(N)
            .map_init(rand::thread_rng, |rng, window| {
                self.selector
                    .select(window.try_into().expect("window"), rng)
            })
            .flat_map_iter(|result| std::iter::once(result).flatten_ok())
            .collect::<Result<Vec<_>, _>>()
            .map_err(WindowsError::Select)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WindowsError<S> {
    #[error(transparent)]
    Select(S),
    #[error("window size is greater than population size")]
    TooLarge,
    #[error("window is empty")]
    Empty,
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::core::operator::mutator::add::Add;
    use crate::core::operator::recombinator::sum::Sum;
    use crate::core::operator::selector::best::Best;
    use crate::core::operator::selector::worst::Worst;
    use crate::core::operator::selector::Selector;
    use crate::core::population::Population;

    use super::{ArrayWindows, ParArrayWindows, ParWindows, Windows, WindowsError};

    #[test]
    fn test_select_windows() {
        let population = [1, 2, 3, 4, 5];

        let a = population
            .select(Windows::new(Best, 2).mutate(Add(1)))
            .unwrap();
        let b = population
            .select(Windows::new(Best, 3).mutate(Add(1)))
            .unwrap();
        let c = population
            .select(Windows::new(Best, 4).mutate(Add(1)))
            .unwrap();
        let d = population
            .select(Windows::new(Best, 5).mutate(Add(1)))
            .unwrap();
        let e = population
            .select(Windows::new(Worst, 2).mutate(Add(1)))
            .unwrap();
        let f = population
            .select(Windows::new(Best.and(Worst), 2).mutate(Add(1)))
            .unwrap();
        let g = population
            .select(Windows::new(Best.and(Worst), 4).mutate(Add(1)))
            .unwrap();
        let h = population
            .select(Windows::new(Best.and(Worst).recombine(Sum), 4).mutate(Add(1)))
            .unwrap();
        let i = population.select(Windows::new(Best, 0));
        let j = population.select(Windows::new(Best, 6));

        assert_eq!(a, [3, 4, 5, 6]);
        assert_eq!(b, [4, 5, 6]);
        assert_eq!(c, [5, 6]);
        assert_eq!(d, [6]);
        assert_eq!(e, [2, 3, 4, 5]);
        assert_eq!(f, [3, 2, 4, 3, 5, 4, 6, 5]);
        assert_eq!(g, [5, 2, 6, 3]);
        assert_eq!(h, [6, 8]);
        assert_eq!(i, Err(WindowsError::Empty));
        assert_eq!(j, Err(WindowsError::TooLarge));
    }

    #[test]
    fn test_populations_windows() {
        let a = [1, 2, 3, 4].select(Best.windows(1)).unwrap();
        let b = vec![1, 2, 3, 4].select(Best.windows(1)).unwrap();
        let c = [1, 2, 3, 4].as_slice().select(Best.windows(1)).unwrap();

        assert_eq!(a, [1, 2, 3, 4]);
        assert_eq!(b, [1, 2, 3, 4]);
        assert_eq!(c, [1, 2, 3, 4]);
    }

    #[test]
    fn test_select_par_windows() {
        let population = [1, 2, 3, 4, 5];

        let a = population
            .select(ParWindows::new(Best, 2).mutate(Add(1)))
            .unwrap();
        let b = population
            .select(ParWindows::new(Best, 3).mutate(Add(1)))
            .unwrap();
        let c = population
            .select(ParWindows::new(Best, 4).mutate(Add(1)))
            .unwrap();
        let d = population
            .select(ParWindows::new(Best, 5).mutate(Add(1)))
            .unwrap();
        let e = population
            .select(ParWindows::new(Worst, 2).mutate(Add(1)))
            .unwrap();
        let f = population
            .select(ParWindows::new(Best.and(Worst), 2).mutate(Add(1)))
            .unwrap();
        let g = population
            .select(ParWindows::new(Best.and(Worst), 4).mutate(Add(1)))
            .unwrap();
        let h = population
            .select(ParWindows::new(Best.and(Worst).recombine(Sum), 4).mutate(Add(1)))
            .unwrap();
        let i = population.select(ParWindows::new(Best, 0));
        let j = population.select(ParWindows::new(Best, 6));

        assert_eq!(a, [3, 4, 5, 6]);
        assert_eq!(b, [4, 5, 6]);
        assert_eq!(c, [5, 6]);
        assert_eq!(d, [6]);
        assert_eq!(e, [2, 3, 4, 5]);
        assert_eq!(f, [3, 2, 4, 3, 5, 4, 6, 5]);
        assert_eq!(g, [5, 2, 6, 3]);
        assert_eq!(h, [6, 8]);
        assert_eq!(i, Err(WindowsError::Empty));
        assert_eq!(j, Err(WindowsError::TooLarge));
    }

    #[test]
    fn test_populations_par_windows() {
        let a = [1, 2, 3, 4].select(Best.par_windows(1)).unwrap();
        let b = vec![1, 2, 3, 4].select(Best.par_windows(1)).unwrap();
        let c = [1, 2, 3, 4].as_slice().select(Best.par_windows(1)).unwrap();

        assert_eq!(a, [1, 2, 3, 4]);
        assert_eq!(b, [1, 2, 3, 4]);
        assert_eq!(c, [1, 2, 3, 4]);
    }

    #[test]
    fn test_select_array_windows() {
        let population = [1, 2, 3, 4, 5];

        let a = population
            .select(ArrayWindows::<2, _, _>::new(Best).mutate(Add(1)))
            .unwrap();
        let b = population
            .select(ArrayWindows::<3, _, _>::new(Best).mutate(Add(1)))
            .unwrap();
        let c = population
            .select(ArrayWindows::<4, _, _>::new(Best).mutate(Add(1)))
            .unwrap();
        let d = population
            .select(ArrayWindows::<5, _, _>::new(Best).mutate(Add(1)))
            .unwrap();
        let e = population
            .select(ArrayWindows::<2, _, _>::new(Worst).mutate(Add(1)))
            .unwrap();
        let f = population
            .select(ArrayWindows::<2, _, _>::new(Best.and(Worst)).mutate(Add(1)))
            .unwrap();
        let g = population
            .select(ArrayWindows::new(Best::<[_; 4]>.and(Worst)).mutate(Add(1)))
            .unwrap();
        let h = population
            .select(ArrayWindows::<4, _, _>::new(Best.and(Worst).recombine(Sum)).mutate(Add(1)))
            .unwrap();
        let i = population.select(ArrayWindows::<0, _, _>::new(Best));
        let j = population.select(ArrayWindows::<6, _, _>::new(Best));

        assert_eq!(a, [3, 4, 5, 6]);
        assert_eq!(b, [4, 5, 6]);
        assert_eq!(c, [5, 6]);
        assert_eq!(d, [6]);
        assert_eq!(e, [2, 3, 4, 5]);
        assert_eq!(f, [3, 2, 4, 3, 5, 4, 6, 5]);
        assert_eq!(g, [5, 2, 6, 3]);
        assert_eq!(h, [6, 8]);
        assert_eq!(i, Err(WindowsError::Empty));
        assert_eq!(j, Err(WindowsError::TooLarge));
    }

    #[test]
    fn test_populations_array_windows() {
        let a = [1, 2, 3, 4].select(Best.array_windows::<1, _>()).unwrap();
        let b = vec![1, 2, 3, 4]
            .select(Best.array_windows::<1, _>())
            .unwrap();
        let c = [1, 2, 3, 4]
            .as_slice()
            .select(Best.array_windows::<1, _>())
            .unwrap();

        assert_eq!(a, [1, 2, 3, 4]);
        assert_eq!(b, [1, 2, 3, 4]);
        assert_eq!(c, [1, 2, 3, 4]);
    }

    struct Both;

    impl Selector<[i32; 2]> for Both {
        type Output = [i32; 2];
        type Error = Infallible;

        fn select<Rng>(&self, &[a, b]: &[i32; 2], _: &mut Rng) -> Result<Self::Output, Self::Error>
        where
            Rng: rand::Rng + ?Sized,
        {
            Ok([a, b])
        }
    }

    #[test]
    fn test_inference_array_windows() {
        let population = [1, 2, 3, 4, 5];

        let a = population
            .select(Both.array_windows().recombine(Sum))
            .unwrap();
        let b = population
            .select(ArrayWindows::new(Both).recombine(Sum))
            .unwrap();
        let c = population
            .select(ArrayWindows::new(Both.recombine(Sum)))
            .unwrap();
        let d = population
            .select(ArrayWindows::new(Best::<[_; 3]>))
            .unwrap();

        assert_eq!(a, [24]);
        assert_eq!(b, [24]);
        assert_eq!(c, [3, 5, 7, 9]);
        assert_eq!(d, [3, 4, 5]);
    }

    #[test]
    fn test_select_par_array_windows() {
        let population = [1, 2, 3, 4, 5];

        let a = population
            .select(ParArrayWindows::<2, _, _>::new(Best).mutate(Add(1)))
            .unwrap();
        let b = population
            .select(ParArrayWindows::<3, _, _>::new(Best).mutate(Add(1)))
            .unwrap();
        let c = population
            .select(ParArrayWindows::<4, _, _>::new(Best).mutate(Add(1)))
            .unwrap();
        let d = population
            .select(ParArrayWindows::<5, _, _>::new(Best).mutate(Add(1)))
            .unwrap();
        let e = population
            .select(ParArrayWindows::<2, _, _>::new(Worst).mutate(Add(1)))
            .unwrap();
        let f = population
            .select(ParArrayWindows::<2, _, _>::new(Best.and(Worst)).mutate(Add(1)))
            .unwrap();
        let g = population
            .select(ParArrayWindows::new(Best::<[_; 4]>.and(Worst)).mutate(Add(1)))
            .unwrap();
        let h = population
            .select(ParArrayWindows::<4, _, _>::new(Best.and(Worst).recombine(Sum)).mutate(Add(1)))
            .unwrap();
        let i = population.select(ParArrayWindows::<0, _, _>::new(Best));
        let j = population.select(ParArrayWindows::<6, _, _>::new(Best));

        assert_eq!(a, [3, 4, 5, 6]);
        assert_eq!(b, [4, 5, 6]);
        assert_eq!(c, [5, 6]);
        assert_eq!(d, [6]);
        assert_eq!(e, [2, 3, 4, 5]);
        assert_eq!(f, [3, 2, 4, 3, 5, 4, 6, 5]);
        assert_eq!(g, [5, 2, 6, 3]);
        assert_eq!(h, [6, 8]);
        assert_eq!(i, Err(WindowsError::Empty));
        assert_eq!(j, Err(WindowsError::TooLarge));
    }

    #[test]
    fn test_populations_par_array_windows() {
        let a = [1, 2, 3, 4]
            .select(Best.par_array_windows::<1, _>())
            .unwrap();
        let b = vec![1, 2, 3, 4]
            .select(Best.par_array_windows::<1, _>())
            .unwrap();
        let c = [1, 2, 3, 4]
            .as_slice()
            .select(Best.par_array_windows::<1, _>())
            .unwrap();

        assert_eq!(a, [1, 2, 3, 4]);
        assert_eq!(b, [1, 2, 3, 4]);
        assert_eq!(c, [1, 2, 3, 4]);
    }

    #[test]
    fn test_inference_par_array_windows() {
        let population = [1, 2, 3, 4, 5];

        let a = population
            .select(Both.par_array_windows().recombine(Sum))
            .unwrap();
        let b = population
            .select(ParArrayWindows::new(Both).recombine(Sum))
            .unwrap();
        let c = population
            .select(ParArrayWindows::new(Both.recombine(Sum)))
            .unwrap();
        let d = population
            .select(ParArrayWindows::new(Best::<[_; 3]>))
            .unwrap();

        assert_eq!(a, [24]);
        assert_eq!(b, [24]);
        assert_eq!(c, [3, 5, 7, 9]);
        assert_eq!(d, [3, 4, 5]);
    }
}
