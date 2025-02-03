use std::convert::Infallible;
use std::sync::atomic::{AtomicU64, Ordering};

use atomic_traits::fetch::Add;
use num_traits::One;

use super::Generator;

pub struct Counter<T> {
    atomic: T,
}

impl<T> Counter<T>
where
    T: Add<Type: One>,
{
    pub fn new(atomic: T) -> Self {
        Self { atomic }
    }
}

impl Counter<AtomicU64> {
    pub fn u64() -> Self {
        Self::new(AtomicU64::new(0))
    }
}

impl<T> Generator<T::Type> for Counter<T>
where
    T: Add<Type: One>,
{
    type Error = Infallible;

    fn generate<Rng>(&self, _: &mut Rng) -> Result<T::Type, Self::Error>
    where
        Rng: rand::Rng + ?Sized,
    {
        Ok(self.atomic.fetch_add(T::Type::one(), Ordering::Relaxed))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::operator::generator::Generator;

    use super::Counter;

    #[test]
    fn test_generate() {
        let mut rng = rand::rng();

        let counter = Counter::u64();

        let a = counter.generate(&mut rng).unwrap();
        let b = counter.generate(&mut rng).unwrap();
        let c = counter.generate(&mut rng).unwrap();

        assert_eq!(a, 0);
        assert_eq!(b, 1);
        assert_eq!(c, 2);
    }
}
