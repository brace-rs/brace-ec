use super::individual::Individual;

pub trait Population {
    type Individual: Individual;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T, const N: usize> Population for [T; N]
where
    T: Individual,
{
    type Individual = T;

    fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<T> Population for Vec<T>
where
    T: Individual,
{
    type Individual = T;

    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Population;

    fn erase<P: Population>(population: P) -> impl Population {
        population
    }

    #[test]
    fn test_population_array() {
        let population = erase([[0, 0]]);

        assert!(!population.is_empty());
        assert_eq!(population.len(), 1);

        let population = erase([[0, 0], [1, 1]]);

        assert!(!population.is_empty());
        assert_eq!(population.len(), 2);
    }

    #[test]
    fn test_population_vec() {
        let population = erase(Vec::<[u32; 2]>::new());

        assert!(population.is_empty());
        assert_eq!(population.len(), 0);

        let population = erase(vec![[0, 0]]);

        assert!(!population.is_empty());
        assert_eq!(population.len(), 1);

        let population = erase(vec![[0, 0], [1, 1]]);

        assert!(!population.is_empty());
        assert_eq!(population.len(), 2);
    }
}
