pub mod uniform;

pub trait Generator<T>: Sized {
    type Error;

    fn generate<Rng>(&self, rng: &mut Rng) -> Result<T, Self::Error>
    where
        Rng: rand::Rng + ?Sized;
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::convert::Infallible;

    use super::Generator;

    struct Count(Cell<u8>);

    impl Generator<u8> for Count {
        type Error = Infallible;

        fn generate<Rng>(&self, _: &mut Rng) -> Result<u8, Self::Error>
        where
            Rng: rand::Rng + ?Sized,
        {
            let n = self.0.get() + 1;

            self.0.set(n);

            Ok(n)
        }
    }

    #[test]
    fn test_generate() {
        let mut rng = rand::thread_rng();

        let count = Count(Cell::new(0));

        let a = count.generate(&mut rng).unwrap();
        let b = count.generate(&mut rng).unwrap();
        let c = count.generate(&mut rng).unwrap();

        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 3);
    }
}
